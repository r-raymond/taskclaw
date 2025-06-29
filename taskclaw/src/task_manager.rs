use crate::config::Config;
use crate::task::{Task, TaskInStore, task_from_store};
use std::fs;
use std::io::{self, Write};
use std::str::FromStr;
use uuid::Uuid;

pub struct TaskManager {
    tasks: Vec<Task>,
    next_index: usize,
    config: Config,
}

#[derive(Debug, Clone)]
pub enum UUIDorIndex {
    UUID(Uuid),
    Index(usize),
}

impl FromStr for UUIDorIndex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(idx) = s.parse::<usize>() {
            Ok(UUIDorIndex::Index(idx))
        } else {
            match Uuid::parse_str(s) {
                Ok(uuid) => Ok(UUIDorIndex::UUID(uuid)),
                Err(_) => Err(format!("Invalid UUID or index: {}", s)),
            }
        }
    }
}

impl TaskManager {
    pub fn new(config: Config) -> Self {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            next_index: 0,
            config,
        };
        manager.load_tasks();
        manager
    }

    fn save_task(&self, task: &Task) -> io::Result<()> {
        if let Some(task_store_path) = self.config.get_task_store_path() {
            fs::create_dir_all(&task_store_path)?;
            let file_path = task_store_path.join(format!("{}.json", task.uuid));
            let json = serde_json::to_string_pretty(task)?;
            let mut file = fs::File::create(file_path)?;
            file.write_all(json.as_bytes())?;
        }
        Ok(())
    }

    fn load_tasks(&mut self) {
        if let Some(task_store_path) = self.config.get_task_store_path() {
            if task_store_path.exists() {
                for entry in fs::read_dir(task_store_path).expect("Failed to read task directory") {
                    let entry = entry.expect("Failed to read directory entry");
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                        let json = fs::read_to_string(&path).expect("Failed to read task file");
                        let task_in_store: TaskInStore =
                            serde_json::from_str(&json).expect("Failed to parse task JSON");
                        if let Some(file_stem) = path.file_stem() {
                            if let Some(uuid_str) = file_stem.to_str() {
                                if let Ok(uuid) = Uuid::parse_str(uuid_str) {
                                    let task = task_from_store(task_in_store, uuid);
                                    self.tasks.push(task);
                                } else {
                                    eprintln!(
                                        "Warning: Could not parse UUID from filename: {}",
                                        path.display()
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn create_task(&mut self, title: String) -> Task {
        let task = Task::new(title, self.next_index);
        self.next_index += 1;
        self.tasks.push(task.clone());
        self.save_task(&task).expect("Failed to save task");
        task
    }

    pub fn remove_task(&mut self, id: UUIDorIndex) -> Option<Task> {
        let search = match id {
            UUIDorIndex::UUID(uuid) => self.tasks.iter().position(|task| task.uuid == uuid),
            UUIDorIndex::Index(id) => self.tasks.iter().position(|task| task.index == id),
        };

        if let Some(index) = search {
            let removed_task = self.tasks.remove(index);
            if let Some(task_store_path) = self.config.get_task_store_path() {
                let file_path = task_store_path.join(format!("{}.json", removed_task.uuid));
                if file_path.exists() {
                    fs::remove_file(file_path).expect("Failed to delete task file");
                }
            }
            Some(removed_task)
        } else {
            None
        }
    }

    pub fn get_task(&self, id: UUIDorIndex) -> Option<&Task> {
        match id {
            UUIDorIndex::UUID(uuid) => self.tasks.iter().find(|task| task.uuid == uuid),
            UUIDorIndex::Index(index) => self.tasks.iter().find(|task| task.index == index),
        }
    }

    pub fn get_tasks_by_priority(&self, amount: usize) -> Vec<&Task> {
        self.tasks.iter().take(amount).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use uuid::Uuid;

    // Helper to create a dummy TaskManager and add tasks
    fn setup_manager_with_tasks(n: usize) -> TaskManager {
        let mut manager = TaskManager::new(Config::default());
        for i in 0..n {
            manager.create_task(format!("Task {}", i));
        }
        manager
    }

    #[test]
    fn test_create_task_normal() {
        let mut manager = TaskManager::new(Config::default());
        let task = manager.create_task("Test Task".to_string());
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.index, 0);
        assert_eq!(manager.tasks.len(), 1);
        assert_eq!(manager.tasks[0].uuid, task.uuid);
    }

    #[test]
    fn test_create_task_empty_title() {
        let mut manager = TaskManager::new(Config::default());
        let task = manager.create_task("".to_string());
        assert_eq!(task.title, "");
        assert_eq!(task.index, 0);
        assert_eq!(manager.tasks.len(), 1);
    }

    #[test]
    fn test_remove_task_by_uuid_existing() {
        let mut manager = setup_manager_with_tasks(2);
        let uuid = manager.tasks[0].uuid;
        manager.remove_task(UUIDorIndex::UUID(uuid));
        assert_eq!(manager.tasks.len(), 1);
        assert!(manager.tasks.iter().all(|t| t.uuid != uuid));
    }

    #[test]
    fn test_remove_task_by_uuid_non_existing() {
        let mut manager = setup_manager_with_tasks(2);
        let fake_uuid = Uuid::now_v7();
        let len_before = manager.tasks.len();
        manager.remove_task(UUIDorIndex::UUID(fake_uuid));
        assert_eq!(manager.tasks.len(), len_before);
    }

    #[test]
    fn test_remove_task_by_index_valid() {
        let mut manager = setup_manager_with_tasks(3);
        manager.remove_task(UUIDorIndex::Index(1));
        assert_eq!(manager.tasks.len(), 2);
        // Ensure the correct task was removed
        assert_eq!(manager.tasks[0].index, 0);
        assert_eq!(manager.tasks[1].index, 2);
    }

    #[test]
    fn test_remove_task_by_index_out_of_bounds() {
        let mut manager = setup_manager_with_tasks(2);
        manager.remove_task(UUIDorIndex::Index(5));
        assert_eq!(manager.tasks.len(), 2);
    }

    #[test]
    fn test_get_tasks_by_uuid_existing() {
        let manager = setup_manager_with_tasks(2);
        let uuid = manager.tasks[1].uuid;
        let task = manager.get_task(UUIDorIndex::UUID(uuid));
        assert!(task.is_some());
        assert_eq!(task.unwrap().uuid, uuid);
    }

    #[test]
    fn test_get_tasks_by_uuid_non_existing() {
        let manager = setup_manager_with_tasks(2);
        let fake_uuid = Uuid::now_v7();
        let task = manager.get_task(UUIDorIndex::UUID(fake_uuid));
        assert!(task.is_none());
    }

    #[test]
    fn test_get_tasks_by_index_valid() {
        let manager = setup_manager_with_tasks(2);
        let task = manager.get_task(UUIDorIndex::Index(1));
        assert!(task.is_some());
        assert_eq!(task.unwrap().index, 1);
    }

    #[test]
    fn test_get_tasks_by_index_out_of_bounds() {
        let manager = setup_manager_with_tasks(2);
        let task = manager.get_task(UUIDorIndex::Index(5));
        assert!(task.is_none());
    }

    #[test]
    fn test_save_and_load_tasks() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let store_location = temp_dir.path().to_path_buf();
        let config = Config {
            store_location: Some(store_location.clone()),
        };

        // Create a manager and add some tasks
        let mut manager = TaskManager::new(config.clone());
        let t1 = manager.create_task("Task 1".to_string());
        let t2 = manager.create_task("Task 2".to_string());
        let t3 = manager.create_task("Task 3".to_string());
        assert_eq!(manager.tasks.len(), 3);

        // Create a new manager with the same config to load tasks
        let new_manager = TaskManager::new(config.clone());
        assert_eq!(new_manager.tasks.len(), 3);

        // Verify loaded tasks
        assert_eq!(
            new_manager
                .get_task(UUIDorIndex::UUID(t1.uuid))
                .unwrap()
                .title,
            "Task 1"
        );
        assert_eq!(
            new_manager
                .get_task(UUIDorIndex::UUID(t2.uuid))
                .unwrap()
                .title,
            "Task 2"
        );
        assert_eq!(
            new_manager
                .get_task(UUIDorIndex::UUID(t3.uuid))
                .unwrap()
                .title,
            "Task 3"
        );

        // Clean up
        temp_dir
            .close()
            .expect("Failed to clean up temporary directory");
    }

    #[test]
    fn test_remove_task_deletes_file() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let store_location = temp_dir.path().to_path_buf();
        let config = Config {
            store_location: Some(store_location.clone()),
        };

        let mut manager = TaskManager::new(config.clone());
        let task = manager.create_task("Task to be removed".to_string());
        let task_file_path = store_location
            .join("tasks")
            .join(format!("{}.json", task.uuid));
        assert!(task_file_path.exists());

        manager.remove_task(UUIDorIndex::UUID(task.uuid));
        assert!(!task_file_path.exists());

        temp_dir
            .close()
            .expect("Failed to clean up temporary directory");
    }
}
