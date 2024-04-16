//#![doc = include_str!("../README.md")]

mod resources;
mod shape_renderer;
mod test;
pub use shape_renderer::*;
mod load_texture;
pub use crossterm::event::KeyCode;
pub use load_texture::*;
use renderer::*;
pub use resources::*;
pub use resources::{delta_time, input};
pub use ABC_ECS::{Component, EntitiesAndComponents, Entity, System, World};
pub mod camera;
pub mod physics;
pub mod renderer;

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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    /// z is used for depth sorting, the actual value of z does not matter, only the relative values of z between objects
    pub z: f64,
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
            z: 0.0,
            rotation: 0.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        }
    }

    /// returns the distance between two transforms
    pub fn distance_to(&self, other: &Transform) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        (x.powi(2) + y.powi(2)).sqrt()
    }

    /// squared distance is faster than distance, but it is not the actual distance
    /// this is useful for comparing distances, but not for getting the actual distance
    pub fn squared_distance_to(&self, other: &Transform) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        x.powi(2) + y.powi(2)
    }
}

/// a is the parent
impl<'a, 'b> std::ops::Add<&'b Transform> for &'a Transform {
    type Output = Transform;

    fn add(self, other: &'b Transform) -> Transform {
        Transform {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
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

    /// the background color of the scene
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// if the characters are random, or if they are all the same character as specified by the set_character function
    pub fn set_random_chars(&mut self, is_random_chars: bool) {
        self.is_random_chars = is_random_chars;
    }

    /// the character that will be displayed if is_random_chars is false
    pub fn set_character(&mut self, character: char) {
        self.character = character;
    }
}

/// Scene is responsible for holding all objects and the background color
pub struct Scene {
    pub world: World,
    pub scene_params: SceneParams,
}

impl Scene {
    pub fn new() -> Scene {
        let mut scene = Scene {
            world: World::new(),
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
