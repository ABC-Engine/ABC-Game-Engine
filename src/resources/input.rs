use fxhash::{FxHashMap, FxHashSet};
use gilrs::{Event, Gilrs};
use ABC_ECS::Resource;

// Stolen directly from winit, this is a list of all the keycodes that can be pressed on a keyboard.
// I copy pasted this because I don't want to depend on winit in this crate.
// The keycodes might also change in the future, so it's better to have a copy of them here.
/// Key codes for keyboard keys.
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum KeyCode {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    Backspace,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,

    Apostrophe,
    Asterisk,
    Backslash,
    Capital,
    Colon,
    Comma,
    Convert,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Mute,
    MyComputer,
    // also called "Next"
    NavigateForward,
    // also called "Prior"
    NavigateBackward,
    NextTrack,
    NoConvert,
    OEM102,
    Period,
    PlayPause,
    Plus,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GamepadButton {
    Start,
    Menu,
    Select,

    /// The north face button.
    ///
    /// * Nintendo: X
    /// * Playstation: Triangle
    /// * XBox: Y
    North,
    /// The south face button.
    ///
    /// * Nintendo: B
    /// * Playstation: X
    /// * XBox: A
    South,
    /// The east face button.
    ///
    /// * Nintendo: A
    /// * Playstation: Circle
    /// * XBox: B
    East,
    /// The west face button.
    ///
    /// * Nintendo: Y
    /// * Playstation: Square
    /// * XBox: X
    West,

    LeftStick,
    RightStick,

    LeftTrigger,
    RightTrigger,
    LeftBumper,
    RightBumper,

    LeftShoulder,
    RightShoulder,

    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

/// The state of a key relative to the previous frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum KeyState {
    /// NotPressed is sent every frame that a key is not pressed. not including the first frame it is released, released is sent instead.
    NotPressed,
    /// Pressed is sent the frame that a key is pressed.
    Pressed,
    /// Held is sent every frame that a key is held down. including the first frame.
    Held,
    /// Released is sent the frame that a key is released.
    Released,
}

impl Ord for KeyState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (KeyState::NotPressed, KeyState::NotPressed) => std::cmp::Ordering::Equal,
            (KeyState::NotPressed, KeyState::Pressed) => std::cmp::Ordering::Less,
            (KeyState::NotPressed, KeyState::Held) => std::cmp::Ordering::Less,
            (KeyState::NotPressed, KeyState::Released) => std::cmp::Ordering::Less,

            (KeyState::Pressed, KeyState::NotPressed) => std::cmp::Ordering::Greater,
            (KeyState::Pressed, KeyState::Pressed) => std::cmp::Ordering::Equal,
            (KeyState::Pressed, KeyState::Held) => std::cmp::Ordering::Less,
            (KeyState::Pressed, KeyState::Released) => std::cmp::Ordering::Less,

            (KeyState::Held, KeyState::NotPressed) => std::cmp::Ordering::Greater,
            (KeyState::Held, KeyState::Pressed) => std::cmp::Ordering::Greater,
            (KeyState::Held, KeyState::Held) => std::cmp::Ordering::Equal,
            (KeyState::Held, KeyState::Released) => std::cmp::Ordering::Less,

            (KeyState::Released, KeyState::NotPressed) => std::cmp::Ordering::Greater,
            (KeyState::Released, KeyState::Pressed) => std::cmp::Ordering::Greater,
            (KeyState::Released, KeyState::Held) => std::cmp::Ordering::Greater,
            (KeyState::Released, KeyState::Released) => std::cmp::Ordering::Equal,
        }
    }
}

impl KeyState {
    fn next_state(&self, pressed: bool) -> Self {
        match self {
            KeyState::NotPressed => {
                if pressed {
                    KeyState::Pressed
                } else {
                    KeyState::NotPressed
                }
            }
            KeyState::Pressed => {
                if pressed {
                    KeyState::Held
                } else {
                    KeyState::Released
                }
            }
            KeyState::Held => {
                if pressed {
                    KeyState::Held
                } else {
                    KeyState::Released
                }
            }
            KeyState::Released => {
                if pressed {
                    KeyState::Pressed
                } else {
                    KeyState::NotPressed
                }
            }
        }
    }
}

/// For either a key or a mouse button.
/// (note: This naming seems a bit off, if you have a better name, please suggest it.)
pub enum Key {
    KeyCode(KeyCode),
    MouseButton(MouseButton),
    GamepadButton((GamepadButton, Option<u32>)),
}

impl Into<Key> for KeyCode {
    fn into(self) -> Key {
        Key::KeyCode(self)
    }
}

impl Into<Key> for MouseButton {
    fn into(self) -> Key {
        Key::MouseButton(self)
    }
}

impl Into<Key> for (GamepadButton, Option<u32>) {
    fn into(self) -> Key {
        Key::GamepadButton(self)
    }
}

impl Into<Key> for GamepadButton {
    fn into(self) -> Key {
        Key::GamepadButton((self, None))
    }
}

/// A button is a combination of keys that can be pressed to activate it or deactivate it.
/// This is essential for custom keybindings or for games that want to support multiple control schemes.
pub struct Button {
    /// if a single positive key is pressed, the button is pressed unless a negative key is also pressed.
    positive_keys: Vec<Key>,
    /// if a single negative key is pressed, the button is not pressed.
    negative_keys: Vec<Key>,
}

impl Button {
    pub fn new(positive_keys: Vec<Key>, negative_keys: Vec<Key>) -> Self {
        Self {
            positive_keys,
            negative_keys,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u32),
}

/// all of the info neccessary set from the gamepad.
struct GamepadInputInfo {
    gamepad_axes: FxHashMap<u32, [f32; 2]>,
    gamepad_states: FxHashMap<GamepadButton, bool>,
    last_gamepad_states: FxHashMap<GamepadButton, bool>,
    buttons_awaiting_release: FxHashSet<GamepadButton>,
}

impl GamepadInputInfo {
    fn new() -> Self {
        Self {
            gamepad_axes: FxHashMap::default(),
            gamepad_states: FxHashMap::default(),
            last_gamepad_states: FxHashMap::default(),
            buttons_awaiting_release: FxHashSet::default(),
        }
    }

    pub fn set_gamepad_button_down(&mut self, button: GamepadButton) {
        self.gamepad_states.insert(button, true);
    }

    pub fn clear_gamepad_states(&mut self) {
        self.last_gamepad_states = self.gamepad_states.clone();
        self.gamepad_states.clear();
    }

    pub fn get_gamepad_state(&self, button: GamepadButton) -> KeyState {
        let last = self
            .last_gamepad_states
            .get(&button)
            .copied()
            .unwrap_or(false);
        let current = self.gamepad_states.get(&button).copied().unwrap_or(false);
        if last {
            if current {
                KeyState::Held
            } else {
                KeyState::Released
            }
        } else {
            if current {
                KeyState::Pressed
            } else {
                KeyState::NotPressed
            }
        }
    }

    /// gets the axis of a gamepad. The axis is a value between -1 and 1.
    /// Assumes you want the first gamepad, otherwise you can use get_gamepad_axis_with_id.
    /// returns [0.0, 0.0] if no gamepad is found.
    pub fn get_gamepad_axis(&self) -> [f32; 2] {
        self.gamepad_axes.get(&0).copied().unwrap_or([0.0, 0.0])
    }

    pub fn set_gamepad_axis(&mut self, id: u32, axis: [f32; 2]) {
        self.gamepad_axes.insert(id, axis);
    }

    /// gets the axis of a gamepad. The axis is a value between -1 and 1.
    /// returns [0.0, 0.0] if the gamepad is not found.
    pub fn get_gamepad_axis_with_id(&self, id: u32) -> [f32; 2] {
        self.gamepad_axes.get(&id).copied().unwrap_or([0.0, 0.0])
    }
}

pub struct Input {
    last_key_states: FxHashMap<KeyCode, bool>,
    key_states: FxHashMap<KeyCode, bool>,
    last_mouse_states: FxHashMap<MouseButton, bool>,

    mouse_states: FxHashMap<MouseButton, bool>,
    mouse_position: [f32; 2],
    mouse_wheel: f32,

    gamepad_infos: FxHashMap<u32, GamepadInputInfo>,
    last_active_gamepad: u32,

    buttons: FxHashMap<String, Button>,
    // gamepad input handling
    gilrs: Gilrs,
}

impl Input {
    pub fn new() -> Self {
        Self {
            last_key_states: FxHashMap::default(),
            key_states: FxHashMap::default(),
            last_mouse_states: FxHashMap::default(),
            mouse_states: FxHashMap::default(),
            mouse_position: [0.0, 0.0],
            mouse_wheel: 0.0,
            gamepad_infos: FxHashMap::default(),
            last_active_gamepad: 0,
            buttons: FxHashMap::default(),
            gilrs: Gilrs::new().unwrap(),
        }
    }

    pub fn get_key_state(&self, key: KeyCode) -> KeyState {
        let last = self.last_key_states.get(&key).copied().unwrap_or(false);
        let current = self.key_states.get(&key).copied().unwrap_or(false);
        if last {
            if current {
                KeyState::Held
            } else {
                KeyState::Released
            }
        } else {
            if current {
                KeyState::Pressed
            } else {
                KeyState::NotPressed
            }
        }
    }

    pub fn get_mouse_position(&self) -> [f32; 2] {
        self.mouse_position
    }

    /// sets the mouse wheel value. Unless you are implementing a rendering system, don't call this.
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = [x, y];
    }

    pub fn get_mouse_wheel(&self) -> f32 {
        self.mouse_wheel
    }

    /// sets the mouse wheel value. Unless you are implementing a rendering system, don't call this.
    pub fn set_mouse_wheel(&mut self, wheel: f32) {
        self.mouse_wheel = wheel;
    }

    /// sets the key state of a key. Unless you are implementing a rendering system, don't call this.
    pub fn set_key_down(&mut self, key: KeyCode) {
        self.key_states.insert(key, true);
    }

    /// Moves all current key states to previous key states. Unless you are implementing a rendering system, don't call this.
    /// if you are implementing a rendering system, call this before calling set_key_state.
    pub fn clear_key_states(&mut self) {
        self.last_key_states = self.key_states.clone();
        self.key_states.clear();
    }

    pub fn set_mouse_down(&mut self, button: MouseButton) {
        self.mouse_states.insert(button, true);
    }

    pub fn clear_mouse_states(&mut self) {
        self.last_mouse_states = self.mouse_states.clone();
        self.mouse_states.clear();
    }

    pub fn get_mouse_state(&self, button: MouseButton) -> KeyState {
        let last = self
            .last_mouse_states
            .get(&button)
            .copied()
            .unwrap_or(false);
        let current = self.mouse_states.get(&button).copied().unwrap_or(false);
        if last {
            if current {
                KeyState::Held
            } else {
                KeyState::Released
            }
        } else {
            if current {
                KeyState::Pressed
            } else {
                KeyState::NotPressed
            }
        }
    }

    pub fn add_button(&mut self, name: &str, button: Button) {
        self.buttons.insert(name.to_string(), button);
    }

    pub fn get_button_state(&self, name: &str) -> KeyState {
        let button = self
            .buttons
            .get(name)
            .expect(format!("Button {} not found", name).as_str());

        let positive_keystate = self.find_highest_state(&button.positive_keys);

        let negative_keystate = self.find_highest_state(&button.negative_keys);
        let negative =
            negative_keystate == KeyState::Pressed || negative_keystate == KeyState::Held;

        if positive_keystate == KeyState::Held
            || positive_keystate == KeyState::Pressed
            || positive_keystate == KeyState::Released
        {
            if positive_keystate == KeyState::Released || negative_keystate == KeyState::Pressed {
                return KeyState::Released; // doesn't matter if negative keys are pressed, if a positive key is released, the button is released.
            } else if negative {
                return KeyState::NotPressed; // if a negative key is pressed, the button is not pressed.
            } else {
                return KeyState::Pressed; // if a positive key is pressed, the button is pressed.
            }
        } else {
            KeyState::NotPressed
        }
    }

    fn find_highest_state(&self, keys: &[Key]) -> KeyState {
        let mut highest = KeyState::NotPressed;
        for key in keys {
            match key {
                Key::KeyCode(key) => {
                    let keystate = self.get_key_state(*key);

                    if (keystate == KeyState::Pressed
                        || keystate == KeyState::Held
                        || keystate == KeyState::Released)
                        && keystate > highest
                    {
                        highest = keystate;
                    }
                }
                Key::MouseButton(button) => {
                    let keystate = self.get_mouse_state(*button);

                    if (keystate == KeyState::Pressed
                        || keystate == KeyState::Held
                        || keystate == KeyState::Released)
                        && keystate > highest
                    {
                        highest = keystate;
                    }
                }
                Key::GamepadButton((button, id)) => {
                    let id = id.unwrap_or(self.last_active_gamepad);

                    let keystate = self
                        .gamepad_infos
                        .get(&id)
                        .map(|info| info.get_gamepad_state(*button))
                        .unwrap_or(KeyState::NotPressed);

                    if (keystate == KeyState::Pressed
                        || keystate == KeyState::Held
                        || keystate == KeyState::Released)
                        && keystate > highest
                    {
                        highest = keystate;
                    }
                }
            }
        }

        highest
    }

    /// gets the axis of a gamepad. The axis is a value between -1 and 1.
    /// Assumes you want the first gamepad, otherwise you can use get_gamepad_axis_with_id.
    /// returns [0.0, 0.0] if no gamepad is found.
    pub fn get_gamepad_axis(&self) -> [f32; 2] {
        self.gamepad_infos
            .get(&0)
            .map(|info| info.get_gamepad_axis())
            .unwrap_or([0.0, 0.0])
    }

    pub fn set_gamepad_axis(&mut self, gamepad_id: u32, axis_id: u32, axis: [f32; 2]) {
        self.gamepad_infos
            .entry(gamepad_id)
            .or_insert_with(GamepadInputInfo::new)
            .set_gamepad_axis(axis_id, axis);
    }

    pub fn set_gamepad_button_down(&mut self, gamepad_id: u32, button: GamepadButton) {
        self.gamepad_infos
            .entry(gamepad_id)
            .or_insert_with(GamepadInputInfo::new)
            .set_gamepad_button_down(button);
    }

    /// gets the axis of a gamepad. The axis is a value between -1 and 1.
    /// returns [0.0, 0.0] if the gamepad is not found.
    pub fn get_gamepad_axis_with_id(&self, id: u32) -> [f32; 2] {
        self.gamepad_infos
            .get(&id)
            .map(|info| info.get_gamepad_axis_with_id(id))
            .unwrap_or([0.0, 0.0])
    }

    fn gilrs_button_to_gamepad_button(button: gilrs::Button) -> Option<GamepadButton> {
        match button {
            gilrs::Button::South => Some(GamepadButton::South),
            gilrs::Button::East => Some(GamepadButton::East),
            gilrs::Button::North => Some(GamepadButton::North),
            gilrs::Button::West => Some(GamepadButton::West),
            gilrs::Button::C => Some(GamepadButton::RightShoulder),
            gilrs::Button::Z => Some(GamepadButton::LeftShoulder),
            gilrs::Button::LeftTrigger => Some(GamepadButton::LeftBumper),
            gilrs::Button::RightTrigger => Some(GamepadButton::RightBumper),
            gilrs::Button::LeftTrigger2 => Some(GamepadButton::LeftTrigger),
            gilrs::Button::RightTrigger2 => Some(GamepadButton::RightTrigger),
            gilrs::Button::LeftThumb => Some(GamepadButton::LeftStick),
            gilrs::Button::RightThumb => Some(GamepadButton::RightStick),
            gilrs::Button::Select => Some(GamepadButton::Select),
            gilrs::Button::Start => Some(GamepadButton::Start),
            gilrs::Button::DPadUp => Some(GamepadButton::DPadUp),
            gilrs::Button::DPadDown => Some(GamepadButton::DPadDown),
            gilrs::Button::DPadLeft => Some(GamepadButton::DPadLeft),
            gilrs::Button::DPadRight => Some(GamepadButton::DPadRight),
            _ => None,
        }
    }
}

impl Resource for Input {
    fn update(&mut self) {
        // handle gamepad input
        for info in self.gamepad_infos.values_mut() {
            info.clear_gamepad_states();
        }

        while let Some(Event { id, event, .. }) = self.gilrs.next_event() {
            let id = usize::from(id) as u32;
            println!("{:?}", event);

            match event {
                gilrs::EventType::ButtonRepeated(button, _) => {
                    self.last_active_gamepad = id;
                    let button = Self::gilrs_button_to_gamepad_button(button);
                    if let Some(button) = button {
                        self.set_gamepad_button_down(id, button);
                    }
                }
                gilrs::EventType::ButtonPressed(button, _) => {
                    self.last_active_gamepad = id;
                    let button = Self::gilrs_button_to_gamepad_button(button);

                    if let Some(button) = button {
                        self.set_gamepad_button_down(id, button);
                        self.gamepad_infos
                            .get_mut(&id)
                            .unwrap()
                            .buttons_awaiting_release
                            .insert(button);
                    }
                }
                gilrs::EventType::ButtonReleased(button, _) => {
                    self.last_active_gamepad = id;
                    let button = Self::gilrs_button_to_gamepad_button(button);

                    if let Some(button) = button {
                        self.gamepad_infos
                            .get_mut(&id)
                            .unwrap()
                            .buttons_awaiting_release
                            .remove(&button);
                    }
                }
                _ => {}
            }
        }

        let mut axis_data = vec![];

        for (id, gamepad) in self.gilrs.gamepads() {
            let id = usize::from(id) as u32;

            let left_stick_x = gamepad.value(gilrs::Axis::LeftStickX);
            let left_stick_y = gamepad.value(gilrs::Axis::LeftStickY);

            let left_stick = [left_stick_x, left_stick_y];

            let right_stick_x = gamepad.value(gilrs::Axis::RightStickX);
            let right_stick_y = gamepad.value(gilrs::Axis::RightStickY);

            let right_stick = [right_stick_x, right_stick_y];

            // has to be done for borrow checker
            axis_data.push((id, left_stick, right_stick));
        }

        for (id, left_stick, right_stick) in axis_data {
            self.set_gamepad_axis(id, 0, left_stick);
            self.set_gamepad_axis(id, 1, right_stick);
        }

        let mut all_buttons_awating_release = vec![];

        for (id, gamepad) in self.gamepad_infos.iter() {
            for button in gamepad.buttons_awaiting_release.iter() {
                all_buttons_awating_release.push((*id, *button));
            }
        }

        for (id, button) in all_buttons_awating_release {
            self.set_gamepad_button_down(id, button)
        }

        self.gilrs.inc();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
