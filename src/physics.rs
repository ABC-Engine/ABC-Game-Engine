use crate::Scene;

pub mod colliders;
pub mod rigidbody;

pub fn add_default_physics_systems(scene: &mut Scene) {
    scene
        .game_engine
        .add_system(Box::new(rigidbody::RigidBodyDynamicsSystem {}));
    scene
        .game_engine
        .add_system(Box::new(colliders::CollisionSystem {}));
}
