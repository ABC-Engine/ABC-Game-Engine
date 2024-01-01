//#![doc = include_str!("../README.md")]

mod resources;
mod shape_renderer;
mod test;
pub use shape_renderer::*;
mod load_texture;
pub use crossterm::event::KeyCode;
pub use load_texture::*;
pub use resources::*;
mod renderer;
pub use renderer::*;
pub use resources::{delta_time, input};
pub use ABC_ECS::{Component, EntitiesAndComponents, Entity, GameEngine, System};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 1.0,
        }
    }
}

/// Transform is a struct that holds the position, rotation, and scale of an object
#[derive(Clone, Copy)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub scale: f32,
    /// origin relative to the position of the object
    pub origin_x: f32,
    /// origin relative to the position of the object
    pub origin_y: f32,
}

impl Transform {
    pub fn default() -> Transform {
        Transform {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        }
    }
}

/// a is the parent
impl<'a, 'b> std::ops::Add<&'b Transform> for &'a Transform {
    type Output = Transform;

    fn add(self, other: &'b Transform) -> Transform {
        Transform {
            x: self.x + other.x,
            y: self.y + other.y,
            rotation: self.rotation + other.rotation,
            scale: self.scale * other.scale,
            origin_x: self.origin_x - other.x as f32,
            origin_y: self.origin_y - other.y as f32,
        }
    }
}

/// SceneParams is a struct that holds the background color, if the characters are random, and the character that will be displayed otherwise
pub struct SceneParams {
    background_color: Color,
    is_random_chars: bool,
    character: char,
}

impl SceneParams {
    pub fn new() -> SceneParams {
        SceneParams {
            background_color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 1.0,
            },
            is_random_chars: false,
            character: '=',
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_random_chars(&mut self, is_random_chars: bool) {
        self.is_random_chars = is_random_chars;
    }

    pub fn set_character(&mut self, character: char) {
        self.character = character;
    }
}

/// Scene is responsible for holding all objects and the background color
pub struct Scene {
    pub game_engine: GameEngine,
    pub scene_params: SceneParams,
}

impl Scene {
    pub fn new() -> Scene {
        let mut scene = Scene {
            game_engine: GameEngine::new(),
            scene_params: SceneParams {
                background_color: Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 1.0,
                },
                is_random_chars: false,
                character: '=',
            },
        };

        add_default_resources(&mut scene);

        scene
    }
}
