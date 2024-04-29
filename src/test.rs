// not sure how to test this without nocap, i'll have to update this in the future
#[cfg(test)]
mod inputs_test {
    use std::thread;

    use crate::input::{Input, KeyCode, KeyState};
    use crate::{Scene, Transform};
    use console_renderer::camera::Camera;
    use console_renderer::*;

    //#[test]
    // this is a manual test, it will print A and B to the console when the keys are pressed
    // it is also terribly broken, but it is a start
    fn input_test() {
        let mut scene = Scene::new();
        let mut renderer = Renderer::new();
        let camera = Camera::new(1, 1);
        // add the camera to the scene
        scene
            .world
            .entities_and_components
            .add_entity_with((camera, Transform::default()));
        for _ in 0..100 {
            let entities_and_components = &mut scene.world.entities_and_components;
            let input = entities_and_components
                .get_resource::<Input>()
                .expect("Failed to get input");
            //input.update();
            if input.get_key_state(KeyCode::Escape) == KeyState::Held {
                break;
            }
            if input.get_key_state(KeyCode::A) == KeyState::Pressed {
                print!("A");
            }
            if input.get_key_state(KeyCode::B) == KeyState::Pressed {
                print!("B");
            }
            println!();
            renderer.render(entities_and_components);
        }
    }
}
