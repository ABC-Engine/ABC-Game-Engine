use ABC_Game_Engine::renderer::Renderer;
use ABC_Game_Engine::renderer::{Image, Sprite};
use ABC_Game_Engine::*;
use ABC_Game_Engine::{camera::Camera, Transform};

struct CameraMovementSystem {}

impl System for CameraMovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let entities = entities_and_components
            .get_entities_with_component::<Camera>()
            .cloned()
            .collect::<Vec<Entity>>();

        for entity in entities {
            let mut normalized_dir = [0.0 as f64; 2];
            let delta_time: f64;
            {
                delta_time = entities_and_components
                    .get_resource::<DeltaTime>()
                    .expect("failed to get delta time")
                    .delta_time;
                let input = entities_and_components
                    .get_resource::<Input>()
                    .expect("failed to get input");
                if input.is_key_pressed(Vk::W) {
                    normalized_dir[1] += -1.0;
                }
                if input.is_key_pressed(Vk::S) {
                    normalized_dir[1] += 1.0;
                }
                if input.is_key_pressed(Vk::A) {
                    normalized_dir[0] += -1.0;
                }
                if input.is_key_pressed(Vk::D) {
                    normalized_dir[0] += 1.0;
                }
                let magnitude = (normalized_dir[0].powi(2) + normalized_dir[1].powi(2)).sqrt();
                if magnitude != 0.0 {
                    normalized_dir[0] /= magnitude;
                    normalized_dir[1] /= magnitude;
                }
            }

            let (transform,) = entities_and_components.get_components_mut::<(Transform,)>(entity);

            transform.x += normalized_dir[0] * 10.0 * delta_time;
            transform.y += normalized_dir[1] * 10.0 * delta_time;
        }
    }
}

fn main() {
    let mut scene = Scene::new();
    let mut renderer = Renderer::new();
    renderer.set_stretch(1.0);
    {
        let entities_and_components = &mut scene.world.entities_and_components;
        let camera = Camera::new(160, 160);
        entities_and_components.add_entity_with((camera, Transform::default()));

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
    }

    {
        scene.world.add_system(CameraMovementSystem {});
    }

    loop {
        scene.world.run();
        renderer.render(
            &mut scene.world.entities_and_components,
            &scene.scene_params,
        );
    }
}
