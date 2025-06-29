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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskInStore {
    pub index: usize,
    pub title: String,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub project: Option<String>,
    pub due_date: Option<std::time::SystemTime>,
}

impl From<Task> for TaskInStore {
    fn from(task: Task) -> Self {
        TaskInStore {
            index: task.index,
            title: task.title,
            created_at: task.created_at,
            updated_at: task.updated_at,
            tags: task.tags,
            description: task.description,
            project: task.project,
            due_date: task.due_date,
        }
    }
}

pub fn task_from_store(task: TaskInStore, uuid: Uuid) -> Task {
    Task {
        uuid,
        index: task.index,
        title: task.title,
        created_at: task.created_at,
        updated_at: task.updated_at,
        tags: task.tags,
        description: task.description,
        project: task.project,
        due_date: task.due_date,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_task_new() {
        let title = String::from("Test Task");
        let index = 1;
        let task = Task::new(title.clone(), index);
        assert_eq!(task.title, title);
        assert_eq!(task.index, index);
        assert!(task.tags.is_empty());
        assert!(task.description.is_none());
        assert!(task.project.is_none());
        assert!(task.due_date.is_none());
    }

    #[test]
    fn test_task_display() {
        let title = String::from("Display Task");
        let index = 42;
        let task = Task::new(title.clone(), index);
        let display = format!("{}", task);
        assert_eq!(display, format!("[{}]: {}", index, title));
    }

    #[test]
    fn test_task_clone() {
        let task1 = Task::new("Clone Task".to_string(), 2);
        let task2 = task1.clone();
        assert_eq!(task1.title, task2.title);
        assert_eq!(task1.index, task2.index);
        assert_eq!(task1.uuid, task2.uuid);
    }

    #[test]
    fn test_task_timestamps() {
        let task = Task::new("Timestamp Task".to_string(), 3);
        let now = SystemTime::now();
        // Allow a small difference due to execution time
        assert!(task.created_at <= now);
        assert!(task.updated_at <= now);
        assert!(now.duration_since(task.created_at).unwrap() < Duration::from_secs(5));
        assert!(now.duration_since(task.updated_at).unwrap() < Duration::from_secs(5));
    }
}
