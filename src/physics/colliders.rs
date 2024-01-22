use glam::Vec2;

use crate::*;

use super::rigidbody::{self, RigidBody};

#[derive(Clone, Copy)]
pub struct Collider {
    pub shape: ColliderShape,
    pub properties: ColliderProperties,
}

impl Collider {
    pub fn new(shape: ColliderShape, properties: ColliderProperties) -> Self {
        Self { shape, properties }
    }
}

#[derive(Clone, Copy)]
pub struct ColliderProperties {
    is_static: bool,
}

impl ColliderProperties {
    pub fn new(is_static: bool) -> Self {
        Self { is_static }
    }
    pub fn default() -> Self {
        Self { is_static: false }
    }
}

#[derive(Clone, Copy)]
pub enum ColliderShape {
    Circle(CircleCollider),
    Box(BoxCollider),
}

#[derive(Clone, Copy)]
pub struct CircleCollider {
    radius: f64,
}

impl CircleCollider {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl From<CircleCollider> for ColliderShape {
    fn from(circle_collider: CircleCollider) -> Self {
        Self::Circle(circle_collider)
    }
}

#[derive(Clone, Copy)]
pub struct BoxCollider {
    width: f64,
    height: f64,
}

impl BoxCollider {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl From<BoxCollider> for ColliderShape {
    fn from(box_collider: BoxCollider) -> Self {
        Self::Box(box_collider)
    }
}

// I am not sure if there is a way to generically implement this for all types that implement Collider
// Because, the number of functions needed to implement this would be n^2, where n is the number of types that implement Collider

fn circle_circle_collision(
    circle_collider_1: &CircleCollider,
    circle_1_transform: &mut Transform,
    circle_collider_2: &CircleCollider,
    circle_2_transform: &mut Transform,
) -> (bool, [f64; 2]) {
    let combined_radii = circle_collider_1.radius + circle_collider_2.radius;
    let distance_between_centers = ((circle_1_transform.x - circle_2_transform.x).powi(2)
        + (circle_1_transform.y - circle_2_transform.y).powi(2))
    .sqrt();

    let normalized_vector = [
        (circle_1_transform.x - circle_2_transform.x) / distance_between_centers,
        (circle_1_transform.y - circle_2_transform.y) / distance_between_centers,
    ];

    let magnitude = distance_between_centers - combined_radii;

    let force_vector = [
        -normalized_vector[0] * magnitude,
        -normalized_vector[1] * magnitude,
    ];

    (distance_between_centers <= combined_radii, force_vector)
}

/// returns if there is a collision and the depth of the collision
fn circle_box_collision(
    circle_collider: &CircleCollider,
    circle_transform: &Transform,
    box_collider: &BoxCollider,
    box_transform: &Transform,
) -> (bool, [f64; 2]) {
    let circle_x = circle_transform.x;
    let circle_y = circle_transform.y;
    let circle_radius = circle_collider.radius;

    let box_x = box_transform.x;
    let box_y = box_transform.y;
    let box_width = box_collider.width;
    let box_height = box_collider.height;

    // I believe this is how I set up the corners to render
    let closest_x = circle_x.clamp(box_x - box_width / 2.0, box_x + box_width / 2.0);
    let closest_y = circle_y.clamp(box_y - box_height / 2.0, box_y + box_height / 2.0);

    if closest_x == circle_x && closest_y == circle_y {
        // the circle center is inside the box i'll work out the exact depth later
        // Placeholder for now, it shouldn't really happen anyway
        return (true, [circle_radius, circle_radius]);
    } else {
        // calculate the distance between the center of the circle and the closest point on the box
        let distance_between_centers =
            ((circle_x - closest_x).powi(2) + (circle_y - closest_y).powi(2)).sqrt();

        if distance_between_centers <= circle_radius {
            let vector = [(closest_x - circle_x), (closest_y - circle_y)];
            let normalized_vector = [
                vector[0] / distance_between_centers,
                vector[1] / distance_between_centers,
            ];

            let force_magnitude = distance_between_centers - circle_radius;
            let force_vector = [
                normalized_vector[0] * force_magnitude,
                normalized_vector[1] * force_magnitude,
            ];
            return (true, force_vector);
        } else {
            return (false, [0.0, 0.0]);
        }
    }
}

// f32 for the depth of the collision
fn box_box_collision(
    box_collider_1: &BoxCollider,
    box_transform_1: &Transform,
    box_collider_2: &BoxCollider,
    box_transform_2: &Transform,
) -> (bool, [f64; 2]) {
    let box_1_x = box_transform_1.x;
    let box_1_y = box_transform_1.y;
    let box_1_width = box_collider_1.width;
    let box_1_height = box_collider_1.height;

    let box_2_x = box_transform_2.x;
    let box_2_y = box_transform_2.y;
    let box_2_width = box_collider_2.width;
    let box_2_height = box_collider_2.height;

    if box_1_x < box_2_x + box_2_width
        && box_1_x + box_1_width > box_2_x
        && box_1_y < box_2_y + box_2_height
        && box_1_y + box_1_height > box_2_y
    {
        let corners = [
            (box_1_x, box_1_y),
            (box_1_x + box_1_width, box_1_y),
            (box_1_x, box_1_y + box_1_height),
            (box_1_x + box_1_width, box_1_y + box_1_height),
        ];

        let mut closest_corner = corners[0];
        let mut closest_distance = f32::INFINITY;

        for corner in corners.iter() {
            let corner_x = corner.0;
            let corner_y = corner.1;

            let distance_between_centers =
                ((box_2_x - corner_x).powi(2) + (box_2_y - corner_y).powi(2)).sqrt();

            if distance_between_centers <= closest_distance as f64 {
                closest_corner = *corner;
                closest_distance = distance_between_centers as f32;
            }
        }

        let closest_corner_x = closest_corner.0;
        let closest_corner_y = closest_corner.1;

        let distance_to_positive_bounds = [
            box_2_x + box_2_width - closest_corner_x,
            box_2_y + box_2_width - closest_corner_y,
        ];

        let distance_to_negative_bounds = [
            -(box_2_x - box_2_width - closest_corner_x),
            -(box_2_y - box_2_width - closest_corner_y),
        ];

        let smallest_x = distance_to_positive_bounds[0].min(distance_to_negative_bounds[0]);
        let smallest_y = distance_to_positive_bounds[1].min(distance_to_negative_bounds[1]);
        let smallest_distance = smallest_x.min(smallest_y);

        let closest_point: [f64; 2];
        if smallest_distance == distance_to_positive_bounds[0] {
            closest_point = [box_2_x + box_2_width, closest_corner_y];
        } else if smallest_distance == distance_to_negative_bounds[0] {
            closest_point = [box_2_x - box_2_width, closest_corner_y];
        } else if smallest_distance == distance_to_positive_bounds[1] {
            closest_point = [closest_corner_x, box_2_y + box_2_height];
        } else {
            closest_point = [closest_corner_x, box_2_y - box_2_height];
        }

        let vec_to_move = [
            closest_point[0] - closest_corner_x,
            closest_point[1] - closest_corner_y,
        ];

        return (true, vec_to_move);
    }

    (false, [0.0, 0.0])
}

pub struct CollisionSystem {}

/// A basic O(n^2) naive collision detection system, this will be replaced with a quadtree in the near future (hopefully)
/// I just wanted to get something working for now  ¯\_(ツ)_/¯
impl System for CollisionSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let entities = entities_and_components
            .get_entities_with_component::<Collider>()
            .cloned()
            .collect::<Vec<Entity>>();

        for entity_1 in entities.iter() {
            let (collider_1, transform_1, rigidbody_1) = entities_and_components
                .try_get_components_mut::<(Collider, Transform, RigidBody)>(*entity_1);

            // if the first entity doesn't have a rigidbody, it is static and we don't need to check for collisions as they will be handled by the other entity

            match (collider_1, transform_1, rigidbody_1) {
                (Some(collider_1), Some(transform_1), Some(rigidbody_1)) => {
                    // our use of unsafe here is sound because when we get more components we check
                    // the entity id to make sure it is the same as the entity id we are currently iterating over
                    let collider_1_pointer: *mut Collider = collider_1;
                    let collider_1 = unsafe { &mut *collider_1_pointer };
                    let transform_1_pointer: *mut Transform = transform_1;
                    let transform_1 = unsafe { &mut *transform_1_pointer };
                    let rigidbody_1_pointer: *mut RigidBody = rigidbody_1;
                    let rigidbody_1 = unsafe { &mut *rigidbody_1_pointer };

                    for entity_2 in entities.iter() {
                        if entity_1 == entity_2 {
                            continue;
                        }

                        match (entities_and_components
                            .try_get_components_mut::<(Collider, Transform, RigidBody)>(*entity_2))
                        {
                            (Some(collider_2), Some(transform_2), Some(rigidbody_2)) => {
                                check_and_resolve_collision(
                                    collider_1,
                                    transform_1,
                                    rigidbody_1,
                                    collider_2,
                                    transform_2,
                                    rigidbody_2,
                                );
                            }
                            (Some(collider_2), Some(transform_2), None) => {
                                check_and_resolve_collision(
                                    collider_1,
                                    transform_1,
                                    rigidbody_1,
                                    collider_2,
                                    transform_2,
                                    &mut rigidbody::RigidBody::default(),
                                );
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

// there is a problem with this for now, I believe each collision is being resolved twice, once for each entity
fn check_and_resolve_collision(
    collider_1: &Collider,
    transform_1: &mut Transform,
    rb_1: &mut RigidBody,
    collider_2: &Collider,
    transform_2: &mut Transform,
    rb_2: &mut RigidBody,
) -> bool {
    if collider_1.properties.is_static && collider_2.properties.is_static {
        return false;
    }

    let force_vec: [f64; 2];
    let is_colliding: bool;

    match (&collider_1.shape, &collider_2.shape) {
        (ColliderShape::Circle(circle_collider_1), ColliderShape::Circle(circle_collider_2)) => {
            (is_colliding, force_vec) = circle_circle_collision(
                circle_collider_1,
                transform_1,
                circle_collider_2,
                transform_2,
            );
        }
        (ColliderShape::Circle(circle_collider), ColliderShape::Box(box_collider)) => {
            (is_colliding, force_vec) =
                circle_box_collision(circle_collider, transform_1, box_collider, transform_2);
        }
        (ColliderShape::Box(box_collider), ColliderShape::Circle(circle_collider)) => {
            (is_colliding, force_vec) =
                circle_box_collision(circle_collider, transform_1, box_collider, transform_2);
        }
        (ColliderShape::Box(box_collider_1), ColliderShape::Box(box_collider_2)) => {
            (is_colliding, force_vec) =
                box_box_collision(box_collider_1, transform_1, box_collider_2, transform_2);
        }
    }

    if is_colliding {
        resolve_collision(
            transform_1,
            &collider_1.properties,
            rb_1,
            transform_2,
            &collider_2.properties,
            rb_2,
            force_vec,
        );
        return true;
    }
    return false;
}

fn resolve_collision(
    transform_1: &mut Transform,
    collision_properties_1: &ColliderProperties,
    rb_1: &mut RigidBody,
    transform_2: &mut Transform,
    collision_properties_2: &ColliderProperties,
    rb_2: &mut RigidBody,
    // this will be a vector in which direction the object should move
    // it will also store the depth of the collision
    force_vector: [f64; 2],
) {
    // should never happen, so not sure if it is worth checking for
    if force_vector[0] == 0.0 && force_vector[1] == 0.0 {
        return;
    }

    // for now we are going to push the objects apart by half the depth of the collision
    // this is not a perfect solution, but it is good enough for now, in the future I think it should be proportional to the mass of the objects
    if collision_properties_1.is_static {
        handle_static_collision(transform_2, rb_2, &[-force_vector[0], -force_vector[1]])
    } else if collision_properties_2.is_static {
        handle_static_collision(transform_1, rb_1, &force_vector)
    } else {
        handle_non_static_collsion(transform_1, rb_1, transform_2, rb_2, &force_vector);
    }
}

fn handle_static_collision(
    transform_1: &mut Transform,
    rb_1: &mut RigidBody,
    force_vector: &[f64; 2],
) {
    transform_1.x += force_vector[0];
    transform_1.y += force_vector[1];

    // time for some elastic collisions
    // I think I'm using the correct formula here but, correct me if I'm wrong
    // source for the formula: https://phys.libretexts.org/Bookshelves/University_Physics/Mechanics_and_Relativity_(Idema)/04%3A_Momentum/4.07%3A_Totally_Elastic_Collisions
    let velocity_1 = rb_1.get_velocity();

    // final_velocity_1 = ((mass_1 - mass_2) / total_mass) * total_velocity_1 as f64;
    // as mass_2 -> infinity, ((mass_1 - mass_2) / total_mass) approaches -1
    let mut final_velocity_1 = -velocity_1;

    // now lets adapt the direction of the velocity
    // we are going to blend the direction of the velocity and the direction of the force vector based on the momentum of the object
    // this is to prevent the object from not changing direction when it should, like when two balls hit each other at an angle and
    // don't gain any horizontal velocity

    let velocity_1_magnitude = final_velocity_1.length();
    final_velocity_1 = Vec2::new(force_vector[0] as f32, force_vector[1] as f32).normalize();
    final_velocity_1 *= velocity_1_magnitude;

    rb_1.set_velocity(final_velocity_1 * rb_1.get_elasticity());
}

fn handle_non_static_collsion(
    transform_1: &mut Transform,
    rb_1: &mut RigidBody,
    transform_2: &mut Transform,
    rb_2: &mut RigidBody,
    force_vector: &[f64; 2],
) {
    let mass_1 = rb_1.get_mass() as f64;
    let mass_2 = rb_2.get_mass() as f64;
    let total_mass = mass_1 + mass_2;
    let mass_percentage_1 = mass_1 / total_mass;
    let mass_percentage_2 = mass_2 / total_mass;

    transform_1.x += force_vector[0] * mass_percentage_2;
    transform_1.y += force_vector[1] * mass_percentage_2;

    transform_2.x -= force_vector[0] * mass_percentage_1;
    transform_2.y -= force_vector[1] * mass_percentage_1;

    // time for some elastic collisions
    // I think I'm using the correct formula here but, correct me if I'm wrong
    let velocity_1 = rb_1.get_velocity();
    let velocity_2 = rb_2.get_velocity();

    let momentum_1 = velocity_1 * mass_1 as f32;
    let momentum_2 = velocity_2 * mass_2 as f32;

    let mut final_velocity_1 =
        (mass_2 as f32 * (velocity_2 - velocity_1) + momentum_1 + momentum_2)
            / (mass_1 + mass_2) as f32;

    let mut final_velocity_2 =
        (mass_1 as f32 * (velocity_1 - velocity_2) + momentum_1 + momentum_2)
            / (mass_1 + mass_2) as f32;

    // now lets adapt the direction of the velocity
    // we are going to blend the direction of the velocity and the direction of the force vector based on the momentum of the object
    // this is to prevent the object from not changing direction when it should, like when two balls hit each other at an angle and
    // don't gain any horizontal velocity

    let velocity_1_magnitude = final_velocity_1.length();
    let velocity_2_magnitude = final_velocity_1.length();
    let momentum_1 = velocity_1_magnitude * mass_1 as f32;
    let momentum_2 = velocity_2_magnitude * mass_2 as f32;
    let momentum_percentage_1 = momentum_1 / (momentum_1 + momentum_2);
    let momentum_percentage_2 = momentum_2 / (momentum_1 + momentum_2);

    // lerp the velocity and the force vector with t = momentum_percentage
    // first we need to normalize both vectors
    let mut force_vector = Vec2::new(force_vector[0] as f32, force_vector[1] as f32);
    force_vector = force_vector.normalize();

    final_velocity_1 = final_velocity_1.normalize();
    final_velocity_2 = final_velocity_2.normalize();

    // the lower the momentum percentage, the more the velocity will be changed
    final_velocity_1 =
        (1.0 - momentum_percentage_1) * force_vector + momentum_percentage_1 * final_velocity_1;
    final_velocity_1 = final_velocity_1.normalize();
    final_velocity_1 *= velocity_1_magnitude;

    final_velocity_2 =
        (1.0 - momentum_percentage_2) * -force_vector + momentum_percentage_2 * final_velocity_2;
    final_velocity_2 = final_velocity_2.normalize();
    final_velocity_2 *= velocity_2_magnitude;

    let average_elasticity = (rb_1.get_elasticity() + rb_2.get_elasticity()) / 2.0;

    // apply the elasticity to the difference in velocity
    // I think this is the correct way to do this, I did it this way because otherwise when two balls collide
    // they would both loose almost all of their velocity no matter the direction of the collision
    // one example is if two balls are both falling and are both on the same x position, they will both loose almost all of their velocity, even the bottom ball which should gain velocity
    final_velocity_1 = velocity_1 - average_elasticity * (velocity_1 - final_velocity_1);
    final_velocity_2 = velocity_2 - average_elasticity * (velocity_2 - final_velocity_2);

    // mix the current velocity with the final velocity based on the elasticity of the object
    // I have no idea if this is the correct way to do this, but it seems to work
    rb_1.set_velocity(final_velocity_1);
    rb_2.set_velocity(final_velocity_2);
}
