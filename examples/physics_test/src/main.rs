use ABC_Game_Engine::camera::Camera;
use ABC_Game_Engine::physics::rigidbody::{RigidBody, Vec2};
use ABC_Game_Engine::renderer::{Circle, Renderer, Sprite};
use ABC_Game_Engine::*;

fn main() {
    let mut renderer = Renderer::new();
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();
    {
        let entities_and_components = &mut scene.game_engine.entities_and_components;

        let camera = Camera::new(160, 80);

        entities_and_components.add_entity_with((camera, Transform::default()));

        scene.scene_params.set_background_color(Color {
            r: 100,
            g: 0,
            b: 0,
            a: 0.0,
        });

        scene.scene_params.set_random_chars(true);

        let ball = Circle {
            radius: 10.0,
            color: Color {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0,
            },
        };

        let ball_object = entities_and_components.add_entity_with((
            Sprite::Circle(ball),
            Transform {
                x: -20.0,
                y: -20.0,
                z: 0.0,
                rotation: 0.0,
                scale: 2.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
            RigidBody::new(1.0, Vec2::ZERO, 0.9807),
        ));
        physics::add_default_physics_systems(&mut scene);
    }

    loop {
        scene.game_engine.run();
        renderer.render(
            &mut scene.game_engine.entities_and_components,
            &scene.scene_params,
        );
    }
}
