use crate::config::Config;
use crate::task::Task;
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
        TaskManager {
            tasks: Vec::new(),
            next_index: 0,
            config,
        }
    }

    pub fn create_task(&mut self, title: String) -> Task {
        let task = Task::new(title, self.next_index);
        self.next_index += 1;
        self.tasks.push(task.clone());
        task
    }

    pub fn remove_task(&mut self, id: UUIDorIndex) -> Option<Task> {
        let search = match id {
            UUIDorIndex::UUID(uuid) => self.tasks.iter().position(|task| task.uuid == uuid),
            UUIDorIndex::Index(id) => self.tasks.iter().position(|task| task.index == id),
        };

        if let Some(index) = search {
            let removed_task = self.tasks.remove(index);
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
        manager.remove_task_by_uuid(&uuid);
        assert_eq!(manager.tasks.len(), 1);
        assert!(manager.tasks.iter().all(|t| t.uuid != uuid));
    }

    #[test]
    fn test_remove_task_by_uuid_non_existing() {
        let mut manager = setup_manager_with_tasks(2);
        let fake_uuid = Uuid::now_v7();
        let len_before = manager.tasks.len();
        manager.remove_task_by_uuid(&fake_uuid);
        assert_eq!(manager.tasks.len(), len_before);
    }

    #[test]
    fn test_remove_task_by_index_valid() {
        let mut manager = setup_manager_with_tasks(3);
        manager.remove_task_by_index(1);
        assert_eq!(manager.tasks.len(), 2);
        // Ensure the correct task was removed
        assert_eq!(manager.tasks[0].index, 0);
        assert_eq!(manager.tasks[1].index, 2);
    }

    #[test]
    fn test_remove_task_by_index_out_of_bounds() {
        let mut manager = setup_manager_with_tasks(2);
        manager.remove_task_by_index(5);
        assert_eq!(manager.tasks.len(), 2);
    }

    #[test]
    fn test_get_tasks_by_uuid_existing() {
        let manager = setup_manager_with_tasks(2);
        let uuid = manager.tasks[1].uuid;
        let task = manager.get_tasks_by_uuid(&uuid);
        assert!(task.is_some());
        assert_eq!(task.unwrap().uuid, uuid);
    }

    #[test]
    fn test_get_tasks_by_uuid_non_existing() {
        let manager = setup_manager_with_tasks(2);
        let fake_uuid = Uuid::now_v7();
        let task = manager.get_tasks_by_uuid(&fake_uuid);
        assert!(task.is_none());
    }

    #[test]
    fn test_get_tasks_by_index_valid() {
        let manager = setup_manager_with_tasks(2);
        let task = manager.get_tasks_by_index(1);
        assert!(task.is_some());
        assert_eq!(task.unwrap().index, 1);
    }

    #[test]
    fn test_get_tasks_by_index_out_of_bounds() {
        let manager = setup_manager_with_tasks(2);
        let task = manager.get_tasks_by_index(5);
        assert!(task.is_none());
    }
}
