use ABC_Game_Engine::camera::Camera;
use ABC_Game_Engine::physics::colliders::{
    BoxCollider, CircleCollider, Collider, ColliderProperties,
};
use ABC_Game_Engine::physics::rigidbody::{RigidBody, Vec2};
use ABC_Game_Engine::renderer::{Circle, Rectangle, Renderer, Sprite};
use ABC_Game_Engine::*;

struct Player {}

struct PlayerController {
    speed: f32,
    jump_force: f32,
}

impl System for PlayerController {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let delta_time: f32;
        let mut normalized_dir = [0.0 as f32; 2];
        {
            delta_time = entities_and_components
                .get_resource::<DeltaTime>()
                .expect("Failed to get DeltaTime resource")
                .delta_time as f32;

            let input = entities_and_components.get_resource::<Input>().unwrap();

            if input.is_key_pressed(Vk::A) {
                normalized_dir[0] -= 1.0;
            }

            if input.is_key_pressed(Vk::D) {
                normalized_dir[0] += 1.0;
            }

            if input.is_key_pressed(Vk::Space) {
                normalized_dir[1] -= 1.0;
            }
        }

        let player_entities = entities_and_components
            .get_entities_with_component::<Player>()
            .cloned()
            .collect::<Vec<Entity>>();

        let player = player_entities[0];

        if let (Some(player), Some(transform), Some(rigid_body)) =
            entities_and_components.try_get_components_mut::<(Player, Transform, RigidBody)>(player)
        {
            rigid_body.apply_force(Vec2::new(
                normalized_dir[0] * self.speed * delta_time,
                normalized_dir[1] * self.jump_force * delta_time,
            ));
        }
    }
}

fn main() {
    let mut renderer = Renderer::new();
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();
    {
        let entities_and_components = &mut scene.world.entities_and_components;

        let camera = Camera::new(160, 160);

        entities_and_components.add_entity_with((camera, Transform::default()));

        renderer.set_scene_params(renderer.get_scene_params().with_background_color(Color {
            r: 100,
            g: 0,
            b: 0,
            a: 0.0,
        }));

        renderer.set_scene_params(renderer.get_scene_params().with_random_chars(true));

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
            ColliderProperties::new(false),
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
            RigidBody::new(25.0, Vec2::ZERO, 8.0),
            circle_collider,
            Player {},
        ));

        let ground_collider = Collider::new(
            BoxCollider::new(160.0, 10.0).into(),
            ColliderProperties::new(true),
        );

        entities_and_components.add_entity_with((
            Sprite::Rectangle(Rectangle {
                width: 160.0,
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
    }

    scene.world.add_system(PlayerController {
        speed: 100.0,
        jump_force: 1000.0,
    });
    physics::add_default_physics_systems(&mut scene);

    loop {
        scene.world.run();

        renderer.render(&mut scene.world.entities_and_components);
    }
}
