use core::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub fn get_input() -> Option<KeyCode> {
    if event::poll(Duration::from_secs(0)).expect("Error polling for event") {
        if let Event::Key(key_event) = event::read().expect("Error reading event") {
            if let KeyEvent {
                code,
                modifiers,
                kind,
                state,
            } = key_event
            {
                return Option::Some(code);
            }
        }
    }
    return Option::None;
}
