/// A basic bullet heaven made with the library
/// not yet complete
use rand::Rng;
use std::{time::Instant, vec};
use ABC_Game_Engine::*;
use ABC_Game_Engine::{camera::Camera, Transform};
mod xp;
use xp::*;

const WINDOW_DIMS: (u32, u32) = (80, 80);
const PLAYER_HIT_BOX_RADIUS: f64 = 5.0;
// if true displays pixel like characters if not displays random characters
const PIXEL_MODE: bool = true;

struct Player {
    health: u32,
    bullets_at_once: u32,
    speed: f64,
    xp: u32,
    invincibility_time_ms: u128,
    last_hit: Instant,
    is_invincible: bool,
}

#[derive(Clone)]
struct Enemy {
    health: u32,
    damage: u32,
}

struct Bullet {
    damage: u32,
    direction: [f64; 2],
}

struct PlayerInvincibilitySystem {
    player_entity: Entity,
}

impl System for PlayerInvincibilitySystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let player_component: &mut Player;
        {
            player_component = entities_and_components
                .get_components_mut::<(Player,)>(self.player_entity)
                .0;
        }
        if player_component.is_invincible {
            if player_component.last_hit.elapsed().as_millis() / 1000
                > player_component.invincibility_time_ms
            {
                player_component.is_invincible = false;
            }
        }
    }
}

struct PlayerMovementSystem {
    player_entity: Entity,
    direction: u8,
    walk_animations: Vec<Animation>,
    idle_animations: Vec<Animation>,
    is_idle: bool,
}

impl System for PlayerMovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        const UP_INDEX: u8 = 0;
        const LEFT_INDEX: u8 = 1;
        const RIGHT_INDEX: u8 = 2;
        const DOWN_INDEX: u8 = 3;

        let mut normalized_dir = [0.0 as f64; 2];
        let delta_time: f64;
        let player_speed: f64;
        {
            delta_time = entities_and_components
                .get_resource::<DeltaTime>()
                .expect("failed to get delta time")
                .delta_time;
            let input = entities_and_components
                .get_resource::<Input>()
                .expect("failed to get input");

            player_speed = entities_and_components
                .get_components::<(Player,)>(self.player_entity)
                .0
                .speed;

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

        let (transform, sprite) =
            entities_and_components.get_components_mut::<(Transform, Sprite)>(self.player_entity);

        transform.x += normalized_dir[0] * player_speed * delta_time;
        transform.y += normalized_dir[1] * player_speed * delta_time;

        let mut animation = match sprite {
            Sprite::Animation(animation) => animation,
            _ => panic!("Player sprite is not an animation"),
        };

        if normalized_dir[0] == 0.0 && normalized_dir[1] == 0.0 {
            if self.is_idle {
                return;
            }
            self.is_idle = true;

            *animation = self.idle_animations[self.direction.min(3) as usize].clone();
        } else if normalized_dir[0] > 0.0 {
            if self.direction == RIGHT_INDEX && !self.is_idle {
                return;
            }
            self.is_idle = false;

            *animation = self.walk_animations[RIGHT_INDEX as usize].clone();
            self.direction = RIGHT_INDEX;
        } else if normalized_dir[0] < 0.0 {
            if self.direction == LEFT_INDEX && !self.is_idle {
                return;
            }
            self.is_idle = false;

            *animation = self.walk_animations[LEFT_INDEX as usize].clone();
            self.direction = LEFT_INDEX;
        } else if normalized_dir[1] > 0.0 {
            if self.direction == UP_INDEX && !self.is_idle {
                return;
            }
            self.is_idle = false;

            *animation = self.walk_animations[UP_INDEX as usize].clone();
            self.direction = UP_INDEX;
        } else if normalized_dir[1] < 0.0 {
            if self.direction == DOWN_INDEX && !self.is_idle {
                return;
            }
            self.is_idle = false;

            *animation = self.walk_animations[DOWN_INDEX as usize].clone();
            self.direction = DOWN_INDEX;
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
            z: 0.0,
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
        let delta_time: f64;
        {
            delta_time = entities_and_components
                .get_resource::<DeltaTime>()
                .expect("failed to get delta time")
                .delta_time;
        }

        let entities_with_bullets = entities_and_components
            .get_entities_with_component::<Bullet>()
            .cloned()
            .collect::<Vec<Entity>>();
        for bullet_entity in entities_with_bullets {
            let (bullet_transform, bullet) =
                entities_and_components.get_components_mut::<(Transform, Bullet)>(bullet_entity); // can't fail unless multithreaded

            bullet_transform.x -= self.bullet_speed * bullet.direction[0] * delta_time;
            bullet_transform.y -= self.bullet_speed * bullet.direction[1] * delta_time;
        }
    }
}

struct BulletCollisionSystem {}

impl System for BulletCollisionSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let entities_with_bullets = entities_and_components
            .get_entities_with_component::<Bullet>()
            .cloned()
            .collect::<Vec<Entity>>();

        // needs to be done this way because entity count changes as bullets are removed
        for self_entity in entities_with_bullets {
            if let Some(self_transform) =
                entities_and_components.try_get_component::<Transform>(self_entity)
            {
                // this is a very inefficient way to do this, but this serves as a good incentive to implement a collision system in the engine
                let enemy_entities = entities_and_components
                    .get_entities_with_component::<Enemy>()
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
                            spawn_xp_orb(
                                entities_and_components,
                                [other_transform.x, other_transform.y],
                                1,
                            );
                            entities_and_components.remove_entity(self_entity);
                            entities_and_components.remove_entity(enemy_entity);
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
        let delta_time: f64;
        {
            delta_time = entities_and_components
                .get_resource::<DeltaTime>()
                .expect("failed to get delta time")
                .delta_time;
        }
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
                self_transform.x += normalized_dir[0] * self.enemy_speed * delta_time;
                self_transform.y += normalized_dir[1] * self.enemy_speed * delta_time;
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
                    z: 0.0,
                    rotation: 0.0,
                    scale: 1.0,
                    origin_x: 0.0,
                    origin_y: 0.0,
                },
            );
            entities_and_components.add_component_to(
                enemy_entity,
                Enemy {
                    health: 1,
                    damage: 10,
                },
            );
            self.last_spawn = Instant::now();
            self.spawn_rate = (self.spawn_rate as f32 * 0.95) as u128;
        }
    }
}

struct EnemyPlayerCollisionSystem {
    player_entity: Entity,
}

impl System for EnemyPlayerCollisionSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        if entities_and_components
            .get_components::<(Player,)>(self.player_entity)
            .0
            .is_invincible
        {
            return;
        }
        let player_transform: Transform;
        {
            player_transform = entities_and_components
                .get_components::<(Transform,)>(self.player_entity)
                .0
                .clone();
        }

        let entities_with_enemies = entities_and_components
            .get_entities_with_component::<Enemy>()
            .cloned()
            .collect::<Vec<Entity>>();

        for enemy_entity in entities_with_enemies {
            let enemy_transform: Transform;
            let enemy_component: Enemy;
            {
                enemy_transform = entities_and_components
                    .get_components::<(Transform,)>(enemy_entity)
                    .0
                    .clone();
                enemy_component = entities_and_components
                    .get_components::<(Enemy,)>(enemy_entity)
                    .0
                    .clone();
            }

            let distance = ((player_transform.x - enemy_transform.x).powi(2)
                + (player_transform.y - enemy_transform.y).powi(2))
            .sqrt();

            if distance < PLAYER_HIT_BOX_RADIUS {
                let player = entities_and_components
                    .get_components_mut::<(Player,)>(self.player_entity)
                    .0;
                player.health -= enemy_component.damage;
                player.is_invincible = true;
                player.last_hit = Instant::now();
            }
        }
    }
}

struct PlayerDeathSystem {
    player_entity: Entity,
}

impl System for PlayerDeathSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        if entities_and_components
            .get_components::<(Player,)>(self.player_entity)
            .0
            .health
            <= 0
        {
            entities_and_components.remove_entity(self.player_entity);
            panic!("Player died");
        }
    }
}

struct CameraMovementSystem {
    player_entity: Entity,
    camera_entity: Entity,
    camera_speed: f64,
}

impl System for CameraMovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let player_transform: Transform;
        let delta_time: f64;
        {
            delta_time = entities_and_components
                .get_resource::<DeltaTime>()
                .expect("failed to get delta time")
                .delta_time;

            player_transform = entities_and_components
                .get_components::<(Transform,)>(self.player_entity)
                .0
                .clone();
        }

        let (camera_transform,) =
            entities_and_components.get_components_mut::<(Transform,)>(self.camera_entity); // can't fail unless multithreaded

        camera_transform.x +=
            (player_transform.x - camera_transform.x) * delta_time * self.camera_speed;
        camera_transform.y +=
            (player_transform.y - camera_transform.y) * delta_time * self.camera_speed;
    }
}

struct BackgroundSystem {
    camera_entity: Entity,
    background_tiles: Vec<Entity>,
    background_sprite: Sprite,
}

/// places background tiles neat the camera so that the player can't see the edge of the screen
impl System for BackgroundSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        const TRANSFORM: Transform = Transform {
            x: 0.0,
            y: 0.0,
            z: -5.0,
            rotation: 0.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        };

        let camera_transform: Transform;
        {
            camera_transform = entities_and_components
                .get_components::<(Transform,)>(self.camera_entity)
                .0
                .clone();
        }

        if self.background_tiles.is_empty() {
            let mut tile_transform = TRANSFORM.clone();
            tile_transform.x = camera_transform.x;
            tile_transform.y = camera_transform.y;
            let tile_entity = entities_and_components.add_entity();
            entities_and_components.add_component_to(tile_entity, self.background_sprite.clone());
            entities_and_components.add_component_to(tile_entity, tile_transform);
            self.background_tiles.push(tile_entity);
        }

        /*if self.background_tiles.len() < 9 {
            for entity in &self.background_tiles {
                entities_and_components.remove_entity(*entity);
            }
            // the tile repeats every 4x4 pixels so you can use a modulo to get the correct tile position
            for i in 0..3 {
                for j in 0..3 {
                    let mut tile_transform = TRANSFORM.clone();
                    tile_transform.x = camera_transform.x + i as f64 * 160.0;
                    tile_transform.y = camera_transform.y + j as f64 * 160.0;
                    let tile_entity = entities_and_components.add_entity();
                    entities_and_components
                        .add_component_to(tile_entity, self.background_sprite.clone());
                    entities_and_components.add_component_to(tile_entity, tile_transform);
                    self.background_tiles.push(tile_entity);
                }
            }
        } else {
            // see which tiles are within the camera view and which are not
            let mut tiles_to_move = vec![];
            for (i, tile_entity) in self.background_tiles.iter().enumerate() {
                let tile_transform: Transform;
                {
                    tile_transform = entities_and_components
                        .get_components::<(Transform,)>(*tile_entity)
                        .0
                        .clone();
                }
                if tile_transform.x < camera_transform.x - 160.0
                    || tile_transform.x > camera_transform.x + 160.0
                    || tile_transform.y < camera_transform.y - 160.0
                    || tile_transform.y > camera_transform.y + 160.0
                {
                    tiles_to_move.push(i);
                }
            }

            // see which tiles are within the camera view but don't exist in the scene
            let mut tiles_to_move_to = vec![];
            for i in 0..3 {
                for j in 0..3 {
                    let mut tile_transform = TRANSFORM.clone();
                    tile_transform.x = camera_transform.x + i as f64 * 160.0;
                    tile_transform.y = camera_transform.y + j as f64 * 160.0;
                    let mut tile_exists = false;
                    for tile_entity in &self.background_tiles {
                        let tile_transform: Transform;
                        {
                            tile_transform = entities_and_components
                                .get_components::<(Transform,)>(*tile_entity)
                                .0
                                .clone();
                        }
                        if tile_transform.x == tile_transform.x
                            && tile_transform.y == tile_transform.y
                        {
                            tile_exists = true;
                            break;
                        }
                    }
                    if !tile_exists {
                        tiles_to_move_to.push(tile_transform);
                    }
                }
            }

            // move the tiles that are outside the camera view to the tiles that are inside the camera view but don't exist in the scene
            for i in 0..tiles_to_move.len().min(tiles_to_move_to.len()) {
                let tile_entity = self.background_tiles[tiles_to_move[i]];
                let tile_transform = tiles_to_move_to[i];
                entities_and_components
                    .get_components_mut::<(Transform,)>(tile_entity)
                    .0
                    .x = tile_transform.x;
                entities_and_components
                    .get_components_mut::<(Transform,)>(tile_entity)
                    .0
                    .y = tile_transform.y;
            }
        }*/
    }
}

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = Renderer::new();
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();
    let player_object: Entity;
    let camera_object: Entity;
    {
        let entities_and_components = &mut scene.game_engine.entities_and_components;

        scene.scene_params.set_background_color(Color {
            r: 100,
            g: 0,
            b: 0,
            a: 0.0,
        });

        match PIXEL_MODE {
            true => {
                scene.scene_params.set_character('█');
            }
            false => {
                scene.scene_params.set_random_chars(true);
            }
        }

        let player_image = Image {
            texture: load_texture("Sample_Images/Icon10_01.png"),
        };

        let idle_animations = load_spritesheet(4, 4, 100, "Animations/sprite_sheet_idle.png");

        player_object = entities_and_components.add_entity();
        entities_and_components.add_component_to(
            player_object,
            Sprite::Animation(idle_animations.get(0).unwrap().clone()),
        );
        entities_and_components.add_component_to(
            player_object,
            Transform {
                x: 20.0,
                y: 20.0,
                z: 100.0,
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
                speed: 40.0,
                xp: 0,
                invincibility_time_ms: 500,
                last_hit: Instant::now(),
                is_invincible: false,
            },
        );

        let camera = Camera::new(WINDOW_DIMS.0, WINDOW_DIMS.1);

        camera_object = entities_and_components.add_entity_with((camera, Transform::default()));
    }

    {
        let idle_animations = load_spritesheet(4, 4, 100, "Animations/sprite_sheet_idle.png");
        let walk_animations = load_spritesheet(4, 4, 100, "Animations/sprite_sheet_walk.png");

        // probably not proper form but for now it is more efficient than searching for every object with a component
        //  this should change in the future.
        scene.game_engine.add_system(Box::new(PlayerMovementSystem {
            player_entity: player_object,
            idle_animations,
            walk_animations,
            direction: 0,
            is_idle: true,
        }));
        scene.game_engine.add_system(Box::new(EnemyMovementSystem {
            player_entity: player_object,
            enemy_speed: 10.0,
        }));
        scene.game_engine.add_system(Box::new(EnemySpawnerSystem {
            last_spawn: Instant::now(),
            spawn_rate: 200000,
        }));
        scene.game_engine.add_system(Box::new(PlayerShootingSystem {
            player_entity: player_object,
            last_shot: Instant::now(),
            shot_rate: 1,
        }));
        scene.game_engine.add_system(Box::new(BulletMovementSystem {
            bullet_speed: 100.0,
        }));
        scene
            .game_engine
            .add_system(Box::new(BulletCollisionSystem {}));
        scene.game_engine.add_system(Box::new(XpOrbMovementSystem {
            player_entity: player_object,
            orb_speed: 50.0,
        }));
        scene.game_engine.add_system(Box::new(XpOrbCollisionSystem {
            player_entity: player_object,
        }));
        scene
            .game_engine
            .add_system(Box::new(PlayerUpgradingSystem {
                player_entity: player_object,
                next_upgrade: 10,
            }));
        scene
            .game_engine
            .add_system(Box::new(EnemyPlayerCollisionSystem {
                player_entity: player_object,
            }));
        scene.game_engine.add_system(Box::new(PlayerDeathSystem {
            player_entity: player_object,
        }));
        scene.game_engine.add_system(Box::new(CameraMovementSystem {
            player_entity: player_object,
            camera_entity: camera_object,
            camera_speed: 2.0,
        }));
        // can't be added until z ordering is implemented
        scene.game_engine.add_system(Box::new(BackgroundSystem {
            camera_entity: camera_object,
            background_tiles: vec![],
            background_sprite: Sprite::Image(Image {
                texture: load_texture("Sample_Images/Background.png"),
            }),
        }));
    }

    // start the main game music
    {
        let audio_handle = scene
            .game_engine
            .entities_and_components
            .get_resource::<AudioHandle>()
            .expect("Failed to get audio handle");

        let audio_file = AudioFile::new("main_music.wav");
        audio_handle.play_infinitely(audio_file);
    }

    loop {
        //std::env::set_var("RUST_BACKTRACE", "full");
        scene.game_engine.run();

        // should be implemented as a system later
        renderer.render(
            &mut scene.game_engine.entities_and_components,
            &scene.scene_params,
        );
    }
}
