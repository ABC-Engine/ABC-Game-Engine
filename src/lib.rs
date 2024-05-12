//#![doc = include_str!("../README.md")]

mod resources;
mod test;

pub use resources::*;
pub use resources::{delta_time, input};
pub use ABC_ECS::{Component, EntitiesAndComponents, Entity, Resource, System, World};
pub mod physics;

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

/// Scene is responsible for holding all objects and the background color
pub struct Scene {
    pub world: World,
}

impl Scene {
    pub fn new() -> Scene {
        let mut scene = Scene {
            world: World::new(),
        };

        add_default_resources(&mut scene);

        scene
    }
}
