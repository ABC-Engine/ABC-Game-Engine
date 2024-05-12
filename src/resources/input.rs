use fxhash::FxHashMap;
use ABC_ECS::Resource;

/// Stolen directly from winit, this is a list of all the keycodes that can be pressed on a keyboard.
/// I copy pasted this because I don't want to depend on winit in this crate.
/// The keycodes might also change in the future, so it's better to have a copy of them here.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

pub struct Input {
    last_key_states: FxHashMap<KeyCode, bool>,
    /// KeyPressed is sent the frame that a key is pressed.
    key_states: FxHashMap<KeyCode, bool>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            last_key_states: FxHashMap::default(),
            key_states: FxHashMap::default(),
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
}

impl Resource for Input {
    fn update(&mut self) {
        //self.update();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
