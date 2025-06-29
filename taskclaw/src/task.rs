use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub uuid: Uuid,
    pub index: usize,
    pub title: String,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
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
        }
    }
}
