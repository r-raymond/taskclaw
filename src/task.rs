use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    pub next_id: usize,
}

impl TaskList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_task(&mut self, description: String) -> usize {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        let id = task.id;
        self.tasks.push(task);
        self.next_id += 1;
        id
    }

    pub fn complete_task(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            true
        } else {
            false
        }
    }

    pub fn remove_task(&mut self, id: usize) -> bool {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_task(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

pub fn get_data_file() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".taskclaw.json");
    path
}

pub fn load_tasks() -> TaskList {
    let data_file = get_data_file();
    if data_file.exists() {
        let content = fs::read_to_string(&data_file).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        TaskList::default()
    }
}

pub fn save_tasks(task_list: &TaskList) -> Result<(), Box<dyn std::error::Error>> {
    let data_file = get_data_file();
    let content = serde_json::to_string_pretty(task_list)?;
    fs::write(&data_file, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task {
            id: 1,
            description: "Test task".to_string(),
            completed: false,
        };

        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Test task");
        assert!(!task.completed);
    }

    #[test]
    fn test_tasklist_new() {
        let task_list = TaskList::new();
        assert!(task_list.is_empty());
        assert_eq!(task_list.len(), 0);
        assert_eq!(task_list.next_id, 0);
    }

    #[test]
    fn test_add_task() {
        let mut task_list = TaskList::new();
        let id = task_list.add_task("Test task".to_string());

        assert_eq!(id, 0);
        assert_eq!(task_list.len(), 1);
        assert_eq!(task_list.next_id, 1);

        let task = task_list.get_task(0).unwrap();
        assert_eq!(task.description, "Test task");
        assert!(!task.completed);
    }

    #[test]
    fn test_complete_task() {
        let mut task_list = TaskList::new();
        let id = task_list.add_task("Test task".to_string());

        assert!(task_list.complete_task(id));
        let task = task_list.get_task(id).unwrap();
        assert!(task.completed);

        // Test completing non-existent task
        assert!(!task_list.complete_task(999));
    }

    #[test]
    fn test_remove_task() {
        let mut task_list = TaskList::new();
        let id = task_list.add_task("Test task".to_string());

        assert_eq!(task_list.len(), 1);
        assert!(task_list.remove_task(id));
        assert_eq!(task_list.len(), 0);
        assert!(task_list.get_task(id).is_none());

        // Test removing non-existent task
        assert!(!task_list.remove_task(999));
    }

    #[test]
    fn test_multiple_tasks() {
        let mut task_list = TaskList::new();

        let id1 = task_list.add_task("First task".to_string());
        let id2 = task_list.add_task("Second task".to_string());
        let id3 = task_list.add_task("Third task".to_string());

        assert_eq!(task_list.len(), 3);
        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(id3, 2);

        task_list.complete_task(id2);
        assert!(task_list.get_task(id2).unwrap().completed);
        assert!(!task_list.get_task(id1).unwrap().completed);
        assert!(!task_list.get_task(id3).unwrap().completed);

        task_list.remove_task(id1);
        assert_eq!(task_list.len(), 2);
        assert!(task_list.get_task(id1).is_none());
    }

    #[test]
    fn test_task_serialization() {
        let mut task_list = TaskList::new();
        task_list.add_task("Test task".to_string());
        task_list.complete_task(0);

        let json = serde_json::to_string(&task_list).unwrap();
        let deserialized: TaskList = serde_json::from_str(&json).unwrap();

        assert_eq!(task_list, deserialized);
    }
}
