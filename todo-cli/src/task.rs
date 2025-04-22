use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(description: String) -> Self {
        Task {
            id: Uuid::new_v4().to_string(),
            description,
            created_at: Utc::now(),
            status: TaskStatus::Pending,
        }
    }

    pub fn mark_completed(&mut self) {
        self.status = TaskStatus::Completed;
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, TaskStatus::Completed)
    }
}