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

    let mut lumenpyx_eventloop = LumenpyxEventLoop::new(&mut scene.world, [160, 160], "UI Example");

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

        let slider = add_slider(entities_and_components);

        let button = add_button(entities_and_components);

        let content_entity = entities_and_components.add_entity_with((Transform::default(),));
        entities_and_components.set_parent(button, content_entity);
        entities_and_components.set_parent(slider, content_entity);

        add_scrollbar(entities_and_components, content_entity);
    }

    add_all_systems(&mut scene.world);

    lumenpyx_eventloop.run(&mut scene.world, |world| {
        world.run();

        render(&mut world.entities_and_components);
    });
}

fn add_slider(entities_and_components: &mut EntitiesAndComponents) -> Entity {
    let slider_rect = Rectangle::new([0.0, 1.0, 0.0, 1.0], 100.0, 4.0);

    let slider_entity =
        entities_and_components.add_entity_with((Transform::default(), slider_rect));

    let knob = Circle::new([1.0, 1.0, 1.0, 1.0], 4.0);
    let knob_entity = entities_and_components.add_entity_with((knob, Transform::default()));

    entities_and_components.set_parent(knob_entity, slider_entity);

    let mut slider = Slider::new(0.0, 100.0, -50.0, 50.0)
        .with_callback(|_, value| {
            println!("Slider value: {}", value);
        })
        .with_width(5.0);

    slider.set_knob_entity(knob_entity);

    entities_and_components.add_component_to(slider_entity, slider);

    slider_entity
}

fn add_button(entities_and_components: &mut EntitiesAndComponents) -> Entity {
    let button_rect = Rectangle::new([1.0, 0.0, 0.0, 1.0], 10.0, 10.0);

    let button_entity = entities_and_components.add_entity_with((
        Transform {
            y: 20.0,
            z: 1.0,
            ..Default::default()
        },
        button_rect,
    ));

    let mut button = ui::Button::new()
        .with_callback(|_, was_held| {
            if was_held {
                println!("Button was held");
            } else {
                println!("Button was released");
            }
        })
        .with_width(10.0)
        .with_height(10.0);

    entities_and_components.add_component_to(button_entity, button);

    button_entity
}

fn add_scrollbar(entities_and_components: &mut EntitiesAndComponents, content_entity: Entity) {
    let scrollbar_rect = Rectangle::new([0.0, 0.0, 1.0, 1.0], 4.0, 60.0);

    let scrollbar_transform = Transform {
        x: 60.0,
        y: 0.0,
        z: 1.0,
        ..Default::default()
    };

    let scrollbar_entity =
        entities_and_components.add_entity_with((scrollbar_transform, scrollbar_rect));

    let knob = Rectangle::new([1.0, 1.0, 1.0, 1.0], 4.0, 10.0);
    let knob_entity = entities_and_components.add_entity_with((knob, Transform::default()));

    entities_and_components.set_parent(knob_entity, scrollbar_entity);

    let scrollbar = ui::ScrollBar::new(-30.0, 30.0, -50.0, 50.0)
        .with_callback(|_, value| {
            println!("Scrollbar value: {}", value);
        })
        .with_width(4.0)
        .with_knob_entity(knob_entity)
        .with_content_entity(content_entity)
        .with_scroll_speed(4.0);

    entities_and_components.add_component_to(scrollbar_entity, scrollbar);
}
