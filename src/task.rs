use chrono::{DateTime, FixedOffset}; 
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub status: TaskStatus,
    pub due_date: Option<DateTime<FixedOffset>>, 
}

impl Task {
    pub fn new(id: usize, description: String, due_date: Option<DateTime<FixedOffset>>) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Pending, 
            due_date,
        }
    }

    pub fn mark_done(&mut self) {
        self.status = TaskStatus::Done;
    }

    pub fn is_pending(&self) -> bool {
        self.status == TaskStatus::Pending
    }
}
