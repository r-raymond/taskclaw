use crate::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    pub fn create_task(&mut self, title: String) -> Task {
        let index = self.tasks.len();
        let task = Task::new(title, index);
        self.tasks.push(task.clone());
        task
    }

    pub fn remove_task_by_uuid(&mut self, uuid: &uuid::Uuid) {
        self.tasks.retain(|task| task.uuid != *uuid);
    }

    pub fn remove_task_by_index(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        }
    }

    pub fn get_tasks_by_uuid(&self, uuid: &uuid::Uuid) -> Option<&Task> {
        self.tasks.iter().find(|task| task.uuid == *uuid)
    }

    pub fn get_tasks_by_index(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Task;
    use uuid::Uuid;

    // Helper to create a dummy TaskManager and add tasks
    fn setup_manager_with_tasks(n: usize) -> TaskManager {
        let mut manager = TaskManager::new();
        for i in 0..n {
            manager.create_task(format!("Task {}", i));
        }
        manager
    }

    #[test]
    fn test_create_task_normal() {
        let mut manager = TaskManager::new();
        let task = manager.create_task("Test Task".to_string());
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.index, 0);
        assert_eq!(manager.tasks.len(), 1);
        assert_eq!(manager.tasks[0].uuid, task.uuid);
    }

    #[test]
    fn test_create_task_empty_title() {
        let mut manager = TaskManager::new();
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
