use crate::delta_time;
use fxhash::{FxHashMap, FxHashSet};
use gilrs::{Event, Gilrs};
use ABC_ECS::{EntitiesAndComponents, Resource};

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

/// The buttons on a gamepad.
/// (note: if there is something wrong with this list or if something is missing, please let me know.)
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

pub enum AxisEventDirection {
    /// left or down
    Negative,
    /// right or up
    Positive,
}

/// An axis event is an event that is triggered when an axis passes a certain threshold.
pub struct AxisEvent {
    direction: AxisEventDirection,
    axis: String,
    // how much the axis has to be pressed to be considered pressed.
    // this is a bit of a confusing name, because it's the same as the deadzone for gamepad axes.
    // but it's the same concept except not analog.
    // always positive
    threshold: f32,
}

impl AxisEvent {
    /// Creates a new axis event.
    pub fn new(direction: AxisEventDirection, axis: String, threshold: f32) -> Self {
        Self {
            direction,
            axis,
            threshold: threshold.abs(),
        }
    }
}

/// For either a key or a mouse button.
/// (note: This naming seems a bit off, if you have a better name, please suggest it.)
pub enum Key {
    KeyCode(KeyCode),
    MouseButton(MouseButton),
    GamepadButton((GamepadButton, Option<u32>)),
    AxisEvent(AxisEvent),
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

impl Into<Key> for AxisEvent {
    fn into(self) -> Key {
        Key::AxisEvent(self)
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
    /// Creates a new button.
    /// positive_keys are keys that when pressed, the button is pressed.
    /// negative_keys are keys that when pressed, the button is not pressed, even if a positive key is pressed.
    pub fn new(positive_keys: Vec<Key>, negative_keys: Vec<Key>) -> Self {
        Self {
            positive_keys,
            negative_keys,
        }
    }
}

/// The direction of an axis.
/// X is left and right, Y is up and down.
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

/// A gamepad axis specifies a specific axis on a specific gamepad.
/// if the gamepad is not specified, the last gamepad is used.
#[derive(Debug, Clone)]
pub struct GamepadAxis {
    // if none, the last gamepad is used.
    id: Option<u32>,
    axis: u32,
    direction: AxisDirection,
    deadzone: f32,
}

impl GamepadAxis {
    /// Creates a new gamepad axis.
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
    presvious_value: f32,
}

impl Axis {
    /// Creates a new axis.
    pub fn new(positive_keys: Vec<Key>, negative_keys: Vec<Key>, axes: Vec<GamepadAxis>) -> Self {
        Self {
            positive_keys,
            negative_keys,
            axes,
            gravity: None,
            sensitivity: None,
            value: 0.0,
            presvious_value: 0.0,
        }
    }

    /// Creates a new axis with a gravity value.
    pub fn with_gravity(mut self, gravity: f32) -> Self {
        self.gravity = Some(gravity);
        self
    }

    /// Creates a new axis with a sensitivity value.
    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = Some(sensitivity);
        self
    }

    /// Creates a new axis with certain axes.
    pub fn with_axes(mut self, axes: Vec<GamepadAxis>) -> Self {
        self.axes = axes;
        self
    }

    /// Creates a new axis with certain positive keys.
    pub fn with_positive_keys(mut self, positive_keys: Vec<Key>) -> Self {
        self.positive_keys = positive_keys;
        self
    }

    /// Creates a new axis with certain negative keys.
    pub fn with_negative_keys(mut self, negative_keys: Vec<Key>) -> Self {
        self.negative_keys = negative_keys;
        self
    }

    /// Creates a new axis with a certain value.
    pub fn set_axis(&mut self, axes: Vec<GamepadAxis>) {
        self.axes = axes;
    }

    /// sets the positive keys of the axis.
    pub fn set_positive_keys(&mut self, positive_keys: Vec<Key>) {
        self.positive_keys = positive_keys;
    }

    /// sets the negative keys of the axis.
    pub fn set_negative_keys(&mut self, negative_keys: Vec<Key>) {
        self.negative_keys = negative_keys;
    }

    /// sets the gravity of the axis.
    pub fn set_gravity(&mut self, gravity: f32) {
        self.gravity = Some(gravity);
    }

    /// sets the sensitivity of the axis.
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = Some(sensitivity);
    }

    /// gets the value of the axis.
    pub fn update_value_from_raw(&mut self, raw_value: f32, delta_time: f32) {
        self.presvious_value = self.value;

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
    }
}

/// The mouse buttons.
/// Other can be any other mouse button that isn't left, right or middle.
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

    /// sets a gamepad button as down
    pub fn set_gamepad_button_down(&mut self, button: GamepadButton) {
        self.gamepad_states.insert(button, true);
    }

    /// clears all gamepad states.
    pub fn clear_gamepad_states(&mut self) {
        self.last_gamepad_states = self.gamepad_states.clone();
        self.gamepad_states.clear();
    }

    /// gets the state of a gamepad button.
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

    /// sets the axis of a gamepad. The axis is a value between -1 and 1.
    pub fn set_gamepad_axis(&mut self, id: u32, axis: [f32; 2]) {
        self.gamepad_axes.insert(id, axis);
    }

    /// gets the axis of a gamepad. The axis is a value between -1 and 1.
    /// returns [0.0, 0.0] if the gamepad is not found.
    pub fn get_gamepad_axis_with_id(&self, id: u32) -> [f32; 2] {
        self.gamepad_axes.get(&id).copied().unwrap_or([0.0, 0.0])
    }
}

/// The input resource.
/// This is the resource that is used to get input from the user.
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
    /// Creates a new input resource.
    /// mostly don't need to call this, as the engine will create it for you.
    ///
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

    /// gets the state of a key.
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

    /// gets the position of the mouse.
    pub fn get_mouse_position(&self) -> [f32; 2] {
        self.mouse_position
    }

    /// sets the mouse wheel value. Unless you are implementing a rendering system, don't call this.
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = [x, y];
    }

    /// gets the mouse wheel value.
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
    /// also keep in mind this will not clear the mouse states, that is done by clear_mouse_states.
    pub fn clear_key_states(&mut self) {
        self.last_key_states = self.key_states.clone();
        self.key_states.clear();
    }

    /// sets the mouse state of a mouse button. Unless you are implementing a rendering system, don't call this.
    pub fn set_mouse_down(&mut self, button: MouseButton) {
        self.mouse_states.insert(button, true);
    }

    /// Moves all current mouse states to previous mouse states. Unless you are implementing a rendering system, don't call this.
    pub fn clear_mouse_states(&mut self) {
        self.last_mouse_states = self.mouse_states.clone();
        self.mouse_states.clear();
    }

    /// gets the state of a mouse button.
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

    /// Adds a button to the input resource.
    pub fn add_button(&mut self, name: &str, button: Button) {
        self.buttons.insert(name.to_string(), button);
    }

    /// gets the state of a button.
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

    /// finds the highest state of a list of keys.
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
                Key::AxisEvent(axis_event) => {
                    let axis = self.get_axis(&axis_event.axis);

                    let is_active = match axis_event.direction {
                        AxisEventDirection::Negative => axis < -axis_event.threshold,
                        AxisEventDirection::Positive => axis > axis_event.threshold,
                    };

                    // this isn't the greatest but currently there is no way to find the delta of the axis.
                    // which would be neccessary to determine if the axis was pressed or held or released.
                    /*if is_active && KeyState::Held > highest {
                        highest = KeyState::Held;
                    }*/

                    let previous_axis = self.get_previous_axis(&axis_event.axis);
                    let was_active = match axis_event.direction {
                        AxisEventDirection::Negative => previous_axis < -axis_event.threshold,
                        AxisEventDirection::Positive => previous_axis > axis_event.threshold,
                    };

                    let mut current_state = KeyState::NotPressed;
                    if was_active && !is_active {
                        current_state = KeyState::Released;
                    } else if is_active && !was_active {
                        current_state = KeyState::Pressed;
                    } else if is_active {
                        current_state = KeyState::Held;
                    }

                    if current_state > highest {
                        highest = current_state;
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

    /// sets the axis of a gamepad. The axis is a value between -1 and 1.
    pub fn set_gamepad_axis(&mut self, gamepad_id: u32, axis_id: u32, axis: [f32; 2]) {
        self.gamepad_infos
            .entry(gamepad_id)
            .or_insert_with(GamepadInputInfo::new)
            .set_gamepad_axis(axis_id, axis);
    }

    /// sets the state of a gamepad button. unless you are implementing a rendering system, don't call this.
    pub fn set_gamepad_button_down(&mut self, gamepad_id: u32, button: GamepadButton) {
        self.gamepad_infos
            .entry(gamepad_id)
            .or_insert_with(GamepadInputInfo::new)
            .set_gamepad_button_down(button);
    }

    /// gets the state of a gamepad button.
    pub fn get_gamepad_state(&self, gamepad_id: u32, button: GamepadButton) -> KeyState {
        self.gamepad_infos
            .get(&gamepad_id)
            .map(|info| info.get_gamepad_state(button))
            .unwrap_or(KeyState::NotPressed)
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

    /// Adds an axis to the input resource.
    /// The axis can be accessed by name by calling get_axis.
    pub fn add_axis(&mut self, name: &str, axis: Axis) {
        self.axes.insert(name.to_string(), axis);
    }

    /// gets the raw value of an axis.
    /// This is the value of the axis before gravity and sensitivity are applied.
    /// Just as received from the controller, with no modifications.
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

    /// gets the value of an axis.
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

    /// gets the previous value of an axis.
    fn get_previous_axis(&self, name: &str) -> f32 {
        let axis = self.axes.get(name).expect(
            format!(
                "Axis {} not found... did you ever add it? the list of available axes are: {:?}",
                name,
                self.axes.keys()
            )
            .as_str(),
        );

        axis.presvious_value
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
    /// Creates a new input update system.
    pub(crate) fn new() -> Self {
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

#[cfg(test)]
mod tests {
    use crate::Scene;

    use super::*;
    use ABC_ECS::System;

    #[test]
    fn test_button() {
        let button = Button::new(
            vec![Key::KeyCode(KeyCode::A)],
            vec![Key::KeyCode(KeyCode::B)],
        );

        assert_eq!(button.positive_keys.len(), 1);
        assert_eq!(button.negative_keys.len(), 1);
    }

    #[test]
    fn test_key_state() {
        let key_state = KeyState::Pressed;

        assert!(key_state > KeyState::NotPressed);
        assert!(key_state < KeyState::Held);
        assert!(key_state < KeyState::Released);
    }

    #[test]
    fn test_axis() {
        let axis = Axis::new(
            vec![Key::KeyCode(KeyCode::A)],
            vec![Key::KeyCode(KeyCode::B)],
            vec![GamepadAxis::new(None, 0, AxisDirection::X, 0.5)],
        );

        assert_eq!(axis.positive_keys.len(), 1);
        assert_eq!(axis.negative_keys.len(), 1);
        assert_eq!(axis.axes.len(), 1);
    }

    #[test]
    fn test_gamepad_button() {
        let button: Key = GamepadButton::South.into();

        match button {
            Key::GamepadButton((GamepadButton::South, None)) => {}
            _ => panic!("Key not GamepadButton::South"),
        }
    }

    #[test]
    fn test_gamepad_axis() {
        let axis = GamepadAxis::new(None, 0, AxisDirection::X, 0.5);

        assert_eq!(axis.axis, 0);
    }

    #[test]
    fn test_releasing() {
        let mut input = Input::new();

        input.set_key_down(KeyCode::A);
        input.set_mouse_down(MouseButton::Left);

        input.clear_key_states();
        input.clear_mouse_states();

        assert_eq!(input.get_key_state(KeyCode::A), KeyState::Released);
        assert_eq!(input.get_mouse_state(MouseButton::Left), KeyState::Released);
    }

    #[test]
    fn test_pressed() {
        let mut input = Input::new();

        input.set_key_down(KeyCode::A);
        input.set_mouse_down(MouseButton::Left);

        assert_eq!(input.get_key_state(KeyCode::A), KeyState::Pressed);
        assert_eq!(input.get_mouse_state(MouseButton::Left), KeyState::Pressed);
    }

    #[test]
    fn test_held() {
        let mut input = Input::new();

        input.set_key_down(KeyCode::A);
        input.set_mouse_down(MouseButton::Left);

        input.clear_key_states();
        input.clear_mouse_states();

        input.set_key_down(KeyCode::A);
        input.set_mouse_down(MouseButton::Left);

        assert_eq!(input.get_key_state(KeyCode::A), KeyState::Held);
        assert_eq!(input.get_mouse_state(MouseButton::Left), KeyState::Held);
    }

    #[test]
    fn test_button_state() {
        let mut input = Input::new();

        input.add_button(
            "test",
            Button::new(
                vec![Key::KeyCode(KeyCode::A)],
                vec![Key::KeyCode(KeyCode::B)],
            ),
        );

        input.set_key_down(KeyCode::A);

        assert_eq!(input.get_button_state("test"), KeyState::Pressed);

        input.clear_key_states();

        assert_eq!(input.get_button_state("test"), KeyState::Released);
    }

    #[test]
    fn test_axis_value() {
        let mut input = Input::new();

        input.add_axis(
            "test",
            Axis::new(
                vec![Key::KeyCode(KeyCode::A)],
                vec![Key::KeyCode(KeyCode::B)],
                vec![GamepadAxis::new(None, 0, AxisDirection::X, 0.5)],
            ),
        );

        let mut scene = Scene::new();

        let mut input_system = InputUpdateSystem::new();

        let entities_and_components = &mut scene.world.entities_and_components;

        entities_and_components.add_resource(input);

        input_system.run(entities_and_components);

        let input = entities_and_components
            .get_resource_mut::<Input>()
            .expect("Input not found");

        assert_eq!(input.get_axis("test"), 0.0);

        input.set_key_down(KeyCode::A);

        input_system.run(entities_and_components);

        let input = entities_and_components
            .get_resource_mut::<Input>()
            .expect("Input not found");

        assert_eq!(input.get_axis("test"), 1.0);
    }

    #[test]
    fn test_axis_event() {
        let mut input = Input::new();

        input.add_axis(
            "test",
            Axis::new(vec![Key::KeyCode(KeyCode::A)], vec![], vec![]),
        );

        let axis_related_button = Button::new(
            vec![Key::AxisEvent(AxisEvent {
                direction: AxisEventDirection::Positive,
                axis: "test".to_string(),
                threshold: 0.5,
            })],
            vec![],
        );

        input.add_button("axis_related_button", axis_related_button);

        let mut scene = Scene::new();

        let mut input_system = InputUpdateSystem::new();

        let entities_and_components = &mut scene.world.entities_and_components;

        entities_and_components.add_resource(input);

        input_system.run(entities_and_components);

        let input = entities_and_components
            .get_resource_mut::<Input>()
            .expect("Input not found");

        assert_eq!(
            input.get_button_state("axis_related_button"),
            KeyState::NotPressed
        );

        input.set_key_down(KeyCode::A);

        input_system.run(entities_and_components);

        let input = entities_and_components
            .get_resource_mut::<Input>()
            .expect("Input not found");

        assert_eq!(
            input.get_button_state("axis_related_button"),
            KeyState::Pressed
        );
    }
}
