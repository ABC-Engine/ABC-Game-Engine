// this highlights some major issues with the current renderer
use ABC_Game_Engine::*;

const WINDOW_DIMS: (u32, u32) = (160, 80);

struct SpinSystem {}

impl System for SpinSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        // not efficient, but this is what has to be done for now
        let entities = entities_and_components
            .get_entities_with_component::<Sprite>()
            .cloned()
            .collect::<Vec<Entity>>();

        for entity in entities {
            let (transform,) = entities_and_components.get_components_mut::<(Transform,)>(entity);
            transform.rotation += 0.1;
        }
    }
}

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = Renderer::new(WINDOW_DIMS.0, WINDOW_DIMS.1);
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();
    {
        let entities_and_components = &mut scene.game_engine.entities_and_components;

        let camera = Camera::new(WINDOW_DIMS.0, WINDOW_DIMS.1);

        entities_and_components.add_entity_with((camera, Transform::default()));

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
                z: 0.0,
                rotation: 0.0,
                scale: 2.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
        );
    }

    scene.game_engine.add_system(Box::new(SpinSystem {}));

    loop {
        scene.game_engine.run();
        // should be implemented as a system later
        renderer.render(
            &mut scene.game_engine.entities_and_components,
            &scene.scene_params,
        );
    }
}
