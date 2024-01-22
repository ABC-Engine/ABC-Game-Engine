use std::time::{Duration, Instant};

use rand::Rng;
use ABC_Game_Engine::camera::Camera;
use ABC_Game_Engine::physics::colliders::{
    BoxCollider, CircleCollider, Collider, ColliderProperties,
};
use ABC_Game_Engine::physics::rigidbody::{RigidBody, Vec2};
use ABC_Game_Engine::renderer::{Circle, Rectangle, Renderer, Sprite};
use ABC_Game_Engine::*;

fn main() {
    let mut renderer = Renderer::new();
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();
    {
        let entities_and_components = &mut scene.game_engine.entities_and_components;

        let camera = Camera::new(160, 160);

        entities_and_components.add_entity_with((camera, Transform::default()));

        scene.scene_params.set_background_color(Color {
            r: 100,
            g: 0,
            b: 0,
            a: 0.0,
        });

        scene.scene_params.set_random_chars(true);

        let ball = Circle {
            radius: 5.0,
            color: Color {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0,
            },
        };

        let circle_collider = Collider::new(
            CircleCollider::new(5.0).into(),
            ColliderProperties::default(),
        );

        entities_and_components.add_entity_with((
            Sprite::Circle(ball),
            Transform {
                x: -20.0,
                y: -20.0,
                z: 0.0,
                rotation: 0.0,
                scale: 1.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
            RigidBody::new(1.0, Vec2::ZERO, 0.9807),
            circle_collider,
        ));

        entities_and_components.add_entity_with((
            Sprite::Circle(ball),
            Transform {
                x: -19.0,
                y: -32.0,
                z: 0.0,
                rotation: 0.0,
                scale: 1.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
            RigidBody::new(1.0, Vec2::ZERO, 0.9807 * 2.0),
            circle_collider,
        ));

        entities_and_components.add_entity_with((
            Sprite::Circle(ball),
            Transform {
                x: -20.0,
                y: 0.0,
                z: 0.0,
                rotation: 0.0,
                scale: 1.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
            Collider::new(
                CircleCollider::new(5.0).into(),
                ColliderProperties::new(true),
            ),
        ));

        let ground_collider = Collider::new(
            BoxCollider::new(1000.0, 10.0).into(),
            ColliderProperties::new(true),
        );

        entities_and_components.add_entity_with((
            Sprite::Rectangle(Rectangle {
                width: 1000.0,
                height: 10.0,
                color: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 1.0,
                },
            }),
            Transform {
                x: 0.0,
                y: 80.0,
                z: 0.0,
                rotation: 0.0,
                scale: 1.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
            ground_collider,
        ));

        physics::add_default_physics_systems(&mut scene);
    }

    // currenty the circles eventually glitch out and go through the ground, this is likely due to the velocity building up.
    // TODO: implement rigidbody collision response
    let mut last_time_ball_was_spawned = Instant::now();
    loop {
        scene.game_engine.run();
        // add random balls
        if last_time_ball_was_spawned.elapsed() > Duration::from_secs_f32(0.5) {
            last_time_ball_was_spawned = Instant::now();
            // spawn at a random x position
            spawn_rb_ball(
                rand::thread_rng().gen_range(-80.0..80.0),
                &mut scene.game_engine.entities_and_components,
            );
        }
        renderer.render(
            &mut scene.game_engine.entities_and_components,
            &scene.scene_params,
        );
    }
}

fn spawn_rb_ball(x: f64, entities_and_components: &mut EntitiesAndComponents) {
    let circle_collider = Collider::new(
        CircleCollider::new(2.0).into(),
        ColliderProperties::default(),
    );

    let ball = Circle {
        radius: 2.0,
        color: Color {
            r: rand::thread_rng().gen_range(0..255),
            g: rand::thread_rng().gen_range(0..255),
            b: rand::thread_rng().gen_range(0..255),
            a: 1.0,
        },
    };

    entities_and_components.add_entity_with((
        Sprite::Circle(ball),
        Transform {
            x: x,
            y: -40.0,
            z: 0.0,
            rotation: 0.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        },
        RigidBody::new(1.0, Vec2::ZERO, 0.9807),
        circle_collider,
    ));
}
