use std::panic;

/// this is a work in progress, it does not yet work
use crate::physics::colliders::Collider;
use crate::Transform;

#[derive(Clone, Copy)]
struct CollisionObject {
    collider: Collider,
    transform: Transform,
}

struct QuadTreeObject<T> {
    object: T,
    transform: Transform,
}

struct Leaf<T> {
    objects: Vec<QuadTreeObject<T>>,
}

impl<T> Leaf<T> {
    fn new(object: QuadTreeObject<T>) -> Self {
        Self {
            objects: vec![object],
        }
    }
}

/// QuadTreeBranch is a branch of the quad tree, it can either be 4 children nodes or a leaf node
enum QuadTreeBranch<T> {
    // lower left, upper left, lower right, upper right as indexed in the enum
    Node(Box<[QuadTreeNode<T>; 4]>),
    Leaf(Leaf<T>),
}

trait QuadTreeRange {
    fn contains_point(&self, point: &Transform) -> bool;
}

struct QuadTreeRangeCircle {
    center: Transform,
    radius: f64,
}

impl QuadTreeRange for QuadTreeRangeCircle {
    fn contains_point(&self, point: &Transform) -> bool {
        let distance = self.center.squared_distance_to(point);
        distance <= self.radius.powi(2)
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
}

struct QuadTreeNode<T> {
    // lower left corner
    pos: [f64; 2],
    width: f64,
    child: Option<QuadTreeBranch<T>>,
    max_leaf_objects: usize,
    // this being None means that there is no max depth, not that the depth is 0
    depth_left: Option<u32>,
}

impl<T> QuadTreeNode<T> {
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

    fn insert(&mut self, object: QuadTreeObject<T>) {
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

    fn bulk_insert(&mut self, objects: Vec<QuadTreeObject<T>>) {
        // TODO: this is a naive implementation, we should be able to do this in parallel
        // but.. I'm not sure how to do that with the current structure
        for object in objects {
            self.insert(object);
        }
    }

    fn is_empty(&self) -> bool {
        self.child.is_none()
    }

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
        let mut objects = vec![];

        if let Some(ref child) = self.child {
            match child {
                QuadTreeBranch::Node(nodes) => {
                    for node in &**nodes {
                        objects.extend(node.query_range(range));
                    }
                }
                QuadTreeBranch::Leaf(leaf) => {
                    for object in &leaf.objects {
                        if range.contains_point(&object.transform) {
                            objects.push(object);
                        }
                    }
                }
            }
        }

        objects
    }
}

struct QuadTree<T> {
    root: QuadTreeNode<T>,
}

impl<T> QuadTree<T> {
    fn new(pos: [f64; 2], width: f64, max_depth: Option<u32>) -> Self {
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

    fn insert(&mut self, object: QuadTreeObject<T>) {
        if !self.root.could_fit_point(&object) {
            panic!("object does not fit in the quad tree: user error");
        }

        self.root.insert(object);
    }

    fn bulk_insert(&mut self, objects: Vec<QuadTreeObject<T>>) {
        for object in &objects {
            if !self.root.could_fit_point(object) {
                panic!("object does not fit in the quad tree: user error");
            }
        }
        self.root.bulk_insert(objects);
    }

    fn find_closest_object_to_point(
        &self,
        object: &QuadTreeObject<T>,
    ) -> Option<&QuadTreeObject<T>> {
        self.root.find_closest_object_to_point(object)
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

        for _ in 0..10000 {
            let mut rng = rand::thread_rng();
            let random_x = rng.gen::<f64>() * 100.0;
            let random_y = rng.gen::<f64>() * 100.0;

            let object = QuadTreeObject {
                object: Collider {
                    shape: CircleCollider { radius: 10.0 }.into(),
                    properties: ColliderProperties::default(),
                },
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
            };

            let object_to_search = QuadTreeObject {
                object: Collider {
                    shape: CircleCollider { radius: 10.0 }.into(),
                    properties: ColliderProperties::default(),
                },
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
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

        for _ in 0..10000 {
            let mut rng = rand::thread_rng();
            let random_x = rng.gen::<f64>() * 100.0;
            let random_y = rng.gen::<f64>() * 100.0;

            let object = QuadTreeObject {
                object: Collider {
                    shape: CircleCollider { radius: 10.0 }.into(),
                    properties: ColliderProperties::default(),
                },
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
            };

            let object_to_search = QuadTreeObject {
                object: Collider {
                    shape: CircleCollider { radius: 10.0 }.into(),
                    properties: ColliderProperties::default(),
                },
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
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
                radius: 10.0,
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
        for _ in 0..10000 {
            let object = QuadTreeObject {
                object: Collider {
                    shape: CircleCollider { radius: 10.0 }.into(),
                    properties: ColliderProperties::default(),
                },
                transform: Transform {
                    x: random_x,
                    y: random_y,
                    ..Transform::default()
                },
            };

            objects.push(object);
        }

        // I believe it will crash unless we have a max depth
        quad_tree.bulk_insert(objects);
    }
}
