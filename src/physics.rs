use crate::Scene;

pub mod colliders;
pub mod rigidbody;

pub fn add_default_physics_systems(scene: &mut Scene) {
    scene
        .world
        .add_system(rigidbody::RigidBodyDynamicsSystem {});
    scene.world.add_system(colliders::CollisionSystem {});
}
