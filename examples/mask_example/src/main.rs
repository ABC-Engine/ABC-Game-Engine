// this highlights some major issues with the current renderer
use console_renderer::camera::Camera;
use console_renderer::load_texture;
use console_renderer::mask::{Mask, MaskShape};
use console_renderer::Color;
use console_renderer::Renderer;
use console_renderer::Sprite;
use console_renderer::{Image, Rectangle};
use ABC_Game_Engine::Transform;
use ABC_Game_Engine::*;

const WINDOW_DIMS: (u32, u32) = (160, 160);

struct MovementSystem {
    player: Entity,
}

impl System for MovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
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

        let mut mask = entities_and_components
            .get_components_mut::<(Mask,)>(self.player)
            .0;

        mask.transform.x += normalized_dir[0] * 10.0 * delta_time;
        mask.transform.y += normalized_dir[1] * 10.0 * delta_time;
    }
}

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = Renderer::new();
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();

    let mask_entity: Entity;
    {
        let entities_and_components = &mut scene.world.entities_and_components;

        let camera = Camera::new(WINDOW_DIMS.0, WINDOW_DIMS.1);

        entities_and_components.add_entity_with((camera, Transform::default()));

        renderer.set_scene_params(renderer.get_scene_params().with_background_color(Color {
            r: 100,
            g: 0,
            b: 0,
            a: 0.0,
        }));

        renderer.set_scene_params(renderer.get_scene_params().with_random_chars(true));
        renderer.set_scene_params(renderer.get_scene_params().with_background_color(Color {
            r: 100,
            g: 100,
            b: 100,
            a: 1.0,
        }));

        let bar_outline = Image {
            texture: load_texture("Sample_Images/Xp_Bar_Border.png"),
        };

        let bar_outline_object = entities_and_components.add_entity_with((
            Sprite::Image(bar_outline),
            Transform {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                rotation: 0.0,
                scale: 1.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
        ));

        /*let bar_filling = Image {
            texture: load_texture("Sample_Images/Xp_Bar_Filling.png"),
        };*/

        let bar_filling = Rectangle {
            width: 1000.0,
            height: 1000.0,
            color: Color {
                r: 0,
                g: 255,
                b: 0,
                a: 1.0,
            },
        };

        let bar_filling_object = entities_and_components.add_entity_with((
            Sprite::Rectangle(bar_filling),
            Transform {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                rotation: 0.0,
                scale: 1.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
        ));

        let mask_rect = Rectangle {
            width: 10.0,
            height: 10.0,
            color: Color {
                r: 255,
                g: 0,
                b: 0,
                a: 0.1,
            },
        };

        let mask = Mask::new(
            MaskShape::Rectangle(mask_rect),
            Transform {
                x: -50.0,
                y: -50.0,
                z: 0.0,
                rotation: 0.0,
                scale: 1.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
        );

        entities_and_components.add_component_to(bar_filling_object, mask);

        mask_entity = bar_filling_object.clone();
    }

    scene.world.add_system(MovementSystem {
        player: mask_entity,
    });

    loop {
        scene.world.run();
        // should be implemented as a system later
        renderer.render(&mut scene.world.entities_and_components);
    }
}
