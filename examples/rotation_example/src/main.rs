use ABC_Game_Engine::get_transform;
use ABC_Game_Engine::{
    physics::rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder},
    Scene, Transform,
};
use ABC_lumenpyx::{primitives::Circle, render, Camera, LumenpyxEventLoop, RenderSettings};

struct RotationSystem {
    parent: ABC_Game_Engine::Entity,
    a_plus_b_entity: ABC_Game_Engine::Entity,
}

impl ABC_Game_Engine::System for RotationSystem {
    fn run(&mut self, entities_and_components: &mut ABC_Game_Engine::EntitiesAndComponents) {
        let (transform,) = entities_and_components.get_components_mut::<(Transform,)>(self.parent);

        transform.rotation += 0.01;
        transform.x = 50.0 * transform.rotation.cos();

        // to test the Add and Sub impls
        let transform = transform.clone();
        let (a_plus_b_transform,) =
            entities_and_components.get_components_mut::<(Transform,)>(self.a_plus_b_entity);

        *a_plus_b_transform = &transform
            + &Transform {
                x: 20.0,
                y: 20.0,
                z: 1.0,
                ..Default::default()
            };

        *a_plus_b_transform = &*a_plus_b_transform - &transform;
    }
}

fn main() {
    let mut scene = Scene::new();

    let mut lumenpyx_eventloop =
        LumenpyxEventLoop::new(&mut scene.world, [160, 160], "Rotation Example");

    lumenpyx_eventloop.set_render_settings(
        &mut scene.world,
        RenderSettings::default()
            .with_reflections(false)
            .with_shadows(false),
    );

    scene
        .world
        .entities_and_components
        .add_entity_with((Camera::new(), Transform::default()));

    let parent = scene.world.entities_and_components.add_entity_with((
        Circle::new([1.0, 1.0, 1.0, 1.0], 5.0),
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

    let child = scene.world.entities_and_components.add_entity_with((
        Circle::new([1.0, 0.0, 1.0, 1.0], 5.0),
        Transform {
            x: 20.0,
            y: 20.0,
            z: 0.0,
            rotation: 0.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        },
    ));

    // shouldn't move
    let a_plus_b_entity = scene.world.entities_and_components.add_entity_with((
        Circle::new([0.0, 0.0, 1.0, 1.0], 5.0),
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

    scene
        .world
        .entities_and_components
        .set_parent(child, parent);

    scene.world.add_system(RotationSystem {
        parent: parent,
        a_plus_b_entity,
    });

    lumenpyx_eventloop.run(&mut scene.world, |world| {
        world.run();

        render(&mut world.entities_and_components);
    });
}
