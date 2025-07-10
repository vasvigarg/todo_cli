use crate::task::{Task, TaskStatus};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::rc::Rc;
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    #[serde(skip)]
    pub tasks: Rc<RefCell<Vec<Task>>>,
    #[serde(skip)]
    file_path: PathBuf,
    next_id: usize,
}

impl TaskManager {
    pub fn new(file_name: &str) -> io::Result<Self> {
        let file_path = PathBuf::from(file_name);
        let mut manager = TaskManager {
            tasks: Rc::new(RefCell::new(Vec::new())),
            file_path,
            next_id: 0,
        };
        manager.load_tasks()?;
        Ok(manager)
    }

    fn load_tasks(&mut self) -> io::Result<()> {
        if self.file_path.exists() {
            let data = fs::read_to_string(&self.file_path)?;
            let loaded_tasks: Vec<Task> = serde_json::from_str(&data)?;
            self.next_id = loaded_tasks
                .iter()
                .map(|t| t.id)
                .max()
                .map_or(0, |max_id| max_id + 1);
            *self.tasks.borrow_mut() = loaded_tasks;
        } else {
            *self.tasks.borrow_mut() = Vec::new();
            self.next_id = 0;
        }
        Ok(())
    }

    pub fn save_tasks(&self) -> io::Result<()> {
        let data = serde_json::to_string_pretty(&*self.tasks.borrow())?;
        let mut file = fs::File::create(&self.file_path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn add_task(&mut self, description: String, due_date: Option<DateTime<FixedOffset>>) {
        let new_task = Task::new(self.next_id, description, due_date);
        self.tasks.borrow_mut().push(new_task);
        self.next_id += 1;
        if let Err(e) = self.save_tasks() {
            eprintln!("Error saving tasks: {}", e);
        }
        println!("Task added successfully.");
    }

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
