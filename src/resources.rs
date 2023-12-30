mod input;
pub use input::*;
mod delta_time;
use crate::Scene;
pub use delta_time::*;

pub(crate) fn add_default_resources(scene: &mut Scene) {
    scene
        .game_engine
        .entities_and_components
        .add_resource(Input::new());

    scene
        .game_engine
        .entities_and_components
        .add_resource(DeltaTime::new());
}
