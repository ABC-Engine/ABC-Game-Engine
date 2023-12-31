pub mod input;
use crate::Scene;
pub use input::*;
pub mod delta_time;
pub use delta_time::*;
pub mod audio_stream;
pub use audio_stream::*;

pub(crate) fn add_default_resources(scene: &mut Scene) {
    scene
        .game_engine
        .entities_and_components
        .add_resource(Input::new());

    scene
        .game_engine
        .entities_and_components
        .add_resource(DeltaTime::new());

    scene
        .game_engine
        .entities_and_components
        .add_resource(AudioHandle::new());
}
