use crate::storage::{self, load_tasks_from_files, save_task_to_file};
use serde::{Deserialize, Serialize};
use std::fs;

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

    pub fn add_task(&mut self, description: String) -> Result<usize, std::io::Error> {
        let id = self.next_id;
        let task = Task {
            id,
            description,
            completed: false,
        };

        match save_task_to_file(&task) {
            Ok(_) => {
                self.tasks.push(task);
                self.next_id += 1;
                Ok(id)
            }
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            )),
        }
    }

    pub fn complete_task(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            if save_task_to_file(task).is_ok() {
                true
            } else {
                task.completed = false; // revert
                false
            }
        } else {
            false
        }
    }

    pub fn remove_task(&mut self, id: usize) -> bool {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            let tasks_dir = storage::get_tasks_dir();
            let file_path = tasks_dir.join(format!("{}.json", id));
            if fs::remove_file(file_path).is_ok() {
                self.tasks.remove(pos);
                true
            } else {
                false
            }
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

pub fn load_tasks() -> TaskList {
    load_tasks_from_files()
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
}

