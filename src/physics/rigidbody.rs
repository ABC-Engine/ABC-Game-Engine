use crate::*;
pub use glam::Vec2;

pub struct RigidBodyDynamicsSystem;

impl System for RigidBodyDynamicsSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let delta_time: f64;
        {
            delta_time = entities_and_components
                .get_resource::<DeltaTime>()
                .expect("Failed to get DeltaTime resource")
                .delta_time;
        }

        let entities_with_rigid_body = entities_and_components
            .get_entities_with_component::<RigidBody>()
            .cloned()
            .collect::<Vec<Entity>>();

        for rigid_body_entity in entities_with_rigid_body {
            if let (Some(rigid_body), Some(transform)) = entities_and_components
                .try_get_components_mut::<(RigidBody, Transform)>(rigid_body_entity)
            {
                rigid_body.acceleration.y += rigid_body.gravity * delta_time as f32;
                rigid_body.velocity += rigid_body.acceleration;
                rigid_body.acceleration = Vec2::ZERO;

                transform.x += rigid_body.velocity.x as f64 * delta_time;
                transform.y += rigid_body.velocity.y as f64 * delta_time;
            }
        }
    }
}

pub struct RigidBody {
    mass: f32,
    velocity: Vec2,
    gravity: f32,
    acceleration: Vec2,
    terminal_velocity: Option<f32>,
}

impl RigidBody {
    pub fn new(mass: f32, velocity: Vec2, gravity: f32) -> Self {
        Self {
            mass,
            velocity,
            gravity,
            acceleration: Vec2::ZERO,
            terminal_velocity: None,
        }
    }

    pub fn default() -> Self {
        Self {
            mass: 1.0,
            velocity: Vec2::ZERO,
            gravity: 9.807,
            acceleration: Vec2::ZERO,
            terminal_velocity: None,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        if let Some(terminal_velocity) = self.terminal_velocity {
            // I think this is the correct way to do this, but I'm not sure
            // maybe the direction of the velocity should still be changed,
            // while setting the magnitude to the terminal velocity
            if self.velocity.length() > terminal_velocity {
                return;
            }
        }
        self.acceleration += force / self.mass;
    }

    pub fn set_gravity(&mut self, gravity: f32) {
        self.gravity = gravity;
    }

    pub fn get_gravity(&self) -> f32 {
        self.gravity
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    /// gets the acceleration that will be applied to the rigid body during the next physics update
    pub fn get_acceleration(&self) -> Vec2 {
        self.acceleration
    }
}
