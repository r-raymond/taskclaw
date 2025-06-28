#[cfg(test)]
pub mod test_helpers {
    use crate::config::Config;
    use crate::task::TaskList;
    use tempfile::TempDir;

    pub fn create_test_config() -> Config {
        Config {
            data_format: Some("json".to_string()),
            default_priority: Some("high".to_string()),
            show_completed: Some(false),
        }
    }

    pub fn create_sample_task_list() -> TaskList {
        let mut task_list = TaskList::new();
        task_list.add_task("First task".to_string());
        task_list.add_task("Second task".to_string());
        task_list.add_task("Third task".to_string());
        task_list.complete_task(1); // Complete second task
        task_list
    }

    pub fn create_temp_dir_with_tasks() -> TempDir {
        let temp_dir = tempfile::tempdir().unwrap();
        let task_list = create_sample_task_list();

        // Set up temporary data file
        unsafe {
            std::env::set_var("HOME", temp_dir.path());
        }
        crate::task::save_tasks(&task_list).unwrap();

        temp_dir
    }

    pub fn assert_task_exists(task_list: &TaskList, id: usize, description: &str, completed: bool) {
        let task = task_list
            .get_task(id)
            .unwrap_or_else(|| panic!("Task {} should exist", id));
        assert_eq!(task.description, description);
        assert_eq!(task.completed, completed);
    }

    pub fn count_completed_tasks(task_list: &TaskList) -> usize {
        task_list.tasks.iter().filter(|t| t.completed).count()
    }

    pub fn count_pending_tasks(task_list: &TaskList) -> usize {
        task_list.tasks.iter().filter(|t| !t.completed).count()
    }
}

#[cfg(test)]
mod tests {
    use super::test_helpers::*;
    use crate::task::TaskList;

    #[test]
    fn test_create_test_config() {
        let config = create_test_config();
        assert_eq!(config.data_format, Some("json".to_string()));
        assert_eq!(config.default_priority, Some("high".to_string()));
        assert_eq!(config.show_completed, Some(false));
    }

    #[test]
    fn test_create_sample_task_list() {
        let task_list = create_sample_task_list();
        assert_eq!(task_list.len(), 3);
        assert_eq!(count_completed_tasks(&task_list), 1);
        assert_eq!(count_pending_tasks(&task_list), 2);

        assert_task_exists(&task_list, 0, "First task", false);
        assert_task_exists(&task_list, 1, "Second task", true);
        assert_task_exists(&task_list, 2, "Third task", false);
    }

    #[test]
    fn test_task_counting() {
        let mut task_list = TaskList::new();
        assert_eq!(count_completed_tasks(&task_list), 0);
        assert_eq!(count_pending_tasks(&task_list), 0);

        task_list.add_task("Task 1".to_string());
        task_list.add_task("Task 2".to_string());
        assert_eq!(count_completed_tasks(&task_list), 0);
        assert_eq!(count_pending_tasks(&task_list), 2);

        task_list.complete_task(0);
        assert_eq!(count_completed_tasks(&task_list), 1);
        assert_eq!(count_pending_tasks(&task_list), 1);
    }
}
