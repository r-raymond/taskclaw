use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::task::{Task, TaskList};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,
    Insert,
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub task_list: TaskList,
    pub mode: AppMode,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub selected_task: usize,
    pub show_help: bool,
    pub should_quit: bool,
    pub status_message: String,
}

impl App {
    pub fn new(task_list: TaskList) -> Self {
        Self {
            task_list,
            mode: AppMode::Normal,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            selected_task: 0,
            show_help: false,
            should_quit: false,
            status_message: "Press 'h' for help, 'q' to quit".to_string(),
        }
    }

    pub fn into_task_list(self) -> TaskList {
        self.task_list
    }

    pub fn handle_event(&mut self, event: Event) -> bool {
        if self.should_quit {
            return false;
        }

        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => {}
        }

        !self.should_quit
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode(key_event),
            InputMode::Editing => self.handle_editing_mode(key_event),
        }
    }

    fn handle_normal_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Char('h') => {
                self.show_help = !self.show_help;
                self.mode = if self.show_help {
                    AppMode::Help
                } else {
                    AppMode::Normal
                };
            }
            KeyCode::Char('a') => {
                self.input_mode = InputMode::Editing;
                self.mode = AppMode::Insert;
                self.input_buffer.clear();
                self.status_message = "Enter task description (ESC to cancel, Enter to save)".to_string();
            }
            KeyCode::Char('d') => {
                if !self.task_list.tasks.is_empty() {
                    if let Some(task) = self.get_selected_task() {
                        let task_id = task.id;
                        if self.task_list.remove_task(task_id) {
                            self.status_message = format!("Deleted task {}", task_id);
                            if self.selected_task >= self.task_list.tasks.len() && self.selected_task > 0 {
                                self.selected_task -= 1;
                            }
                        }
                    }
                }
            }
            KeyCode::Char(' ') => {
                if let Some(task) = self.get_selected_task() {
                    let task_id = task.id;
                    if self.task_list.complete_task(task_id) {
                        self.status_message = format!("Completed task {}", task_id);
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_task > 0 {
                    self.selected_task -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.task_list.tasks.is_empty() && self.selected_task < self.task_list.tasks.len() - 1 {
                    self.selected_task += 1;
                }
            }
            KeyCode::Home | KeyCode::Char('g') => {
                self.selected_task = 0;
            }
            KeyCode::End | KeyCode::Char('G') => {
                if !self.task_list.tasks.is_empty() {
                    self.selected_task = self.task_list.tasks.len() - 1;
                }
            }
            KeyCode::Char('r') => {
                self.status_message = "Tasks refreshed".to_string();
            }
            _ => {}
        }
    }

    fn handle_editing_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Enter => {
                if !self.input_buffer.trim().is_empty() {
                    self.task_list.add_task(self.input_buffer.clone());
                    self.status_message = format!("Added task: {}", self.input_buffer);
                    self.input_buffer.clear();
                } else {
                    self.status_message = "Task description cannot be empty".to_string();
                }
                self.input_mode = InputMode::Normal;
                self.mode = AppMode::Normal;
            }
            KeyCode::Esc => {
                self.input_buffer.clear();
                self.input_mode = InputMode::Normal;
                self.mode = AppMode::Normal;
                self.status_message = "Cancelled".to_string();
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
    }

    fn get_selected_task(&self) -> Option<&Task> {
        self.task_list.tasks.get(self.selected_task)
    }

    pub fn get_help_text(&self) -> Vec<&'static str> {
        vec![
            "Keyboard Shortcuts:",
            "",
            "Navigation:",
            "  ↑/k     - Move up",
            "  ↓/j     - Move down", 
            "  Home/g  - Go to top",
            "  End/G   - Go to bottom",
            "",
            "Actions:",
            "  a       - Add new task",
            "  Space   - Toggle task completion",
            "  d       - Delete selected task",
            "  r       - Refresh tasks",
            "",
            "General:",
            "  h       - Toggle this help",
            "  q       - Quit application",
            "  Ctrl+C  - Force quit",
            "",
            "Insert Mode:",
            "  Enter   - Save task",
            "  Esc     - Cancel editing",
            "",
            "Press 'h' again to close this help",
        ]
    }
}