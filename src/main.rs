mod manager;
mod task;

use clap::{Parser, Subcommand};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone};

use manager::TaskManager;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        description: String,
        #[arg(long)]
        due: Option<String>,
    },
    List,
    Done {
        index: usize,
    },
    Delete {
        index: usize,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut task_manager = TaskManager::new("tasks.json")?;

    match cli.command {
        Commands::Add { description, due } => {
            let due_date_ist = match due {
                Some(date_str) => Some(parse_due_date(&date_str)?),
                None => None,
            };
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

fn parse_due_date(date_str: &str) -> Result<DateTime<FixedOffset>, String> {
    let naive_datetime = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M")
        .or_else(|_| {
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|e| format!("Invalid date: {}. Error: {}", date_str, e))
                .and_then(|date| {
                    date.and_hms_opt(0, 0, 0)
                        .ok_or_else(|| format!("Could not create time 00:00:00 for date: {}", date_str))
                })
        })
        .map_err(|e| format!(
            "Invalid date format '{}'. Expected 'YYYY-MM-DD HH:MM' or 'YYYY-MM-DD'. Error: {}",
            date_str, e
        ))?;

    let ist_offset = FixedOffset::east_opt(5 * 3600 + 30 * 60)
        .ok_or_else(|| "Failed to create IST offset.".to_string())?;

    let datetime = ist_offset
        .from_local_datetime(&naive_datetime)
        .single()
        .ok_or_else(|| format!("Ambiguous or non-existent local time: '{}'", date_str))?;

    Ok(datetime)
}
