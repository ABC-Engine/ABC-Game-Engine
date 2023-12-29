use std::collections::HashSet;

use winput::message_loop::{self, EventReceiver};
use winput::Action;
pub use winput::Vk;
use ABC_ECS::Resource;

pub struct Input {
    receiver: EventReceiver,
    keys_pressed: HashSet<Vk>,
}

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

impl Resource for Input {
    fn update(&mut self) {
        self.update();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
