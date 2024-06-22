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

    pub fn update(
        &mut self,
        entities_and_components: &mut EntitiesAndComponents,
        transform: Transform,
    ) {
        let input = entities_and_components
            .get_resource::<Input>()
            .expect("Failed to get input");

        let mouse_position = input.get_mouse_position();
        if input.get_mouse_state(MouseButton::Left) == KeyState::Pressed {
            let dist_from_center_y = mouse_position[1];

            if mouse_position[0] >= self.min_position && mouse_position[0] <= self.max_position {
                let percentage = (mouse_position[0] - self.min_position)
                    / (self.max_position - self.min_position);
                self.value = self.min_value + (self.max_value - self.min_value) * percentage;
            }
        }
    }
}

/// just a mockup of a slider system for now
struct SliderSystem;

impl System for SliderSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let mut entities_with_slider = vec![];

        collect_all_entities_with_component_recursive::<Slider>(
            entities_and_components,
            &[],
            &mut entities_with_slider,
        );

        for entity in entities_with_slider {
            let mut slider = get_final_entities_and_components(entities_and_components, &entity)
                .get_components::<(Slider,)>(entity[entity.len() - 1])
                .0
                .clone();

            let transform =
                get_transform_recursive(&entity, entities_and_components, Transform::default());

            slider.update(entities_and_components, transform);
        }
    }
}

fn collect_all_entities_with_component_recursive<T>(
    entities_and_components: &EntitiesAndComponents,
    entity: &[Entity],
    result: &mut Vec<Vec<Entity>>,
) where
    T: Component,
{
    let entities_with_component = entities_and_components.get_entities_with_component::<T>();

    for entity_with_component in entities_with_component {
        let mut parents: Vec<Entity> = entity.iter().cloned().collect();
        parents.push(*entity_with_component);
        result.push(parents);
    }

    let entities_with_children =
        entities_and_components.get_entities_with_component::<EntitiesAndComponents>();

    for entity_with_children in entities_with_children {
        let mut parents: Vec<Entity> = entity.iter().cloned().collect();
        parents.push(*entity_with_children);

        let new_entities_and_components = entities_and_components
            .get_components::<(EntitiesAndComponents,)>(*entity_with_children)
            .0;

        collect_all_entities_with_component_recursive::<T>(
            new_entities_and_components,
            &parents,
            result,
        );
    }
}

fn get_transform_recursive(
    entity: &[Entity],
    entities_and_components: &EntitiesAndComponents,
    offset: Transform,
) -> Transform {
    if entity.is_empty() {
        // should never happen but just in case
        return offset;
    }

    let transform = entities_and_components
        .get_components::<(Transform,)>(entity[0])
        .0
        .clone();

    let offset = &offset + &transform;

    if let Some(parent) = entities_and_components
        .try_get_components::<(EntitiesAndComponents,)>(entity[0])
        .0
    {
        let new_entity = entity[1..].to_vec();
        get_transform_recursive(&new_entity, parent, offset)
    } else {
        offset
    }
}

fn get_final_entities_and_components<'a>(
    entities_and_components: &'a mut EntitiesAndComponents,
    entity: &[Entity],
) -> &'a mut EntitiesAndComponents {
    if entity.is_empty() {
        return entities_and_components;
    }

    let parent = entities_and_components
        .get_components_mut::<(EntitiesAndComponents,)>(entity[0])
        .0;

    get_final_entities_and_components(parent, &entity[1..])
}
