// this highlights some major issues with the current renderer
use std::{thread, time, time::Instant};
use ABC_Game_Engine::*;

const WINDOW_DIMS: (u32, u32) = (160, 80);

struct SpinSystem {}

impl System for SpinSystem {
    fn run(&self, entities_and_components: &mut EntitiesAndComponents) {
        for i in 0..entities_and_components.get_entity_count() {
            let entity = entities_and_components.get_entity(i);
            entities_and_components
                .get_component_mut::<Transform>(entity)
                .rotation += 1.0;
        }
    }
}

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = Renderer::new(WINDOW_DIMS.0, WINDOW_DIMS.1);
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();
    scene.game_engine.add_system(Box::new(SpinSystem {}));
    {
        let mut entities_and_components = &mut scene.game_engine.entities_and_components;

        scene.scene_params.set_background_color(Color {
            r: 100,
            g: 0,
            b: 0,
            a: 0.0,
        });

        scene.scene_params.set_random_chars(true);

        let plague_mask = Image {
            texture: load_texture("Sample_Images/Icon10_01.png"),
        };

        let plague_mask_object = entities_and_components.add_entity();
        entities_and_components.add_component_to(plague_mask_object, Sprite::Image(plague_mask));
        entities_and_components.add_component_to(
            plague_mask_object,
            Transform {
                x: 20.0,
                y: 20.0,
                rotation: 0.0,
                scale: 2.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
        );
    }

    loop {
        let run_start = Instant::now();
        scene.game_engine.run();
        // should be implemented as a system later
        let run_time_ms = run_start.elapsed().as_millis();

        renderer.render(
            &scene.game_engine.entities_and_components,
            &scene.scene_params,
        );

        let render_time_ms = run_start.elapsed().as_millis() - run_time_ms;

        /*println!(
            "run time: {}ms \n render time: {}ms",
            run_time_ms, render_time_ms
        );*/
    }
}
