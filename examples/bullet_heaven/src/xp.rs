use crate::renderer::mask::Mask;
use ABC_Game_Engine::renderer::Circle;

use crate::*;
struct XpOrb {
    pub(crate) xp: u32,
}

pub(crate) fn spawn_xp_orb(
    entities_and_components: &mut EntitiesAndComponents,
    pos: [f64; 2],
    xp: u32,
) {
    let xp_orb_circle = Circle {
        radius: 2.0,
        color: Color {
            r: 0,
            g: 0,
            b: 255,
            a: 1.0,
        },
    };
    let _ = entities_and_components.add_entity_with((
        Sprite::Circle(xp_orb_circle),
        Transform {
            x: pos[0],
            y: pos[1],
            z: -1.0,
            rotation: 0.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        },
        XpOrb { xp },
    ));
}

pub(crate) struct XpOrbMovementSystem {
    pub(crate) player_entity: Entity,
    pub(crate) orb_speed: f64,
}

impl System for XpOrbMovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let player_transform: Transform;
        {
            player_transform = entities_and_components
                .get_components::<(Transform,)>(self.player_entity)
                .0
                .clone(); // player should always have a transform
        }
        let entities_with_xp_orbs = entities_and_components
            .get_entities_with_component::<XpOrb>()
            .cloned()
            .collect::<Vec<Entity>>();
        for xp_orb_entity in entities_with_xp_orbs {
            move_to_transform(
                entities_and_components,
                xp_orb_entity,
                player_transform,
                self.orb_speed,
            )
        }
    }
}

pub(crate) struct XpOrbCollisionSystem {
    pub(crate) player_entity: Entity,
}

impl System for XpOrbCollisionSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let entities_with_xp_orbs = entities_and_components
            .get_entities_with_component::<XpOrb>()
            .cloned()
            .collect::<Vec<Entity>>();

        // needs to be done this way because entity count changes as bullets are removed
        for xp_entity in entities_with_xp_orbs {
            let mut collided = false;
            if let Some(xp_transform) =
                entities_and_components.try_get_component::<Transform>(xp_entity)
            {
                // this is a very inefficient way to do this, but this serves as a good incentive to implement a collision system in the engine
                let player_transform: Transform;

                {
                    player_transform = entities_and_components
                        .get_components::<(Transform,)>(self.player_entity)
                        .0
                        .clone();
                }

                let distance = ((xp_transform.x - player_transform.x).powi(2)
                    + (xp_transform.y - player_transform.y).powi(2))
                .sqrt();

                if distance < 5.0 {
                    collided = true;
                }
            }
            if collided {
                let xp: u32;
                {
                    let (xp_orb,) = entities_and_components.get_components::<(XpOrb,)>(xp_entity);
                    xp = xp_orb.xp;
                }
                let (player_component,) =
                    entities_and_components.get_components_mut::<(Player,)>(self.player_entity);
                player_component.xp += xp;
                entities_and_components.remove_entity(xp_entity);
            }
        }
    }
}

/// tries to move the entity to the target transform at the given speed
fn move_to_transform(
    entities_and_components: &mut EntitiesAndComponents,
    self_entity: Entity,
    target_transform: Transform,
    speed: f64,
) {
    let delta_time: f64;
    {
        delta_time = entities_and_components
            .get_resource::<DeltaTime>()
            .expect("failed to get delta time")
            .delta_time;
    }

    if let (Some(self_transform),) =
        entities_and_components.try_get_components_mut::<(Transform,)>(self_entity)
    {
        let normalized_dir = [
            target_transform.x - self_transform.x,
            target_transform.y - self_transform.y,
        ];
        let magnitude = (normalized_dir[0].powi(2) + normalized_dir[1].powi(2)).sqrt();
        let normalized_dir = [normalized_dir[0] / magnitude, normalized_dir[1] / magnitude];
        self_transform.x += normalized_dir[0] * speed * delta_time;
        self_transform.y += normalized_dir[1] * speed * delta_time;
    }
}

pub(crate) struct PlayerUpgradingSystem {
    pub(crate) player_entity: Entity,
    pub(crate) next_upgrade: u32,
}

impl System for PlayerUpgradingSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let (player_component,) =
            entities_and_components.get_components::<(Player,)>(self.player_entity);
        if player_component.xp >= self.next_upgrade {
            upgrade_player(entities_and_components, self.player_entity);
            self.next_upgrade += 10;
        }
    }
}

fn upgrade_player(entities_and_components: &mut EntitiesAndComponents, player: Entity) {
    let (player_component,) = entities_and_components.get_components_mut::<(Player,)>(player);
    player_component.bullets_at_once += 1;
}

pub(crate) struct XpBarSystem {
    pub(crate) xp_bar_entity: Entity,
    pub(crate) player_entity: Entity,
    pub(crate) camera_entity: Entity,
}

impl System for XpBarSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let (xp, xp_to_next_upgrade);
        {
            let (player_component,) =
                entities_and_components.get_components::<(Player,)>(self.player_entity);
            xp = player_component.xp;
            xp_to_next_upgrade = player_component.xp_to_next_upgrade;
        }

        let camera_transform: Transform;
        {
            camera_transform = entities_and_components
                .get_components::<(Transform,)>(self.camera_entity)
                .0
                .clone();
        }
        {
            let (xp_bar_transform,) =
                entities_and_components.get_components_mut::<(Transform,)>(self.xp_bar_entity);
            xp_bar_transform.x = camera_transform.x;
            xp_bar_transform.y = camera_transform.y;
        }

        {
            let (xp_bar_mask,) =
                entities_and_components.get_components_mut::<(Mask,)>(self.xp_bar_entity);

            xp_bar_mask.transform.x = (xp as f64 / xp_to_next_upgrade as f64) * 81.0 + 19.0;
        }
    }
}
