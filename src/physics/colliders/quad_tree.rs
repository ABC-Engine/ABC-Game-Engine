use std::panic;

/// this is a work in progress, it does not yet work
use crate::physics::colliders::Collider;
use crate::Transform;

#[derive(Clone, Copy)]
struct CollisionObject {
    collider: Collider,
    transform: Transform,
}

struct QuadTreeNode {
    // lower left corner
    pos: [f64; 2],
    width: f64,
    // this is a vec even though at the end it will only have 1 element
    objects: Vec<CollisionObject>,
    // lower left, upper left, lower right, upper right as indexed in the enum
    children_nodes: Vec<QuadTreeNode>,
}

impl QuadTreeNode {
    fn new(pos: [f64; 2], width: f64) -> Self {
        Self {
            pos,
            width,
            objects: vec![],
            children_nodes: vec![],
        }
    }

    fn insert(&mut self, object: CollisionObject) {
        match (self.children_nodes.is_empty(), self.objects.is_empty()) {
            (true, true) => {
                self.objects.push(object);
                return;
            }
            (true, false) => {
                // this is currently a leaf node, we need to split the node
                let mut objects = self.objects.drain(..).collect::<Vec<_>>();
                objects.push(object);

                self.add_children_nodes();
                self.bulk_insert(objects);
            }
            (false, true) => {
                // find the quadrant and insert into the child node
                let quadrant = find_quadrant(self.pos, self.width, &object);

                let child = &mut self.children_nodes[quadrant as usize];
                child.insert(object);
            }
            (false, false) => {
                // should never happen, but just in case
                let mut objects = self.objects.drain(..).collect::<Vec<_>>();
                objects.push(object);

                self.add_children_nodes();
                self.bulk_insert(objects);
            }
        }
    }

    fn bulk_insert(&mut self, objects: Vec<CollisionObject>) {
        // TODO: this is a naive implementation, we should be able to do this in parallel
        // but.. I'm not sure how to do that with the current structure
        for object in objects {
            self.insert(object);
        }
    }

    fn is_empty(&self) -> bool {
        self.objects.is_empty() && self.children_nodes.is_empty()
    }

    fn find_closest_object_to_point(&self, object: &CollisionObject) -> Option<&CollisionObject> {
        // the only case it should ever return None
        if self.is_empty() {
            return None;
        }

        // should mean that we are at the leaf node
        if self.objects.len() == 1 {
            assert!(self.children_nodes.is_empty());
            return Some(&self.objects[0]);
        }

        let quadrant = find_quadrant(self.pos, self.width, object);
        println!(
            "chose quadrant {:?} for object at [{:.2}, {:.2}] with square at range [{} - {}, {} - {}]",
            quadrant,
            object.transform.x,
            object.transform.y,
            self.pos[0],
            self.pos[0] + self.width,
            self.pos[1],
            self.pos[1] + self.width
        );

        let child = &self.children_nodes[quadrant as usize];

        if child.is_empty() {
            // returning None for now, TODO: figure out what to do here
            return None;
        }

        child.find_closest_object_to_point(object)
    }

    fn add_children_nodes(&mut self) {
        let new_width = self.width / 2.0;

        let offsets = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];

        for offset in offsets {
            let new_pos = [
                self.pos[0] + offset[0] * new_width,
                self.pos[1] + offset[1] * new_width,
            ];
            self.children_nodes
                .push(QuadTreeNode::new(new_pos, new_width));
        }
    }

    fn could_fit_point(&self, object: &CollisionObject) -> bool {
        let object_center = object.transform;

        let right = object_center.x >= self.pos[0] + self.width;
        let left = object_center.x <= self.pos[0];
        let top = object_center.y >= self.pos[1] + self.width;
        let bottom = object_center.y <= self.pos[1];

        !(left || right || top || bottom)
    }
}

struct QuadTree {
    root: QuadTreeNode,
}

impl QuadTree {
    fn new(pos: [f64; 2], width: f64) -> Self {
        Self {
            root: QuadTreeNode {
                pos,
                width,
                objects: Vec::new(),
                children_nodes: Vec::new(),
            },
        }
    }

    fn insert(&mut self, object: CollisionObject) {
        if !self.root.could_fit_point(&object) {
            panic!("object does not fit in the quad tree: user error");
        }

        self.root.insert(object);
    }

    fn bulk_insert(&mut self, objects: Vec<CollisionObject>) {
        for object in &objects {
            if !self.root.could_fit_point(object) {
                panic!("object does not fit in the quad tree: user error");
            }
        }
        self.root.bulk_insert(objects);
    }

    fn find_closest_object_to_point(&self, object: &CollisionObject) -> Option<&CollisionObject> {
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

fn find_quadrant(pos: [f64; 2], width: f64, object: &CollisionObject) -> Quadrant {
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
        let mut quad_tree = QuadTree::new([0.0, 0.0], 100.0);

        let mut objects = vec![];

        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let random_x = rng.gen::<f64>() * 100.0;
            let random_y = rng.gen::<f64>() * 100.0;

            let object = CollisionObject {
                collider: Collider {
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

        quad_tree.root.bulk_insert(objects.clone());

        // test the find closest object to point
        for object in objects {
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
}
