// this highlights some major issues with the current renderer
use ABC_Game_Engine::*;
use ABC_Game_Engine::{camera::Camera, Transform};

const WINDOW_DIMS: (u32, u32) = (80, 80);

struct MovementSystem {
    player: Entity,
    /// 0 = UP_INDEX, 1 = RIGHT_INDEX, 2 = DOWN_INDEX, 3 = LEFT_INDEX
    idle_animations: Vec<Animation>,
    walk_animations: Vec<Animation>,
    /// this is used to prevent the animation from changing if the player is already facing that direction
    direction: u8,
    is_idle: bool,
}

impl System for MovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        const UP_INDEX: u8 = 0;
        const LEFT_INDEX: u8 = 1;
        const RIGHT_INDEX: u8 = 2;
        const DOWN_INDEX: u8 = 3;

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

        let (transform, sprite) =
            entities_and_components.get_components_mut::<(Transform, Sprite)>(self.player);

        transform.x += normalized_dir[0] * 10.0 * delta_time;
        transform.y += normalized_dir[1] * 10.0 * delta_time;

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

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = Renderer::new();
    renderer.set_stretch(1.0);
    let mut scene = Scene::new();
    let player_entity: Entity;
    {
        let entities_and_components = &mut scene.game_engine.entities_and_components;

        scene.scene_params.set_background_color(Color {
            r: 125,
            g: 125,
            b: 125,
            a: 1.0,
        });

        scene.scene_params.set_random_chars(false);
        scene.scene_params.set_character('█');

        let idle_animations = load_spritesheet(4, 4, 100, "Animations/sprite_sheet_idle.png");

        player_entity = entities_and_components.add_entity_with((
            Sprite::Animation(idle_animations[0].clone()),
            Transform {
                x: 40.0,
                y: 20.0,
                z: 0.0,
                scale: 1.0,
                rotation: 0.0,
                origin_x: 0.0,
                origin_y: 0.0,
            },
        ));

        let camera = Camera::new(WINDOW_DIMS.0, WINDOW_DIMS.1);

        entities_and_components.add_entity_with((camera, Transform::default()));
    }

    {
        let idle_animations = load_spritesheet(4, 4, 100, "Animations/sprite_sheet_idle.png");
        let walk_animations = load_spritesheet(4, 4, 100, "Animations/sprite_sheet_walk.png");

        scene.game_engine.add_system(Box::new(MovementSystem {
            player: player_entity,
            idle_animations,
            walk_animations,
            direction: 0,
            is_idle: true,
        }));
    }

    loop {
        scene.game_engine.run();
        // should be implemented as a system later
        renderer.render(
            &mut scene.game_engine.entities_and_components,
            &scene.scene_params,
        );
    }
}
