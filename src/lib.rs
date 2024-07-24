//#![doc = include_str!("../README.md")]

mod resources;
mod test;

pub use resources::*;
pub use resources::{delta_time, input};
pub use ABC_ECS::{
    Component, EntitiesAndComponents, EntitiesAndComponentsThreadSafe, Entity, Resource,
    SingleMutEntity, System, World,
};
pub(crate) mod crash_handler;
pub mod physics;
pub mod ui;

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

impl Default for Transform {
    fn default() -> Transform {
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
}

impl Transform {
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

fn rotate_about_origin(x1: f64, y1: f64, x2: f64, y2: f64, rotation: f64) -> (f64, f64) {
    let new_x = x1 + (x2 - x1) * rotation.cos() as f64 - (y2 - y1) * rotation.sin() as f64;
    let new_y = y1 + (x2 - x1) * rotation.sin() as f64 + (y2 - y1) * rotation.cos() as f64;

    (new_x, new_y)
}

/// self is the parent
impl<'a, 'b> std::ops::Add<&'b Transform> for &'a Transform {
    type Output = Transform;

    fn add(self, other: &'b Transform) -> Transform {
        let new_x = self.x + other.x;
        let new_y = self.y + other.y;
        let (new_x, new_y) = rotate_about_origin(self.x, self.y, new_x, new_y, self.rotation);

        let new_transform = Transform {
            x: new_x,
            y: new_y,
            z: self.z + other.z,
            rotation: self.rotation + other.rotation,
            scale: self.scale * other.scale,
            origin_x: self.origin_x - other.origin_x as f32,
            origin_y: self.origin_y - other.origin_y as f32,
        };

        new_transform
    }
}

/// used to "unparent" an object
/// other is the parent
impl<'a, 'b> std::ops::Sub<&'b Transform> for &'a Transform {
    type Output = Transform;

    fn sub(self, other: &'b Transform) -> Transform {
        // rotate opposite direction
        let x1 = other.x;
        let y1 = other.y;
        let x2 = self.x;
        let y2 = self.y;
        let rotation = -other.rotation;

        let (mut new_x, mut new_y) = rotate_about_origin(x1, y1, x2, y2, rotation);
        new_x -= x1;
        new_y -= y1;

        Transform {
            x: new_x,
            y: new_y,
            z: self.z - other.z,
            rotation: self.rotation - other.rotation,
            scale: self.scale / other.scale,
            origin_x: self.origin_x + other.origin_x,
            origin_y: self.origin_y + other.origin_y,
        }
    }
}

fn get_entity_path(
    entity: Entity,
    entities_and_components: &EntitiesAndComponents,
    mut path: Vec<Entity>,
) -> Vec<Entity> {
    path.push(entity);

    let parent = entities_and_components.get_parent(entity);
    if let Some(parent) = parent {
        return get_entity_path(parent, entities_and_components, path);
    } else {
        return path;
    }
}

/// gets the transform of an entity, this will return the total transform of the entity, including the transform(s) of the parent(s)
pub fn get_transform(entity: Entity, entities_and_components: &EntitiesAndComponents) -> Transform {
    // this is neccessary because the parent transform needs to be applied first
    let path = get_entity_path(entity, entities_and_components, Vec::new());
    let mut transform = Transform::default();
    for entity in path.iter().rev() {
        let (transform_component,) =
            entities_and_components.get_components::<(Transform,)>(*entity);
        transform = &transform + &transform_component;
    }

    transform
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
        crash_handler::crash_handler();

        scene
    }

    /// creates a new scene without the crash handler
    /// easier for testing, but make sure to switch to Scene::new() when you are done testing and want to ship the game
    pub fn new_without_crash_handler() -> Scene {
        let mut scene = Scene {
            world: World::new(),
        };

        add_default_resources(&mut scene);

        scene
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_add_sub_cancellation() {
        // test that a - b + b = a

        let a = Transform {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            rotation: 1.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        };

        let b = Transform {
            x: 2.0,
            y: 2.0,
            z: 2.0,
            rotation: 2.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        };

        let b_plus_a = &b + &a;
        let a_plus_b_minus_b = &b_plus_a - &b;

        if a_plus_b_minus_b != a {
            println!("{:?}", a);
            println!("{:?}", b);
            println!("{:?}", b_plus_a);
            println!("{:?}", a_plus_b_minus_b);
            panic!("{:?} != {:?}", a_plus_b_minus_b, a);
        }
    }
}
