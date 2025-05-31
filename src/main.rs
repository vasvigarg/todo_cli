mod manager;
mod task;

use clap::{Parser, Subcommand};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone}; // Added NaiveDate
use manager::TaskManager;

/// A simple command-line ToDo app written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Defines the subcommands for the ToDo CLI.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task
    Add {
        /// Description of the task
        description: String,
        /// Optional due date for the task in "YYYY-MM-DD HH:MM" or "YYYY-MM-DD" format (local time, converted to IST)
        #[arg(long)]
        due: Option<String>,
    },
    /// List all tasks
    List,
    /// Mark a task as done by its index
    Done {
        /// The 0-based index of the task to mark as done
        index: usize,
    },
    /// Delete a task by its index
    Delete {
        /// The 0-based index of the task to delete
        index: usize,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse(); // Moved cli parsing to the beginning

    // Initialize TaskManager, loading tasks from "tasks.json"
    let mut task_manager = TaskManager::new("tasks.json")?; // Changed to `let mut` to allow mutable borrowing

    match cli.command {
        Commands::Add { description, due } => {
            let mut due_date_ist: Option<DateTime<FixedOffset>> = None;
            if let Some(date_str) = due {
                // Try parsing with time first
                let parsed_datetime = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M")
                    .or_else(|_| {
                        // If parsing with time fails, try parsing just the date and default time to 00:00
                        NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                            .map(|date| date.and_hms_opt(0, 0, 0).unwrap()) // Default to 00:00:00
                    })
                    .map_err(|e| format!("Invalid date format: {}. Expected YYYY-MM-DD HH:MM or YYYY-MM-DD. Error: {}", date_str, e))?;


                // Define IST offset (UTC+5:30)
                let ist_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60)
                    .ok_or("Failed to create IST offset")?;

                // Combine naive datetime with IST offset to get DateTime<FixedOffset>
                let ist_datetime = ist_offset.from_local_datetime(&parsed_datetime) // Use parsed_datetime
                    .single()
                    .ok_or(format!("Ambiguous or non-existent local time for IST: {}", date_str))?;

                // Store directly as IST
                due_date_ist = Some(ist_datetime);
            }
            task_manager.add_task(description, due_date_ist);
        }
        Commands::List => {
            task_manager.list_tasks();
        }
        Commands::Done { index } => {
            task_manager.mark_task_done(index);
        }
        Commands::Delete { index } => {
            task_manager.delete_task(index);
        }
    }

    Ok(())
}
