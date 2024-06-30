use core::panic;

use physics::rapier2d::na as nalgebra;
use physics::rapier2d::prelude::RigidBody;

use ui::Slider;
use ABC_Game_Engine::*;
use ABC_lumenpyx::primitives::Circle;
use ABC_lumenpyx::primitives::Rectangle;
use ABC_lumenpyx::render;
use ABC_lumenpyx::Camera;
use ABC_lumenpyx::LumenpyxEventLoop;
use ABC_lumenpyx::RenderSettings;

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

        let slider = Slider::new(0.0, 100.0, 0.0, 100.0).with_callback(|_, value| {
            println!("Slider value: {}", value);
        });

        let slider_entity = entities_and_components.add_entity_with((slider, Transform::default()));
    }

    add_all_systems(&mut scene.world);

    lumenpyx_eventloop.run(&mut scene.world, |world| {
        world.run();

        render(&mut world.entities_and_components);
    });
}
