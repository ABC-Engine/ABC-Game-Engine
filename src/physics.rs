use crate::Scene;

pub mod physics_system;
pub use rapier2d;

pub fn add_default_physics_systems(scene: &mut Scene) {
    println!("Adding default physics systems");
    scene
        .world
        .add_system(physics_system::RapierPhysicsSystem::new());
}
