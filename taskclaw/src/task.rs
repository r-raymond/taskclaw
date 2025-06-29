use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub uuid: Uuid,
    pub index: usize,
    pub title: String,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub project: Option<String>,
    pub due_date: Option<std::time::SystemTime>,
}

impl Task {
    pub fn new(title: String, index: usize) -> Self {
        let uuid = Uuid::now_v7();
        Task {
            uuid,
            title,
            index,
            created_at: std::time::SystemTime::now(),
            updated_at: std::time::SystemTime::now(),
            tags: Vec::new(),
            description: None,
            project: None,
            due_date: None,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.index, self.title)
    }
}
