<h1 align="center">Floneum</h1>
<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/ABC_Game_Engine">
    <img src="https://img.shields.io/crates/v/ABC_Game_Engine.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/ABC_Game_Engine">
    <img src="https://img.shields.io/crates/d/ABC_Game_Engine.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs -->
  <a href="https://docs.rs/ABC_Game_Engine">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- Discord -->
  <a href="https://discord.gg/55R3GsBSYC">
    <img src="https://img.shields.io/discord/1224029164798087239?logo=discord&style=flat-square" alt="Discord Link" />
  </a>
</div>

![s2Kkdcv](https://github.com/ABC-Engine/ABC-Game-Engine/assets/76850177/9f511895-ed68-498e-bd8b-8741ae10cfa2)

# [Website](https://abc-engine.com)

# Under Construction 🚧
This project is still under construction and is in the pre-release stage. Significant changes will be made that might break things. We apologize for the inconvenience.

# Ease of use
The idea for this project is that it would be so intuitive to use that it would be feasible to learn rust through this game engine. Learning is often made better when you can make something interactable rather than *just* manipulating data and we want to provide that opportunity in Rust.

## OS Support
Most major operating systems *should* be supported, however only Windows has been tested.

# Current Features
For a complete list of features that are planned or done visit [here](https://www.figma.com/design/RGxexDMjVnLHFsUxnKaHhu/ABC-Engine-Roadmap?node-id=0-1&t=mPowocl1rquKjVa9-1)

## Renderers
Both a [console renderer](https://github.com/ABC-Engine/console_renderer) and a [pixel art renderer](https://github.com/ABC-Engine/lumenpyx).

## Physics Engine
Rapier is used for physics and can be accessed for collision info as a resource.

## A Simple Platformer Example

```rust
use ABC_Game_Engine::physics::rapier2d::geometry::Ray;
use ABC_Game_Engine::physics::rapier2d::math::Real;
use ABC_Game_Engine::physics::rapier2d::na as nalgebra;
use ABC_Game_Engine::physics::rapier2d::na::vector;
use ABC_Game_Engine::prelude::*;
use ABC_lumenpyx::prelude::*;

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

            let is_ground_check = |collider_handle: ColliderHandle, _: &Collider| {
                let entity = physics_info
                    .get_associated_entity_with_collider_handle(collider_handle.into())
                    .expect("Failed to get associated entity with collider handle");

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
                    normalized_dir[1] = 1.0;
                }
            }
        }

        if let (Some(_), Some(_), Some(rigid_body)) = entities_and_components
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

        let ground_collider = ColliderBuilder::cuboid(160.0, 5.0).build();
        let ground_rb = RigidBodyBuilder::fixed().build();
        let ground_rect = Rectangle::new([0.0, 1.0, 1.0, 1.0], 160.0, 10.0);

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

    let physics_info = scene
        .world
        .entities_and_components
        .get_resource_mut::<RapierPhysicsInfo>();

    physics_info
        .expect("Failed to get PhysicsInfo resource")
        .set_gravity([0.0, -9.81 * 4.0].into());

    lumenpyx_eventloop.run(&mut scene.world, |world| {
        world.run();

        render(&mut world.entities_and_components);
    });
}
```
