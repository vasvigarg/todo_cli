use crate::task::{Task, TaskStatus};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::rc::Rc;
use chrono::{DateTime, FixedOffset}; // Removed TimeZone as it's not directly used here

/// Represents the manager for ToDo tasks, handling storage and operations.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    /// The list of tasks managed by the application.
    /// `Rc<RefCell<...>>` is used to allow shared, mutable access to the tasks
    /// across different parts of the application, especially when passed around.
    #[serde(skip)] // Don't serialize this directly, handle it manually
    pub tasks: Rc<RefCell<Vec<Task>>>,
    /// The file path where tasks are persistently stored.
    #[serde(skip)] // Don't serialize this directly, handle it manually
    file_path: PathBuf,
    /// The next available ID for a new task.
    next_id: usize,
}

impl TaskManager {
    /// Creates a new `TaskManager` instance.
    /// It attempts to load existing tasks from `tasks.json` or initializes an empty list.
    ///
    /// # Arguments
    /// * `file_name` - The name of the JSON file to use for persistent storage.
    pub fn new(file_name: &str) -> io::Result<Self> {
        let file_path = PathBuf::from(file_name);
        let mut manager = TaskManager {
            tasks: Rc::new(RefCell::new(Vec::new())),
            file_path,
            next_id: 0, // Will be updated after loading tasks
        };
        manager.load_tasks()?; // Load tasks on initialization
        Ok(manager)
    }

    /// Loads tasks from the JSON file specified by `file_path`.
    /// If the file does not exist, it initializes an empty task list.
    fn load_tasks(&mut self) -> io::Result<()> {
        if self.file_path.exists() {
            let data = fs::read_to_string(&self.file_path)?;
            let loaded_tasks: Vec<Task> = serde_json::from_str(&data)?;
            // Update next_id based on the highest ID found in loaded tasks
            self.next_id = loaded_tasks
                .iter()
                .map(|t| t.id)
                .max()
                .map_or(0, |max_id| max_id + 1);
            *self.tasks.borrow_mut() = loaded_tasks;
        } else {
            // If file doesn't exist, start with an empty list and ID 0
            *self.tasks.borrow_mut() = Vec::new();
            self.next_id = 0;
        }
        Ok(())
    }

    /// Saves the current list of tasks to the JSON file.
    pub fn save_tasks(&self) -> io::Result<()> {
        let data = serde_json::to_string_pretty(&*self.tasks.borrow())?;
        let mut file = fs::File::create(&self.file_path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    /// Adds a new task to the list.
    ///
    /// # Arguments
    /// * `description` - The description of the new task.
    /// * `due_date` - An optional `DateTime<FixedOffset>` for the task's due date in IST.
    pub fn add_task(&mut self, description: String, due_date: Option<DateTime<FixedOffset>>) {
        let new_task = Task::new(self.next_id, description, due_date);
        self.tasks.borrow_mut().push(new_task);
        self.next_id += 1; // Increment for the next task
        if let Err(e) = self.save_tasks() {
            eprintln!("Error saving tasks: {}", e);
        }
        println!("Task added successfully.");
    }

    /// Lists all tasks, displaying their ID, status, description, and due date.
    pub fn list_tasks(&self) {
        let tasks = self.tasks.borrow();
        if tasks.is_empty() {
            println!("No tasks found. Add one using `todo_cli add \"My task\"`");
            return;
        }

        println!("\n--- Your ToDo Tasks ---");
        for task in tasks.iter() {
            let status_char = match task.status {
                TaskStatus::Pending => ' ',
                TaskStatus::Done => 'x',
            };
            let due_date_str = if let Some(dt) = task.due_date {
                // Since due_date is now stored in IST, just format it directly
                format!(" (Due: {})", dt.format("%Y-%m-%d %H:%M IST"))
            } else {
                String::new()
            };
            println!(
                "[{}] {}. {}{}",
                status_char, task.id, task.description, due_date_str
            );
        }
        println!("-----------------------\n");
    }

    /// Marks a task as done by its index.
    ///
    /// # Arguments
    /// * `index` - The 0-based index of the task in the current list.
    pub fn mark_task_done(&self, index: usize) {
        let mut tasks = self.tasks.borrow_mut();
        if let Some(task) = tasks.get_mut(index) {
            if task.is_pending() {
                task.mark_done();
                if let Err(e) = self.save_tasks() {
                    eprintln!("Error saving tasks: {}", e);
                }
                println!("Task {} marked as done.", task.id);
            } else {
                println!("Task {} is already done.", task.id);
            }
        } else {
            println!("Invalid task index: {}. Use `list` to see available tasks.", index);
        }
    }

    /// Deletes a task by its index.
    ///
    /// # Arguments
    /// * `index` - The 0-based index of the task in the current list.
    pub fn delete_task(&self, index: usize) {
        let mut tasks = self.tasks.borrow_mut();
        if index < tasks.len() {
            let removed_task = tasks.remove(index);
            if let Err(e) = self.save_tasks() {
                eprintln!("Error saving tasks: {}", e);
            }
            println!("Task \"{}\" (ID: {}) deleted.", removed_task.description, removed_task.id);
        } else {
            println!("Invalid task index: {}. Use `list` to see available tasks.", index);
        }
    }
}
