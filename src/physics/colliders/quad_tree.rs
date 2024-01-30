/// this is a work in progress, it does not yet work in it's entirety
use std::panic;

use crate::physics::colliders::Collider;
use crate::physics::colliders::ColliderShape;
use crate::physics::colliders::ColliderShape::Circle;
use crate::physics::colliders::{BoxCollider, CircleCollider};
use crate::Transform;

pub fn collider_to_quad_tree_range(
    collider: &Collider,
    transform: &Transform,
) -> Box<dyn QuadTreeRange> {
    match &collider.shape {
        Circle(CircleCollider { radius }) => Box::new(QuadTreeRangeCircle {
            center: *transform,
            radius: *radius,
        }),
        ColliderShape::Box(BoxCollider { width, height }) => Box::new(QuadTreeRangeRectangle {
            center: Transform {
                x: transform.x,
                y: transform.y,
                ..Transform::default()
            },
            width: *width,
            height: *height,
        }),
    }
}

pub struct QuadTreeObject<'a, T> {
    object: &'a T,
    transform: Transform,
    query_range: Box<dyn QuadTreeRange>,
}

impl<'a, T> QuadTreeObject<'a, T> {
    pub fn new(object: &'a T, transform: Transform, query_range: Box<dyn QuadTreeRange>) -> Self {
        Self {
            object,
            transform,
            query_range,
        }
    }

    pub fn get_object(&self) -> &T {
        self.object
    }
}

struct Leaf<'a, T> {
    objects: Vec<QuadTreeObject<'a, T>>,
}

impl<'a, T> Leaf<'a, T> {
    fn new(object: QuadTreeObject<'a, T>) -> Self {
        Self {
            objects: vec![object],
        }
    }
}

/// QuadTreeBranch is a branch of the quad tree, it can either be 4 children nodes or a leaf node
enum QuadTreeBranch<'a, T> {
    // lower left, upper left, lower right, upper right as indexed in the enum
    Node(Box<[QuadTreeNode<'a, T>; 4]>),
    Leaf(Leaf<'a, T>),
}

struct QuadTreeRect {
    pos: [f64; 2],
    width: f64,
    height: f64,
}

impl QuadTreeRect {
    fn new(pos: [f64; 2], width: f64, height: f64) -> Self {
        Self { pos, width, height }
    }

    fn contains_point(&self, point: &Transform) -> bool {
        let x = point.x - self.pos[0];
        let y = point.y - self.pos[1];

        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;

        x >= -half_width && x <= half_width && y >= -half_height && y <= half_height
    }

    /// this function checks if the range intersects with the current rectangle
    fn could_intersect_point(&self, point: &Transform) -> bool {
        // here we double the width and height of the range, this way one of the two rectangles will always be inside the other

        let dx = point.x - self.pos[0];
        let dy = point.y - self.pos[1];

        dx >= -self.width && dx <= self.width && dy >= -self.height && dy <= self.height
    }

    fn get_double_rect(&self) -> Self {
        Self {
            pos: [self.pos[0], self.pos[1]],
            width: self.width * 2.0,
            height: self.height * 2.0,
        }
    }
}

pub trait QuadTreeRange {
    fn contains_point(&self, point: &Transform) -> bool;
    // this rect will contain all of the points that are in the range, but not necessarily the other way around
    fn get_rect(&self) -> QuadTreeRect;
}

#[derive(Clone, Copy)]
struct QuadTreeRangeCircle {
    center: Transform,
    radius: f64,
}

impl QuadTreeRange for QuadTreeRangeCircle {
    fn contains_point(&self, point: &Transform) -> bool {
        let distance = self.center.squared_distance_to(point);
        distance <= self.radius.powi(2)
    }

    fn get_rect(&self) -> QuadTreeRect {
        QuadTreeRect::new(
            [self.center.x, self.center.y],
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }
}

struct QuadTreeRangeRectangle {
    center: Transform,
    width: f64,
    height: f64,
}

impl QuadTreeRange for QuadTreeRangeRectangle {
    fn contains_point(&self, point: &Transform) -> bool {
        let x = point.x - self.center.x;
        let y = point.y - self.center.y;

        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;

        x >= -half_width && x <= half_width && y >= -half_height && y <= half_height
    }

    fn get_rect(&self) -> QuadTreeRect {
        QuadTreeRect::new([self.center.x, self.center.y], self.width, self.height)
    }
}

struct QuadTreeNode<'a, T> {
    // lower left corner
    pos: [f64; 2],
    width: f64,
    child: Option<QuadTreeBranch<'a, T>>,
    max_leaf_objects: usize,
    // this being None means that there is no max depth, not that the depth is 0
    depth_left: Option<u32>,
}

impl<'a, T> QuadTreeNode<'a, T> {
    fn new(pos: [f64; 2], width: f64, depth_left: Option<u32>) -> Self {
        Self {
            pos,
            width,
            child: Some(QuadTreeBranch::Leaf(Leaf {
                objects: Vec::new(),
            })),
            max_leaf_objects: 1,
            depth_left: depth_left,
        }
    }

    fn insert(&mut self, object: QuadTreeObject<'a, T>) {
        match self.child {
            Some(QuadTreeBranch::Node(ref mut nodes)) => {
                let quadrant = find_quadrant(self.pos, self.width, &object);

                let child = &mut nodes[quadrant as usize];
                child.insert(object);
            }
            Some(QuadTreeBranch::Leaf(ref mut leaf)) => {
                if leaf.objects.len() < self.max_leaf_objects
                    || !(self.depth_left.is_some() && self.depth_left.unwrap() > 0)
                {
                    leaf.objects.push(object);
                    return;
                } else {
                    // this is currently a leaf node, we need to split the node
                    let mut objects = leaf.objects.drain(..).collect::<Vec<_>>();
                    objects.push(object);

                    self.add_children_nodes();
                    self.bulk_insert(objects);
                }
            }
            None => {
                self.child = Some(QuadTreeBranch::Leaf(Leaf::new(object)));
            }
        }
    }

    fn bulk_insert(&mut self, objects: Vec<QuadTreeObject<'a, T>>) {
        // TODO: this is a naive implementation, we should be able to do this in parallel
        // but.. I'm not sure how to do that with the current structure
        for object in objects {
            self.insert(object);
        }
    }

    fn is_empty(&self) -> bool {
        self.child.is_none()
    }

    /// just for testing purposes for now
    fn find_closest_object_to_point(
        &self,
        object: &QuadTreeObject<T>,
    ) -> Option<&QuadTreeObject<T>> {
        match self.child {
            Some(QuadTreeBranch::Node(ref nodes)) => {
                let quadrant = find_quadrant(self.pos, self.width, object);

                let child = &nodes[quadrant as usize];
                child.find_closest_object_to_point(object)
            }
            Some(QuadTreeBranch::Leaf(ref leaf)) => {
                // temporary implementation, just return the first object
                let mut closest_index = 0;
                let mut closest_distance = f64::INFINITY;
                for (i, object) in (&leaf.objects).into_iter().enumerate() {
                    let distance = object.transform.squared_distance_to(&object.transform);
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_index = i;
                    }
                }
                return Some(&leaf.objects[closest_index]);
            }
            None => {
                return None;
            }
        }
    }

    fn add_children_nodes(&mut self) {
        let new_width = self.width / 2.0;
        let new_depth = self.depth_left.map(|depth| depth - 1);

        let children_nodes = [
            QuadTreeNode::new([self.pos[0], self.pos[1] + new_width], new_width, new_depth),
            QuadTreeNode::new([self.pos[0], self.pos[1]], new_width, new_depth),
            QuadTreeNode::new(
                [self.pos[0] + new_width, self.pos[1] + new_width],
                new_width,
                new_depth,
            ),
            QuadTreeNode::new([self.pos[0] + new_width, self.pos[1]], new_width, new_depth),
        ];

        self.child = Some(QuadTreeBranch::Node(Box::new(children_nodes)));
    }

    fn could_fit_point(&self, object: &QuadTreeObject<T>) -> bool {
        let object_center = object.transform;

        let right = object_center.x >= self.pos[0] + self.width;
        let left = object_center.x <= self.pos[0];
        let top = object_center.y >= self.pos[1] + self.width;
        let bottom = object_center.y <= self.pos[1];

        !(left || right || top || bottom)
    }

    fn query_range(&self, range: &dyn QuadTreeRange) -> Vec<&QuadTreeObject<T>> {
        if !self.rect_intersect(&range.get_rect().get_double_rect()) {
            // TODO: FIX THIS
            // without this nothing is wrong

            //return vec![];
        }
        let mut objects = vec![];

        if let Some(ref child) = self.child {
            // check if the range intersects with the current node
            match child {
                QuadTreeBranch::Node(nodes) => {
                    for node in &**nodes {
                        objects.extend(node.query_range(range));
                    }
                }
                QuadTreeBranch::Leaf(leaf) => {
                    for object in &leaf.objects {
                        if range.get_rect().could_intersect_point(&object.transform) {
                            objects.push(object);
                        }
                    }
                }
            }
        }

        objects
    }

    fn query_range_mut(&mut self, range: &dyn QuadTreeRange) -> Vec<&'a mut QuadTreeObject<T>> {
        if !self.rect_intersect(&range.get_rect().get_double_rect()) {
            return vec![];
        }
        let mut objects = vec![];

        if let Some(ref mut child) = self.child {
            // check if the range intersects with the current node
            match child {
                QuadTreeBranch::Node(nodes) => {
                    for node in &mut **nodes {
                        objects.extend(node.query_range_mut(range));
                    }
                }
                QuadTreeBranch::Leaf(leaf) => {
                    for object in &mut leaf.objects {
                        if range.get_rect().could_intersect_point(&object.transform) {
                            objects.push(object);
                        }
                    }
                }
            }
        }

        objects
    }

    fn collect_objects(&self) -> Vec<&QuadTreeObject<T>> {
        let mut objects = vec![];

        if let Some(ref child) = self.child {
            match child {
                QuadTreeBranch::Node(nodes) => {
                    for node in &**nodes {
                        objects.extend(node.collect_objects());
                    }
                }
                QuadTreeBranch::Leaf(leaf) => {
                    for object in &leaf.objects {
                        objects.push(object);
                    }
                }
            }
        }

        objects
    }

    fn rect_intersect(&self, rect: &QuadTreeRect) -> bool {
        let dx = rect.pos[0] - self.pos[0];
        let dy = rect.pos[1] - self.pos[1];

        let combined_half_width = (self.width + rect.width) / 2.0;
        let combined_half_height = (self.width + rect.height) / 2.0;

        dx.abs() <= combined_half_width && dy.abs() <= combined_half_height
    }
}

pub struct QuadTree<'a, T> {
    root: QuadTreeNode<'a, T>,
}

impl<'a, T> QuadTree<'a, T> {
    pub fn new(pos: [f64; 2], width: f64, max_depth: Option<u32>) -> Self {
        Self {
            root: QuadTreeNode {
                pos,
                width,
                child: None,
                max_leaf_objects: 1,
                depth_left: max_depth,
            },
        }
    }

    /// this function will panic if the object does not fit in the quad tree
    pub fn insert(&mut self, object: QuadTreeObject<'a, T>) {
        if !self.root.could_fit_point(&object) {
            panic!("object does not fit in the quad tree: user error");
        }

        self.root.insert(object);
    }

    /// This function will return false if the object does not fit in the quad tree
    pub fn try_insert(&mut self, object: QuadTreeObject<'a, T>) -> bool {
        if !self.root.could_fit_point(&object) {
            return false;
        }

        self.root.insert(object);
        true
    }

    pub fn bulk_insert(&mut self, objects: Vec<QuadTreeObject<'a, T>>) {
        for object in &objects {
            if !self.root.could_fit_point(object) {
                panic!("object does not fit in the quad tree: user error");
            }
        }
        self.root.bulk_insert(objects);
    }

    pub fn find_closest_object_to_point(
        &self,
        object: &QuadTreeObject<T>,
    ) -> Option<&QuadTreeObject<T>> {
        self.root.find_closest_object_to_point(object)
    }

    pub fn find_possible_collisions(&self) -> Vec<[&QuadTreeObject<T>; 2]> {
        let mut collisions = vec![];

        let objects = self.root.collect_objects();

        for object in objects {
            let range = object.query_range.as_ref();
            let objects_in_range = self.root.query_range(range);

            for object_in_range in objects_in_range {
                if object_in_range.transform == object.transform {
                    continue;
                }

                collisions.push([object, object_in_range]);
            }
        }

        collisions
    }
}

#[derive(Debug, PartialEq)]
enum Quadrant {
    LowerLeft,
    UpperLeft,
    LowerRight,
    UpperRight,
}

fn find_quadrant<T>(pos: [f64; 2], width: f64, object: &QuadTreeObject<T>) -> Quadrant {
    let center = [pos[0] + width / 2.0, pos[1] + width / 2.0];
    let object_center = object.transform;

    let left = object_center.x <= center[0];
    let top = object_center.y >= center[1];

    if left {
        if top {
            Quadrant::UpperLeft
        } else {
            Quadrant::LowerLeft
        }
    } else {
        if top {
            Quadrant::UpperRight
        } else {
            Quadrant::LowerRight
        }
    }
}

mod tests {
    use rand::random;

    use super::*;
    use crate::physics::colliders::{CircleCollider, ColliderProperties};
    use rand::Rng;

    #[test]
    fn test_insert() {
        let mut quad_tree = QuadTree::new([0.0, 0.0], 100.0, Some(100));

        let mut objects = vec![];
        let mut objects_to_search = vec![];

        let collider = Collider {
            shape: CircleCollider { radius: 10.0 }.into(),
            properties: ColliderProperties::default(),
        };
        let query_range = QuadTreeRangeCircle {
            center: Transform {
                x: 0.0,
                y: 0.0,
                ..Transform::default()
            },
            radius: 10.0,
        };
        for _ in 0..10000 {
            let mut rng = rand::thread_rng();
            let random_x = rng.gen::<f64>() * 100.0;
            let random_y = rng.gen::<f64>() * 100.0;

            let object = QuadTreeObject {
                object: &collider,
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
                query_range: Box::new(query_range),
            };

            let object_to_search = QuadTreeObject {
                object: &collider,
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
                query_range: Box::new(query_range),
            };

            objects.push(object);
            objects_to_search.push(object_to_search);
        }

        quad_tree.root.bulk_insert(objects);

        // test the find closest object to point
        for object in objects_to_search {
            if let Some(closest_object) = quad_tree.find_closest_object_to_point(&object) {
                let closest_transform = closest_object.transform;
                if closest_transform.x != object.transform.x
                    || closest_transform.y != object.transform.y
                {
                    panic!(
                        "expected object at [{:.2}, {:.2}] found object at [{:.2}, {:.2}]",
                        object.transform.x,
                        object.transform.y,
                        closest_transform.x,
                        closest_transform.y
                    );
                }
            } else {
                panic!(
                    "could not find object at [{:.2}, {:.2}]",
                    object.transform.x, object.transform.y
                );
            }
        }
    }

    #[test]
    fn test_query() {
        let mut quad_tree = QuadTree::new([0.0, 0.0], 100.0, Some(100));

        let mut objects = vec![];
        let mut objects_to_search = vec![];

        let collider = Collider {
            shape: CircleCollider { radius: 10.0 }.into(),
            properties: ColliderProperties::default(),
        };
        let query_range = QuadTreeRangeCircle {
            center: Transform::default(),
            radius: 10.0,
        };

        for _ in 0..10000 {
            let mut rng = rand::thread_rng();
            let random_x = rng.gen::<f64>() * 100.0;
            let random_y = rng.gen::<f64>() * 100.0;

            let mut new_query_range = query_range.clone();
            new_query_range.center.x = random_x;
            new_query_range.center.y = random_y;

            let object = QuadTreeObject {
                object: &collider,
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
                query_range: Box::new(new_query_range),
            };

            let object_to_search = QuadTreeObject {
                object: &collider,
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
                query_range: Box::new(new_query_range),
            };

            objects.push(object);
            objects_to_search.push(object_to_search);
        }

        quad_tree.root.bulk_insert(objects);

        for _ in 0..10000 {
            let mut rng = rand::thread_rng();
            let random_x = rng.gen::<f64>() * 100.0;
            let random_y = rng.gen::<f64>() * 100.0;

            let range = QuadTreeRangeCircle {
                center: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
                radius: 0.0,
            };

            let objects = quad_tree.root.query_range(&range);

            let transforms_in_range = objects
                .iter()
                .map(|object| object.transform)
                .collect::<Vec<_>>();

            for object in objects {
                let object_transform = object.transform;
                let distance = object_transform.distance_to(&range.center);
                if distance > range.radius {
                    panic!(
                        "object at [{:.2}, {:.2}] is outside of the range",
                        object_transform.x, object_transform.y
                    );
                }
            }

            for object in objects_to_search.iter() {
                if !transforms_in_range.contains(&object.transform) {
                    let object_transform = object.transform;
                    let distance = object_transform.distance_to(&range.center);
                    if distance <= range.radius {
                        panic!(
                            "object at [{:.2}, {:.2}] is inside of the range but was not found",
                            object_transform.x, object_transform.y
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_max_depth() {
        let mut quad_tree = QuadTree::new([0.0, 0.0], 100.0, Some(100));

        let mut objects = vec![];

        let mut rng = rand::thread_rng();
        let random_x = rng.gen::<f64>() * 100.0;
        let random_y = rng.gen::<f64>() * 100.0;

        let collider = Collider {
            shape: CircleCollider { radius: 10.0 }.into(),
            properties: ColliderProperties::default(),
        };
        let query_range = QuadTreeRangeCircle {
            center: Transform {
                x: 0.0,
                y: 0.0,
                ..Transform::default()
            },
            radius: 10.0,
        };
        for _ in 0..10000 {
            let object = QuadTreeObject {
                object: &collider,
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
                query_range: Box::new(query_range),
            };

            objects.push(object);
        }

        // I believe it will crash unless we have a max depth
        quad_tree.bulk_insert(objects);
    }

    #[test]
    fn test_collisions() {
        let mut quad_tree = QuadTree::new([0.0, 0.0], 100.0, Some(100));

        let mut objects = vec![];

        let mut rng = rand::thread_rng();

        let collider = Collider {
            shape: CircleCollider { radius: 10.0 }.into(),
            properties: ColliderProperties::default(),
        };

        for _ in 0..200 {
            let random_x = rng.gen::<f64>() * 100.0;
            let random_y = rng.gen::<f64>() * 100.0;
            let new_query_range = QuadTreeRangeCircle {
                center: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
                radius: 10.0,
            };
            let object = QuadTreeObject {
                object: &collider,
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
                query_range: Box::new(new_query_range),
            };

            objects.push(object);
        }

        quad_tree.bulk_insert(objects);

        let collisions = quad_tree.find_possible_collisions();
        let num_maybe_collisions_in_quad = collisions.len() as i32;
        let mut num_collisions_in_quad = 0;
        for [object, object_in_range] in collisions {
            let distance = object
                .transform
                .squared_distance_to(&object_in_range.transform);

            // this doesn't take into account a false positive, because it eliminates them here
            // but it covers all false negatives
            if distance <= 20.0 * 20.0 {
                num_collisions_in_quad += 1;
            }
        }

        let mut num_collisions_in_brute_force = 0;
        let objects = quad_tree.root.collect_objects();

        for (i, object) in objects.clone().into_iter().enumerate() {
            for other_object in objects[i..].into_iter() {
                if object.transform == other_object.transform {
                    continue;
                }
                let distance = object
                    .transform
                    .squared_distance_to(&other_object.transform);
                if distance <= 20.0 * 20.0 {
                    num_collisions_in_brute_force += 1;
                }
            }
        }
        println!(
            "num__maybe_collisions_in_quad: {}, num_collisions_in_quad: {}, num_collisions_in_brute_force: {}",
            num_maybe_collisions_in_quad, num_collisions_in_quad, num_collisions_in_brute_force
        );
        // for now num_collisions_in_quad will have double the amount if the radii are equal.
        assert_eq!(num_collisions_in_quad, num_collisions_in_brute_force * 2);
    }
}
