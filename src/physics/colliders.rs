use crate::*;

pub enum Collider {
    Circle(CircleCollider),
    Box(BoxCollider),
}

struct CircleCollider {
    radius: f64,
}

impl From<CircleCollider> for Collider {
    fn from(circle_collider: CircleCollider) -> Self {
        Self::Circle(circle_collider)
    }
}

struct BoxCollider {
    width: f64,
    height: f64,
}

impl BoxCollider {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl From<BoxCollider> for Collider {
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
) -> (bool, f32) {
    let combined_radii = (circle_collider_1.radius + circle_collider_2.radius).powi(2);
    let distance_between_centers = (circle_collider_1.radius - circle_collider_2.radius).powi(2);

    (
        distance_between_centers <= combined_radii,
        (combined_radii - distance_between_centers) as f32,
    )
}

/// returns if there is a collision and the depth of the collision
fn circle_box_collision(
    circle_collider: &CircleCollider,
    circle_transform: &Transform,
    box_collider: &BoxCollider,
    box_transform: &Transform,
) -> (bool, f32) {
    let circle_x = circle_transform.x;
    let circle_y = circle_transform.y;
    let circle_radius = circle_collider.radius;

    let box_x = box_transform.x;
    let box_y = box_transform.y;
    let box_width = box_collider.width;
    let box_height = box_collider.height;

    let corners = [
        (box_x, box_y),
        (box_x + box_width, box_y),
        (box_x, box_y + box_height),
        (box_x + box_width, box_y + box_height),
    ];

    let mut closest_corner = corners[0];
    let mut closest_distance = f32::INFINITY;

    for corner in corners.iter() {
        let corner_x = corner.0;
        let corner_y = corner.1;

        let distance_between_centers = ((circle_x - corner_x).powi(2)
            + (circle_y - corner_y).powi(2))
        .sqrt()
        .powi(2);

        if distance_between_centers <= circle_radius.powi(2) {
            return (
                true,
                (circle_radius - distance_between_centers.sqrt()) as f32,
            );
        }
    }

    if closest_distance <= circle_radius as f32 {
        return (true, (circle_radius as f32 - closest_distance) as f32);
    } else {
        return (false, 0.0);
    }
}

// f32 for the depth of the collision
fn box_box_collision(
    box_collider_1: &BoxCollider,
    box_transform_1: &Transform,
    box_collider_2: &BoxCollider,
    box_transform_2: &Transform,
) -> (bool, f32) {
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
        // trace the line from the corner of box 1 which is colliding to the center of box 2
        // and trace the line out to the edge of box 2 then find the distance between the edge point of box 2
        // and the corner of box 1, that is our collision depth

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

            let distance_between_centers = ((box_2_x - corner_x).powi(2)
                + (box_2_y - corner_y).powi(2))
            .sqrt()
            .powi(2);

            if distance_between_centers <= closest_distance as f64 {
                closest_corner = *corner;
                closest_distance = distance_between_centers as f32;
            }
        }

        let closest_corner_x = closest_corner.0;
        let closest_corner_y = closest_corner.1;

        let (dx, dy) = (box_2_x - closest_corner_x, box_2_y - closest_corner_y);
        let slope = dy / dx;

        let (p1_x, p1_y) = (corners[0].0, corners[0].1);
        let (p2_x, p2_y) = (corners[1].0, corners[1].1);

        // desmos equation: P_{4}=-\left(\left|\left(P_{3}.x-P_{1}.x\right)+\left(P_{3}.y-P_{2}.y\right)\right|+\left|\left(P_{3}.x-P_{2}.x\right)-\left(P_{3}.y-P_{2}.y\right)\right|-\operatorname{abs}\left(P_{1}.x-P_{2}.x\right)\right)
        let intersection_depth = -(((closest_corner_x - p1_x) + (closest_corner_y - p2_y)).abs()
            + ((closest_corner_x - p2_x) + (closest_corner_y - p2_y)).abs())
            - (p1_x - p2_x).abs();

        return (true, intersection_depth as f32);
    }

    (false, 0.0)
}

pub struct ColliderSystem {}

/// A basic O(n^2) naive collision detection system, this will be replaced with a quadtree in the near future (hopefully)
/// I just wanted to get something working for now  ¯\_(ツ)_/¯
impl System for ColliderSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let entities = entities_and_components
            .get_entities_with_component::<Collider>()
            .cloned()
            .collect::<Vec<Entity>>();

        for entity_1 in entities.iter() {
            let (collider_1, transform_1) =
                entities_and_components.get_components_mut::<(Collider, Transform)>(*entity_1);

            // our use of unsafe here is sound because when we get more components we check
            // the entity id to make sure it is the same as the entity id we are currently iterating over
            let collider_1_pointer: *mut Collider = collider_1;
            let collider_1 = unsafe { &mut *collider_1_pointer };
            let transform_1_pointer: *mut Transform = transform_1;
            let transform_1 = unsafe { &mut *transform_1_pointer };

            for entity_2 in entities.iter() {
                if entity_1 == entity_2 {
                    continue;
                }

                let (collider_2, transform_2) =
                    entities_and_components.get_components_mut::<(Collider, Transform)>(*entity_2);

                check_and_resolve_collision(collider_1, transform_1, collider_2, transform_2);
            }
        }
    }
}

fn check_and_resolve_collision(
    collider_1: &Collider,
    transform_1: &mut Transform,
    collider_2: &Collider,
    transform_2: &mut Transform,
) -> bool {
    match (collider_1, collider_2) {
        (Collider::Circle(circle_collider_1), Collider::Circle(circle_collider_2)) => {
            let (is_colliding, depth) = circle_circle_collision(
                circle_collider_1,
                transform_1,
                circle_collider_2,
                transform_2,
            );

            if is_colliding {
                resolve_collision(transform_1, transform_2, depth);
                return true;
            }
            return false;
        }
        (Collider::Circle(circle_collider), Collider::Box(box_collider)) => {
            let (is_colliding, depth) =
                circle_box_collision(circle_collider, transform_1, box_collider, transform_2);

            if is_colliding {
                resolve_collision(transform_1, transform_2, depth);
                return true;
            }
            return false;
        }
        (Collider::Box(box_collider), Collider::Circle(circle_collider)) => {
            let (is_colliding, depth) =
                circle_box_collision(circle_collider, transform_1, box_collider, transform_2);

            if is_colliding {
                resolve_collision(transform_1, transform_2, depth);
                return true;
            }
            return false;
        }
        (Collider::Box(box_collider_1), Collider::Box(box_collider_2)) => {
            let (is_colliding, depth) =
                box_box_collision(box_collider_1, transform_1, box_collider_2, transform_2);

            if is_colliding {
                resolve_collision(transform_1, transform_2, depth);
                return true;
            }
            return false;
        }
    }
}

fn resolve_collision(transform_1: &mut Transform, transform_2: &mut Transform, depth: f32) {
    let transform_1_x = transform_1.x;
    let transform_1_y = transform_1.y;

    let transform_2_x = transform_2.x;
    let transform_2_y = transform_2.y;

    let transform_1_x = transform_1_x + depth as f64;
    let transform_1_y = transform_1_y + depth as f64;

    let transform_2_x = transform_2_x - depth as f64;
    let transform_2_y = transform_2_y - depth as f64;
}
