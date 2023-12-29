use rand::Rng;
/// A basic bullet heaven made with the library
/// not yet complete
use std::{time::Instant, vec};
use ABC_Game_Engine::*;

const WINDOW_DIMS: (u32, u32) = (320, 320);

struct Player {
    health: u32,
    bullets_at_once: u32,
}

struct Enemy {
    health: u32,
}

struct Bullet {
    damage: u32,
    direction: [f64; 2],
}

struct PlayerMovementSystem {
    player_entity: Entity,
}

impl System for PlayerMovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        /*let input_entities = entities_and_components.get_entities_with_component::<Input>();
        let input_entity = input_entities[0];
        // there should only be one input component
        let (input,) = get_components_mut!(entities_and_components, input_entity, Input);*/
        let mut normalized_dir: [f64; 2] = [0.0, 0.0];
        {
            let input = entities_and_components
                .get_resource::<Input>()
                .expect("failed to get input");

            // this causes diagonal movement to be faster than cardinal movement
            // but that is fine for this example
            if input.is_key_pressed(Vk::W) {
                normalized_dir[1] -= 1.0;
            }
            if input.is_key_pressed(Vk::A) {
                normalized_dir[0] -= 1.0;
            }
            if input.is_key_pressed(Vk::S) {
                normalized_dir[1] += 1.0;
            }
            if input.is_key_pressed(Vk::D) {
                normalized_dir[0] += 1.0;
            }
        }

        {
            let (transform,) =
                entities_and_components.get_components_mut::<(Transform,)>(self.player_entity);

            let magnitude = (normalized_dir[0].powi(2) + normalized_dir[1].powi(2)).sqrt();

            if magnitude != 0.0 {
                normalized_dir = [normalized_dir[0] / magnitude, normalized_dir[1] / magnitude];
            }

            transform.x += normalized_dir[0];
            transform.y += normalized_dir[1];
        }
    }
}

struct PlayerShootingSystem {
    player_entity: Entity,
    last_shot: Instant,
    shot_rate: u128,
}

impl System for PlayerShootingSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        if self.last_shot.elapsed().as_millis() / 1000 > self.shot_rate {
            let bullets_to_fire: u32;
            let player_transform_copy: Transform;
            let mut closest_enemies_dirs: Vec<[f64; 2]>;
            {
                let (player_transform, player_component) = entities_and_components
                    .get_components::<(Transform, Player)>(self.player_entity);

                // check if there is an enemy in the scene and if so, get the normalized direction vector to the closest one
                // otherwise, return
                closest_enemies_dirs = vec![];
                {
                    // ordered largest to smallest
                    let mut closest_enemies: Vec<(Entity, f64)> = vec![];

                    for entity_index in 0..entities_and_components.get_entity_count() {
                        let other_entity = entities_and_components
                            .get_nth_entity(entity_index)
                            .unwrap(); // can't fail unless multithreaded
                        if let Some(_) =
                            entities_and_components.try_get_component::<Enemy>(other_entity)
                        {
                            //let (other_transform,) =
                            //    get_components!(entities_and_components, other_entity, Transform);

                            let (other_transform,) = entities_and_components
                                .get_components::<(Transform,)>(other_entity); // can't fail unless multithreaded

                            let distance = ((player_transform.x - other_transform.x).powi(2)
                                + (player_transform.y - other_transform.y).powi(2))
                            .sqrt();
                            if closest_enemies.len() == 0 {
                                closest_enemies = vec![(other_entity, distance)];
                            } else {
                                for (i, enemies) in closest_enemies.clone().into_iter().enumerate()
                                {
                                    if distance < enemies.1 {
                                        closest_enemies.insert(i, (other_entity, distance))
                                    }
                                }
                            }
                        }
                    }
                    for i in 0..player_component
                        .bullets_at_once
                        .min(closest_enemies.len() as u32)
                    {
                        /*let (closest_enemy_transform,) = get_components!(
                            entities_and_components,
                            closest_enemies.iter().nth(i as usize).unwrap().0,
                            Transform
                        );*/

                        let (closest_enemy_transform,) = entities_and_components
                            .get_components::<(Transform,)>(
                                closest_enemies.iter().nth(i as usize).unwrap().0,
                            );

                        let mut closest_enemy_dir = [
                            player_transform.x - closest_enemy_transform.x,
                            player_transform.y - closest_enemy_transform.y,
                        ];
                        let magnitude =
                            (closest_enemy_dir[0].powi(2) + closest_enemy_dir[1].powi(2)).sqrt();
                        closest_enemy_dir = [
                            closest_enemy_dir[0] / magnitude,
                            closest_enemy_dir[1] / magnitude,
                        ];
                        closest_enemies_dirs.push(closest_enemy_dir)
                    }
                }
                bullets_to_fire = player_component
                    .bullets_at_once
                    .min(closest_enemies_dirs.len() as u32);
                player_transform_copy = player_transform.clone();
            }

            for i in 0..bullets_to_fire {
                spawn_bullet(
                    entities_and_components,
                    [player_transform_copy.x, player_transform_copy.y],
                    *closest_enemies_dirs.iter().nth(i as usize).unwrap(),
                )
            }
            self.last_shot = Instant::now();
        }
    }
}

fn spawn_bullet(entities_and_components: &mut EntitiesAndComponents, pos: [f64; 2], dir: [f64; 2]) {
    let bullet_circle = Circle {
        radius: 2.0,
        color: Color {
            r: 255,
            g: 0,
            b: 0,
            a: 1.0,
        },
    };
    let bullet_entity = entities_and_components.add_entity();
    entities_and_components.add_component_to(bullet_entity, Sprite::Circle(bullet_circle));
    entities_and_components.add_component_to(
        bullet_entity,
        Transform {
            x: pos[0],
            y: pos[1],
            rotation: 0.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        },
    );
    entities_and_components.add_component_to(
        bullet_entity,
        Bullet {
            damage: 1,
            direction: dir,
        },
    );
}

struct BulletMovementSystem {
    bullet_speed: f64,
}

impl System for BulletMovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let entities_with_bullets = entities_and_components
            .get_entities_with_component::<Bullet>()
            .into_iter()
            .flatten()
            .cloned()
            .collect::<Vec<Entity>>();
        for bullet_entity in entities_with_bullets {
            let (bullet_transform, bullet) =
                entities_and_components.get_components_mut::<(Transform, Bullet)>(bullet_entity); // can't fail unless multithreaded

            bullet_transform.x -= self.bullet_speed * bullet.direction[0];
            bullet_transform.y -= self.bullet_speed * bullet.direction[1];
        }
    }
}

struct BulletCollisionSystem {
    player_entity: Entity,
    enemies_killed: u32,
    last_upgrade: u32,
}

impl System for BulletCollisionSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let mut bullet_index = 0;
        let entities_with_bullets = entities_and_components
            .get_entities_with_component::<Bullet>()
            .into_iter()
            .flatten()
            .cloned()
            .collect::<Vec<Entity>>();

        // needs to be done this way because entity count changes as bullets are removed
        for self_entity in entities_with_bullets {
            if let Some(self_transform) =
                entities_and_components.try_get_component::<Transform>(self_entity)
            {
                // this is a very inefficient way to do this, but this serves as a good incentive to implement a collision system in the engine
                let mut enemy_index = 0;
                let mut enemy_entities = entities_and_components
                    .get_entities_with_component::<Enemy>()
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect::<Vec<Entity>>();

                for enemy_entity in enemy_entities {
                    if let Some(other_transform) =
                        entities_and_components.try_get_component::<Transform>(enemy_entity)
                    {
                        let distance = ((self_transform.x - other_transform.x).powi(2)
                            + (self_transform.y - other_transform.y).powi(2))
                        .sqrt();

                        if distance < 5.0 {
                            entities_and_components.remove_entity(self_entity);
                            entities_and_components.remove_entity(enemy_entity);
                            if self.enemies_killed - self.last_upgrade == 5 {
                                upgrade_player(entities_and_components, self.player_entity);
                                self.last_upgrade = self.enemies_killed;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}

struct EnemyMovementSystem {
    player_entity: Entity,
    enemy_speed: f64,
}

impl System for EnemyMovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        for entity_index in 0..entities_and_components.get_entity_count() {
            let self_entity = entities_and_components
                .get_nth_entity(entity_index)
                .unwrap(); // can't fail unless multithreaded
            if let Some(_) = entities_and_components.try_get_component::<Enemy>(self_entity) {
                /*
                    this is how it would work ideally, but for now it is not possible
                    let player_transform =
                        entities_and_components.get_component::<Transform>(self.player_entity);
                    let mut self_transform =
                        entities_and_components.get_component_mut::<Transform>(self_entity);
                */
                let player_transform: Transform;

                {
                    player_transform = entities_and_components
                        .get_components::<(Transform,)>(self.player_entity)
                        .0
                        .clone();
                }

                let (self_transform,) =
                    entities_and_components.get_components_mut::<(Transform,)>(self_entity); // can't fail unless multithreaded

                let normalized_dir = [
                    player_transform.x - self_transform.x,
                    player_transform.y - self_transform.y,
                ];
                let magnitude = (normalized_dir[0].powi(2) + normalized_dir[1].powi(2)).sqrt();
                let normalized_dir = [normalized_dir[0] / magnitude, normalized_dir[1] / magnitude];
                self_transform.x += normalized_dir[0] * self.enemy_speed;
                self_transform.y += normalized_dir[1] * self.enemy_speed;
            }
        }
    }
}

struct EnemySpawnerSystem {
    last_spawn: Instant,
    spawn_rate: u128,
}

impl System for EnemySpawnerSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        if self.last_spawn.elapsed().as_millis() / 1000 > self.spawn_rate {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0.0..WINDOW_DIMS.0 as f64);
            let y = rng.gen_range(0.0..WINDOW_DIMS.1 as f64);
            let enemy_circle = Circle {
                radius: 5.0,
                color: Color {
                    r: 0,
                    g: 150,
                    b: 0,
                    a: 1.0,
                },
            };
            let enemy_entity = entities_and_components.add_entity();
            entities_and_components.add_component_to(enemy_entity, Sprite::Circle(enemy_circle));
            entities_and_components.add_component_to(
                enemy_entity,
                Transform {
                    x,
                    y,
                    rotation: 0.0,
                    scale: 1.0,
                    origin_x: 0.0,
                    origin_y: 0.0,
                },
            );
            entities_and_components.add_component_to(enemy_entity, Enemy { health: 1 });
            self.last_spawn = Instant::now();
            for i in 0..entities_and_components.get_entity_count() {
                let entity = entities_and_components.get_nth_entity(i).unwrap();
                if let Some(_) = entities_and_components.try_get_component::<Player>(entity) {
                    upgrade_player(entities_and_components, entity)
                }
            }
            self.spawn_rate = (self.spawn_rate as f32 * 0.95) as u128;
        }
    }
}

fn upgrade_player(entities_and_components: &mut EntitiesAndComponents, player: Entity) {
    let (player_component,) = entities_and_components.get_components_mut::<(Player,)>(player);
    player_component.bullets_at_once += 1;
}

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = Renderer::new(WINDOW_DIMS.0, WINDOW_DIMS.1);
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();
    let player_object: Entity;
    {
        let mut entities_and_components = &mut scene.game_engine.entities_and_components;

        scene.scene_params.set_background_color(Color {
            r: 100,
            g: 0,
            b: 0,
            a: 0.0,
        });

        scene.scene_params.set_random_chars(true);

        let player_image = Image {
            texture: load_texture("Sample_Images/Icon10_01.png"),
        };

        player_object = entities_and_components.add_entity();
        entities_and_components.add_component_to(player_object, Sprite::Image(player_image));
        entities_and_components.add_component_to(
            player_object,
            Transform {
                x: 20.0,
                y: 20.0,
                rotation: 0.0,
                scale: 1.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
        );
        entities_and_components.add_component_to(
            player_object,
            Player {
                health: 100,
                bullets_at_once: 1,
            },
        )
    }

    {
        // probably not proper form but for now it is more efficient than searching for every object with a component
        //  this should change in the future.
        scene.game_engine.add_system(Box::new(PlayerMovementSystem {
            player_entity: player_object,
        }));
        scene.game_engine.add_system(Box::new(EnemyMovementSystem {
            player_entity: player_object,
            enemy_speed: 0.1,
        }));
        scene.game_engine.add_system(Box::new(EnemySpawnerSystem {
            last_spawn: Instant::now(),
            spawn_rate: 2,
        }));
        scene.game_engine.add_system(Box::new(PlayerShootingSystem {
            player_entity: player_object,
            last_shot: Instant::now(),
            shot_rate: 1,
        }));
        scene
            .game_engine
            .add_system(Box::new(BulletMovementSystem { bullet_speed: 1.0 }));
        scene
            .game_engine
            .add_system(Box::new(BulletCollisionSystem {
                player_entity: player_object,
                enemies_killed: 0,
                last_upgrade: 0,
            }));
    }

    loop {
        std::env::set_var("RUST_BACKTRACE", "full");
        scene.game_engine.run();

        // should be implemented as a system later
        renderer.render(
            &scene.game_engine.entities_and_components,
            &scene.scene_params,
        );
    }
}
