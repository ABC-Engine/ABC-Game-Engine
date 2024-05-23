use crate::Scene;

pub mod physics_system;
pub use rapier2d;
use ABC_ECS::World;

pub fn add_default_physics_systems(world: &mut World) {
    println!("Adding default physics systems");
    let physics_system =
        physics_system::RapierPhysicsSystem::new(&mut world.entities_and_components);
    world.add_system(physics_system);
}
