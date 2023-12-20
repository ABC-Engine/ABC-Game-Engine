/*use core::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};

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
*/

use std::collections::HashSet;

use winput::message_loop::{self, EventReceiver};
use winput::Action;
pub use winput::Vk;
use ABC_ECS::Component;

pub struct Input {
    receiver: EventReceiver,
    keys_pressed: HashSet<Vk>,
}

impl Component for Input {}

impl Input {
    pub fn new() -> Self {
        let receiver =
            message_loop::start().expect("failed to start message loop for input system");
        let keys_pressed = HashSet::new();
        Self {
            receiver,
            keys_pressed,
        }
    }

    pub fn update(&mut self) {
        if let Some(next_event) = self.receiver.try_next_event() {
            match next_event {
                message_loop::Event::Keyboard {
                    vk,
                    action: Action::Press,
                    ..
                } => {
                    self.keys_pressed.insert(vk);
                }

                message_loop::Event::Keyboard {
                    vk,
                    action: Action::Release,
                    ..
                } => {
                    self.keys_pressed.remove(&vk);
                }
                _ => (),
            }
        }
    }

    pub fn is_key_pressed(&self, key: Vk) -> bool {
        self.keys_pressed.contains(&key)
    }
}
