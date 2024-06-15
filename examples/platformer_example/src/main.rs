use core::panic;

use physics::rapier2d::na as nalgebra;
use physics::rapier2d::prelude::RigidBody;
use ABC_Game_Engine::physics::physics_system::RapierPhysicsInfo;
use ABC_Game_Engine::physics::rapier2d::dynamics::RigidBodyBuilder;
use ABC_Game_Engine::physics::rapier2d::geometry::Collider;
use ABC_Game_Engine::physics::rapier2d::geometry::ColliderBuilder;
use ABC_Game_Engine::physics::rapier2d::geometry::ColliderHandle;
use ABC_Game_Engine::physics::rapier2d::geometry::Ray;
use ABC_Game_Engine::physics::rapier2d::math::Real;
use ABC_Game_Engine::physics::rapier2d::na::vector;
use ABC_Game_Engine::physics::rapier2d::pipeline::QueryFilter;
use ABC_Game_Engine::*;
use ABC_lumenpyx::primitives::Circle;
use ABC_lumenpyx::primitives::Rectangle;
use ABC_lumenpyx::render;
use ABC_lumenpyx::Camera;
use ABC_lumenpyx::LumenpyxEventLoop;
use ABC_lumenpyx::RenderSettings;

struct Player;
struct Ground;

struct PlayerController {
    speed: f32,
    jump_force: f32,
}

impl System for PlayerController {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let player_entity;
        let (player_x, player_y) = {
            let player_entities = entities_and_components
                .get_entities_with_component::<Player>()
                .cloned()
                .collect::<Vec<Entity>>();

            player_entity = player_entities[0];

            let (transform,) =
                entities_and_components.get_components::<(Transform,)>(player_entity);
            (transform.x, transform.y)
        };

        let delta_time: f32;
        let mut normalized_dir = [0.0 as f32; 2];
        {
            delta_time = entities_and_components
                .get_resource::<DeltaTime>()
                .expect("Failed to get DeltaTime resource")
                .get_delta_time() as f32;

            let physics_info = entities_and_components
                .get_resource::<RapierPhysicsInfo>()
                .expect("Failed to get PhysicsInfo resource");

            let input = entities_and_components.get_resource::<Input>().unwrap();

            if input.get_key_state(KeyCode::A) == KeyState::Held {
                normalized_dir[0] -= 1.0;
            }

            if input.get_key_state(KeyCode::D) == KeyState::Held {
                normalized_dir[0] += 1.0;
            }

            let is_ground_check = |collider_handle: ColliderHandle, collider: &Collider| {
                let entity = physics_info
                    .get_associated_entity_with_collider_handle(collider_handle.into())
                    .expect("Failed to get associated entity with collider handle");

                let (entities_and_components, entity) =
                    entity.access_entity(entities_and_components);

                entities_and_components
                    .try_get_components::<(Ground,)>(entity)
                    .0
                    .is_some()
            };

            let is_ground_filter = QueryFilter {
                predicate: Some(&is_ground_check),
                ..Default::default()
            };

            let intersection = physics_info.cast_ray(
                &Ray::new(
                    vector![player_x as f32, player_y as f32 - 5.01].into(),
                    vector![0.0, -1.0],
                ),
                Real::MAX,
                true,
                is_ground_filter,
            );

            if input.get_key_state(KeyCode::Space) == KeyState::Pressed && intersection.is_some() {
                let intersection = intersection.unwrap();

                if intersection.1 < 0.01 {
                    let (_, intersection_entity) =
                        intersection.0.access_entity(entities_and_components);

                    if intersection_entity == player_entity {
                        panic!("Player is intersecting with itself");
                    }
                    normalized_dir[1] = 1.0;
                }
            }
        }

        if let (Some(player), Some(transform), Some(rigid_body)) = entities_and_components
            .try_get_components_mut::<(Player, Transform, RigidBody)>(player_entity)
        {
            rigid_body.apply_impulse(
                vector![
                    normalized_dir[0] * self.speed * delta_time,
                    normalized_dir[1] * self.jump_force,
                ],
                true,
            );
        }
    }
}

fn main() {
    let mut scene = Scene::new();

    let mut lumenpyx_eventloop =
        LumenpyxEventLoop::new(&mut scene.world, [160, 160], "Platformer Example");

    lumenpyx_eventloop.set_render_settings(
        &mut scene.world,
        RenderSettings::default()
            .with_reflections(false)
            .with_shadows(false),
    );
    {
        let entities_and_components = &mut scene.world.entities_and_components;

        let camera = Camera::new();

        entities_and_components.add_entity_with((camera, Transform::default()));

        let ball = Circle::new([1.0, 1.0, 1.0, 1.0], 5.0);

        let circle_collider = ColliderBuilder::ball(5.0).build();
        let circle_rb = RigidBodyBuilder::dynamic().additional_mass(1.0).build();

        entities_and_components.add_entity_with((
            ball,
            Transform {
                x: -20.0,
                y: -20.0,
                z: 0.0,
                rotation: 0.0,
                scale: 1.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
            circle_rb,
            circle_collider,
            Player {},
        ));

        let ground_collider = ColliderBuilder::cuboid(160.0, 10.0).build();
        let ground_rb = RigidBodyBuilder::fixed().build();
        let ground_rect = Rectangle::new([1.0, 1.0, 1.0, 1.0], 160.0, 10.0);

        entities_and_components.add_entity_with((
            ground_rect,
            Transform {
                x: 0.0,
                y: -80.0,
                z: 0.0,
                rotation: 0.0,
                scale: 1.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
            ground_collider,
            ground_rb,
            Ground {},
        ));
    }

    scene.world.add_system(PlayerController {
        speed: 1000.0,
        jump_force: 3000.0,
    });
    physics::add_default_physics_systems(&mut scene.world);

    lumenpyx_eventloop.run(&mut scene.world, |world| {
        world.run();

        render(&mut world.entities_and_components);
    });
}
