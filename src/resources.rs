pub mod input;
use crate::Scene;
pub use input::*;
pub mod delta_time;
pub use delta_time::*;
pub mod audio_stream;
pub use audio_stream::*;
use ABC_ECS::World;

pub(crate) fn add_default_resources(scene: &mut Scene) {
    scene
        .world
        .entities_and_components
        .add_resource(Input::new());

    scene
        .world
        .entities_and_components
        .add_resource(DeltaTime::new());

    scene
        .world
        .entities_and_components
        .add_resource(AudioHandle::new());
}

pub fn remove_all_non_internal_systems(scene: &mut World) {
    scene.remove_all_systems();
    crate::physics::add_default_physics_systems(scene);
    scene.add_system(crate::resources::InputUpdateSystem::new());
}
