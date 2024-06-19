use fxhash::{FxHashMap, FxHashSet};
use gilrs::{Event, Gilrs};
use ABC_ECS::{EntitiesAndComponents, Resource};

use crate::delta_time;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisDirection {
    X, // positive x is right, negative x is left.
    Y, // positive y is up, negative y is down.
}

impl From<AxisDirection> for usize {
    fn from(value: AxisDirection) -> Self {
        match value {
            AxisDirection::X => 0,
            AxisDirection::Y => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GamepadAxis {
    // if none, the last gamepad is used.
    id: Option<u32>,
    axis: u32,
    direction: AxisDirection,
    deadzone: f32,
}

impl GamepadAxis {
    pub fn new(id: Option<u32>, axis: u32, direction: AxisDirection, deadzone: f32) -> Self {
        Self {
            id,
            axis,
            direction,
            deadzone,
        }
    }
}

/// A 1D axis is a series of positive and negative keys that can be pressed to move the axis in either the positive or negative direction.
pub struct Axis {
    positive_keys: Vec<Key>,
    negative_keys: Vec<Key>,
    axes: Vec<GamepadAxis>,
    // the speed at which the axis moves back to 0 when no keys are pressed.
    // if this is None, the axis will be moved back to 0 instantly.
    // the higher the value, the faster the axis moves back to 0.
    gravity: Option<f32>,
    // the speed at which the axis moves when a key is pressed.
    // if this is None, the axis will be moved at a constant speed.
    // the higher the value, the faster the axis moves.
    sensitivity: Option<f32>,
    value: f32,
}

impl Axis {
    pub fn new(positive_keys: Vec<Key>, negative_keys: Vec<Key>, axes: Vec<GamepadAxis>) -> Self {
        Self {
            positive_keys,
            negative_keys,
            axes,
            gravity: None,
            sensitivity: None,
            value: 0.0,
        }
    }

    pub fn with_gravity(mut self, gravity: f32) -> Self {
        self.gravity = Some(gravity);
        self
    }

    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = Some(sensitivity);
        self
    }

    pub fn with_axes(mut self, axes: Vec<GamepadAxis>) -> Self {
        self.axes = axes;
        self
    }

    pub fn with_positive_keys(mut self, positive_keys: Vec<Key>) -> Self {
        self.positive_keys = positive_keys;
        self
    }

    pub fn with_negative_keys(mut self, negative_keys: Vec<Key>) -> Self {
        self.negative_keys = negative_keys;
        self
    }

    pub fn set_axis(&mut self, axes: Vec<GamepadAxis>) {
        self.axes = axes;
    }

    pub fn set_positive_keys(&mut self, positive_keys: Vec<Key>) {
        self.positive_keys = positive_keys;
    }

    pub fn set_negative_keys(&mut self, negative_keys: Vec<Key>) {
        self.negative_keys = negative_keys;
    }

    pub fn set_gravity(&mut self, gravity: f32) {
        self.gravity = Some(gravity);
    }

    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = Some(sensitivity);
    }

    pub fn update_value_from_raw(&mut self, raw_value: f32, delta_time: f32) {
        let difference = raw_value - self.value;

        let multiplier;
        if raw_value.abs() < 0.01 {
            if let Some(gravity) = self.gravity {
                multiplier = gravity;
            } else {
                self.value = raw_value;
                return;
            }
        } else {
            if let Some(sensitivity) = self.sensitivity {
                multiplier = sensitivity;
            } else {
                self.value = raw_value;
                return;
            }
        }

        let dir = difference.signum();

        let amount_to_move = multiplier * delta_time * dir;
        if amount_to_move.abs() > difference.abs() {
            self.value = raw_value;
        } else {
            self.value += amount_to_move;
        }

        /*// kind of a mess, but not sure how to make it cleaner.
        if difference > 0.0 {
            if let Some(sensitivity) = self.sensitivity {
                let amount_to_move = sensitivity * delta_time;
                if amount_to_move > difference {
                    self.value = raw_value;
                } else {
                    self.value += amount_to_move;
                }
            } else {
                self.value = raw_value;
            }
        } else if difference < 0.0 {
            if let Some(gravity) = self.gravity {
                let amount_to_move = -1.0 * gravity * delta_time;

                if amount_to_move < difference {
                    self.value = raw_value;
                } else {
                    self.value += amount_to_move;
                }
            } else {
                self.value = raw_value;
            }
        }*/
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
    axes: FxHashMap<String, Axis>,
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
            axes: FxHashMap::default(),
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

        if positive_keystate != KeyState::NotPressed {
            if positive_keystate == KeyState::Released || negative_keystate == KeyState::Pressed {
                return KeyState::Released; // doesn't matter if negative keys are pressed, if a positive key is released, the button is released.
            } else if negative {
                return KeyState::NotPressed; // if a negative key is pressed, the button is not pressed.
            } else {
                return positive_keystate; // no interference from negative keys.
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
    pub fn get_gamepad_axis_with_id(&self, gamepad_id: u32, axis_id: u32) -> [f32; 2] {
        self.gamepad_infos
            .get(&gamepad_id)
            .map(|info| info.get_gamepad_axis_with_id(axis_id))
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

    pub fn add_axis(&mut self, name: &str, axis: Axis) {
        self.axes.insert(name.to_string(), axis);
    }

    pub fn get_axis_raw(&self, name: &str) -> f32 {
        let axis = self.axes.get(name).expect(
            format!(
                "Axis {} not found... did you ever add it? the list of available axes are: {:?}",
                name,
                self.axes.keys()
            )
            .as_str(),
        );

        let mut is_keyboard_input = false;

        // not entirely necessary, but it should help if someone bumps a keybind for only one frame?
        // + it's more readable and consistent with the button code.
        let positive = self.find_highest_state(&axis.positive_keys);
        let negative = self.find_highest_state(&axis.negative_keys);

        let mut value = 0.0;

        if positive == KeyState::Held || positive == KeyState::Pressed {
            value += 1.0;
            is_keyboard_input = true;
        }

        if negative == KeyState::Held || negative == KeyState::Pressed {
            value -= 1.0;
            is_keyboard_input = true;
        }

        if is_keyboard_input {
            // no need to check gamepad input if keyboard input is found.
            // Note: this prioritizes keyboard input over gamepad input, maybe this should be configurable?
            return value;
        }

        for gamepad_axis in &axis.axes {
            let id = gamepad_axis.id.unwrap_or(self.last_active_gamepad);
            let axis = self.get_gamepad_axis_with_id(id, gamepad_axis.axis)
                [gamepad_axis.direction as usize];
            if axis > gamepad_axis.deadzone || axis < -gamepad_axis.deadzone {
                value += axis;
                return value;
            }
        }

        value
    }

    pub fn get_axis(&self, name: &str) -> f32 {
        let axis = self.axes.get(name).expect(
            format!(
                "Axis {} not found... did you ever add it? the list of available axes are: {:?}",
                name,
                self.axes.keys()
            )
            .as_str(),
        );

        axis.value
    }
}

impl Resource for Input {
    fn update(&mut self) {} // update is handled by the InputUpdateSystem
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub(crate) struct InputUpdateSystem;

impl InputUpdateSystem {
    pub fn new() -> Self {
        Self
    }
}

impl ABC_ECS::System for InputUpdateSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let delta_time = entities_and_components
            .get_resource::<delta_time::DeltaTime>()
            .expect("DeltaTime not found")
            .get_delta_time();

        let input = entities_and_components
            .get_resource_mut::<Input>()
            .expect("Input not found");

        // handle gamepad input
        for info in input.gamepad_infos.values_mut() {
            info.clear_gamepad_states();
        }

        while let Some(Event { id, event, .. }) = input.gilrs.next_event() {
            let id = usize::from(id) as u32;

            match event {
                gilrs::EventType::ButtonRepeated(button, _) => {
                    input.last_active_gamepad = id;
                    let button: Option<GamepadButton> =
                        Input::gilrs_button_to_gamepad_button(button);
                    if let Some(button) = button {
                        input.set_gamepad_button_down(id, button);
                    }
                }
                gilrs::EventType::ButtonPressed(button, _) => {
                    input.last_active_gamepad = id;
                    let button = Input::gilrs_button_to_gamepad_button(button);

                    if let Some(button) = button {
                        input.set_gamepad_button_down(id, button);
                        input
                            .gamepad_infos
                            .get_mut(&id)
                            .unwrap()
                            .buttons_awaiting_release
                            .insert(button);
                    }
                }
                gilrs::EventType::ButtonReleased(button, _) => {
                    input.last_active_gamepad = id;
                    let button = Input::gilrs_button_to_gamepad_button(button);

                    if let Some(button) = button {
                        input
                            .gamepad_infos
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

        for (id, gamepad) in input.gilrs.gamepads() {
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
            input.set_gamepad_axis(id, 0, left_stick);
            input.set_gamepad_axis(id, 1, right_stick);
        }

        let mut all_buttons_awating_release = vec![];

        for (id, gamepad) in input.gamepad_infos.iter() {
            for button in gamepad.buttons_awaiting_release.iter() {
                all_buttons_awating_release.push((*id, *button));
            }
        }

        for (id, button) in all_buttons_awating_release {
            input.set_gamepad_button_down(id, button)
        }

        let all_axis_names = input.axes.keys().cloned().collect::<Vec<String>>();
        for axis_name in all_axis_names {
            let raw_value = input.get_axis_raw(&axis_name);

            let axis = input
                .axes
                .get_mut(&axis_name)
                .expect(format!("Axis {} not found", axis_name).as_str());

            axis.update_value_from_raw(raw_value, delta_time as f32);
        }

        input.gilrs.inc();
    }
}
