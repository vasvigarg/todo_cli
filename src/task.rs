use chrono::{DateTime, FixedOffset}; // Changed to FixedOffset for IST storage
use serde::{Deserialize, Serialize};

/// Represents the status of a ToDo task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is pending completion.
    Pending,
    /// Task has been marked as done.
    Done,
}

/// Represents a single ToDo task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier for the task.
    pub id: usize,
    /// Description of the task.
    pub description: String,
    /// Current status of the task (Pending or Done).
    pub status: TaskStatus,
    /// Optional due date for the task in IST (FixedOffset).
    pub due_date: Option<DateTime<FixedOffset>>, // Changed to FixedOffset
}

impl Task {
    /// Creates a new `Task` instance.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the task.
    /// * `description` - A string describing the task.
    /// * `due_date` - An optional `DateTime<FixedOffset>` representing the task's due date in IST.
    pub fn new(id: usize, description: String, due_date: Option<DateTime<FixedOffset>>) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Pending, // New tasks are always pending
            due_date,
        }
    }

    /// Marks the task's status as `Done`.
    pub fn mark_done(&mut self) {
        self.status = TaskStatus::Done;
    }

    /// Checks if the task is currently pending.
    pub fn is_pending(&self) -> bool {
        self.status == TaskStatus::Pending
    }
}
