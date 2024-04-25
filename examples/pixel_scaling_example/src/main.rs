// this highlights some major issues with the current renderer
use console_renderer::camera::Camera;
use console_renderer::load_texture;
use console_renderer::Color;
use console_renderer::{Image, Renderer, Sprite};
use ABC_Game_Engine::Transform;
use ABC_Game_Engine::*;

const WINDOW_DIMS: (u32, u32) = (160, 80);

struct SpinSystem {}

impl System for SpinSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        // not efficient, but this is what has to be done for now
        let entities = entities_and_components
            .get_entities_with_component::<Transform>()
            .cloned()
            .collect::<Vec<Entity>>();

        for entity in entities {
            let (transform,) = entities_and_components.get_components_mut::<(Transform,)>(entity);
            transform.rotation += 0.01;
        }
    }
}

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = Renderer::new();
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();
    {
        let entities_and_components = &mut scene.world.entities_and_components;

        renderer.set_scene_params(renderer.get_scene_params().with_background_color(Color {
            r: 100,
            g: 0,
            b: 0,
            a: 0.0,
        }));

        renderer.set_scene_params(renderer.get_scene_params().with_random_chars(true));
        renderer.set_pixel_scale(2);

        let plague_mask = Image {
            texture: load_texture("Sample_Images/Icon10_01.png"),
        };

        entities_and_components.add_entity_with((
            Sprite::Image(plague_mask),
            Transform {
                x: 20.0,
                y: 20.0,
                z: 0.0,
                rotation: 0.0,
                scale: 2.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
        ));
        let camera = Camera::new(WINDOW_DIMS.0, WINDOW_DIMS.1);

        entities_and_components.add_entity_with((camera, Transform::default()));
    }

    scene.world.add_system(SpinSystem {});

    loop {
        scene.world.run();
        // should be implemented as a system later
        renderer.render(&mut scene.world.entities_and_components);
    }
}
