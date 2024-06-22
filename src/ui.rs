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
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
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
            let transform = crate::get_transform_recursive(
                entity,
                entities_and_components,
                Transform::default(),
            );

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

                if mouse_position[0] >= slider.min_position
                    && mouse_position[0] <= slider.max_position
                    && dist_from_center_y <= slider.width / 2.0
                {
                    let percentage = (mouse_position[0] - slider.min_position)
                        / (slider.max_position - slider.min_position);
                    slider.value =
                        slider.min_value + (slider.max_value - slider.min_value) * percentage;
                }
            }
        }
    }
}
