use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use std::time::Duration;

pub fn next_event() -> io::Result<Event> {
    event::poll(Duration::from_millis(100))?;
    event::read()
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    Key(KeyCode),
    KeyWithModifiers(KeyCode, KeyModifiers),
    Quit,
    Unknown,
}

impl From<Event> for InputEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => InputEvent::Quit,
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => InputEvent::Quit,
            Event::Key(KeyEvent { code, modifiers, .. }) => {
                if modifiers.is_empty() {
                    InputEvent::Key(code)
                } else {
                    InputEvent::KeyWithModifiers(code, modifiers)
                }
            }
            _ => InputEvent::Unknown,
        }
    }
}