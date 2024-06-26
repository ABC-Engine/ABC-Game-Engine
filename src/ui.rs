use ABC_ECS::World;

use crate::input::*;
use crate::Component;
use crate::System;
use crate::Transform;
use crate::{EntitiesAndComponents, Entity};

#[derive(Clone)]
/// A slider is a UI element that can be dragged to change a value
pub struct Slider {
    min_value: f32,
    max_value: f32,
    min_position: f32,
    max_position: f32,
    // the width of the clickable area
    width: f32,
    value: f32,
    callback: fn(&mut EntitiesAndComponents, f32),
    value_changed: bool,
    knob_entity: Option<Entity>,
    mouse_was_held: bool,
}

impl Slider {
    pub fn new(min_value: f32, max_value: f32, min_position: f32, max_position: f32) -> Self {
        Slider {
            min_value,
            max_value,
            min_position,
            max_position,
            width: max_position - min_position,
            value: min_value,
            callback: |_, _| {},
            value_changed: false,
            knob_entity: None,
            mouse_was_held: false,
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    /// sets the value of the slider, clamped between min_value and max_value
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min_value, self.max_value);

        self.value_changed = true;
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn set_callback(&mut self, callback: fn(&mut EntitiesAndComponents, f32)) {
        self.callback = callback;
    }

    pub fn with_callback(mut self, callback: fn(&mut EntitiesAndComponents, f32)) -> Self {
        self.callback = callback;
        self
    }

    pub fn get_callback(&self) -> fn(&mut EntitiesAndComponents, f32) {
        self.callback
    }

    pub fn get_min_value(&self) -> f32 {
        self.min_value
    }

    pub fn get_max_value(&self) -> f32 {
        self.max_value
    }

    pub fn get_min_position(&self) -> f32 {
        self.min_position
    }

    pub fn get_max_position(&self) -> f32 {
        self.max_position
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn set_min_value(&mut self, min_value: f32) {
        self.min_value = min_value;
    }

    pub fn set_max_value(&mut self, max_value: f32) {
        self.max_value = max_value;
    }

    pub fn set_min_position(&mut self, min_position: f32) {
        self.min_position = min_position;
    }

    pub fn set_max_position(&mut self, max_position: f32) {
        self.max_position = max_position;
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn with_min_value(mut self, min_value: f32) -> Self {
        self.min_value = min_value;
        self
    }

    pub fn with_max_value(mut self, max_value: f32) -> Self {
        self.max_value = max_value;
        self
    }

    pub fn with_min_position(mut self, min_position: f32) -> Self {
        self.min_position = min_position;
        self
    }

    pub fn with_max_position(mut self, max_position: f32) -> Self {
        self.max_position = max_position;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_knob_entity(mut self, knob_entity: Entity) -> Self {
        self.knob_entity = Some(knob_entity);
        self
    }

    pub fn get_knob_entity(&self) -> Option<Entity> {
        self.knob_entity
    }

    pub fn set_knob_entity(&mut self, knob_entity: Entity) {
        self.knob_entity = Some(knob_entity);
    }
}

/// just a mockup of a slider system for now
struct SliderSystem;

impl System for SliderSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let entities_with_slider = entities_and_components
            .get_entities_with_component::<Slider>()
            .cloned()
            .collect::<Vec<Entity>>();

        for entity in entities_with_slider {
            let transform = crate::get_transform(entity, entities_and_components);

            let input = entities_and_components
                .get_resource::<Input>()
                .expect("Failed to get input");

            let mouse_position = input.get_mouse_position();
            let is_held = input.get_mouse_state(MouseButton::Left) == KeyState::Held
                || input.get_mouse_state(MouseButton::Left) == KeyState::Pressed;

            let slider = entities_and_components
                .get_components_mut::<(Slider,)>(entity)
                .0;

            if is_held {
                let dist_from_center_y = (mouse_position[1] - transform.y as f32).abs();

                if (mouse_position[0] >= slider.min_position
                    && mouse_position[0] <= slider.max_position
                    && dist_from_center_y <= slider.width / 2.0)
                    || slider.mouse_was_held
                {
                    let percentage = (mouse_position[0] - slider.min_position)
                        / (slider.max_position - slider.min_position);

                    slider.set_value(
                        slider.min_value + (slider.max_value - slider.min_value) * percentage,
                    );

                    slider.mouse_was_held = true;
                }
            } else {
                slider.mouse_was_held = false;
            }

            if slider.value_changed {
                slider.value_changed = false;
                let callback = slider.callback.clone();
                let value = slider.value;
                (callback)(entities_and_components, value);
            }

            let slider = entities_and_components
                .get_components::<(Slider,)>(entity)
                .0
                .clone();

            let knob_entity = slider.knob_entity.clone();
            let min_position = slider.min_position;
            let max_position = slider.max_position;

            if let Some(knob_entity) = knob_entity {
                let knob_x = min_position
                    + (max_position - min_position)
                        * ((slider.value - slider.min_value)
                            / (slider.max_value - slider.min_value));

                entities_and_components
                    .try_get_components_mut::<(Transform,)>(knob_entity)
                    .0
                    .expect("Failed to get knob transform")
                    .x = knob_x as f64;
            }
        }
    }
}

pub(crate) fn add_all_ui_systems(world: &mut World) {
    // remove all ui systems to prevent duplicates
    world.remove_all_systems_of_type::<SliderSystem>();

    world.add_system(SliderSystem {});
}
