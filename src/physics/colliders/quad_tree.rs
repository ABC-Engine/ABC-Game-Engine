/// this is a work in progress, it does not yet work
use crate::physics::colliders::Collider;
use crate::Transform;

#[derive(Clone, Copy)]
struct CollisionObject {
    collider: Collider,
    transform: Transform,
}

struct QuadTreeNode {
    // upper right corner of the node 0, 0 is the lower left corner
    bounds: [f64; 2],
    // this is a vec even though at the end it will only have 1 element
    objects: Vec<CollisionObject>,
    // lower left, upper left, lower right, upper right as indexed in the enum
    children_nodes: Vec<QuadTreeNode>,
}

impl QuadTreeNode {
    fn insert(&mut self, object: CollisionObject) {
        match (self.children_nodes.is_empty(), self.objects.is_empty()) {
            (true, true) => {
                self.objects.push(object);
                return;
            }
            (true, false) => {
                // should never happen, but just in case
                // we need to split the node

                let objects = self.objects.drain(..).collect::<Vec<_>>();
                self.add_children_nodes();

                self.bulk_insert(objects);
            }
            (false, true) => {
                // find the quadrant and insert into the child node
                let quadrant = find_quadrant(self.bounds, &object);
                let child = &mut self.children_nodes[quadrant as usize];
                child.insert(object);
            }
            (false, false) => {
                // also should never happen, but just in case
                let objects = self.objects.drain(..).collect::<Vec<_>>();

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
            return Some(&self.objects[0]);
        }

        let quadrant = find_quadrant(self.bounds, object);

        let child = &self.children_nodes[quadrant as usize];

        if child.is_empty() {
            // returning None for now, TODO: figure out what to do here
            return None;
        }

        child.find_closest_object_to_point(object)
    }

    fn add_children_nodes(&mut self) {
        let center = [self.bounds[0] / 2.0, self.bounds[1] / 2.0];

        let lower_left = QuadTreeNode {
            bounds: [center[0], center[1]],
            objects: Vec::new(),
            children_nodes: Vec::new(),
        };

        let upper_left = QuadTreeNode {
            bounds: [center[0], center[1]],
            objects: Vec::new(),
            children_nodes: Vec::new(),
        };

        let lower_right = QuadTreeNode {
            bounds: [center[0], center[1]],
            objects: Vec::new(),
            children_nodes: Vec::new(),
        };

        let upper_right = QuadTreeNode {
            bounds: [center[0], center[1]],
            objects: Vec::new(),
            children_nodes: Vec::new(),
        };

        self.children_nodes.push(lower_left);
        self.children_nodes.push(upper_left);
        self.children_nodes.push(lower_right);
        self.children_nodes.push(upper_right);
    }
}

struct QuadTree {
    root: QuadTreeNode,
}

impl QuadTree {
    fn new(bounds: [f64; 2]) -> Self {
        Self {
            root: QuadTreeNode {
                bounds,
                objects: Vec::new(),
                children_nodes: Vec::new(),
            },
        }
    }

    fn insert(&mut self, object: CollisionObject) {
        self.root.insert(object);
    }

    fn bulk_insert(&mut self, objects: Vec<CollisionObject>) {
        self.root.bulk_insert(objects);
    }

    fn find_closest_object_to_point(&self, object: &CollisionObject) -> Option<&CollisionObject> {
        self.root.find_closest_object_to_point(object)
    }
}

enum Quadrant {
    LowerLeft,
    UpperLeft,
    LowerRight,
    UpperRight,
}

fn find_quadrant(bounds: [f64; 2], object: &CollisionObject) -> Quadrant {
    let center = [bounds[0] / 2.0, bounds[1] / 2.0];
    let object_center = object.transform;

    let left = object_center.x < center[0];
    let top = object_center.y < center[1];

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
    use super::*;
    use crate::physics::colliders::{CircleCollider, ColliderProperties};

    #[test]
    fn test_insert() {
        let mut quad_tree = QuadTree::new([100.0, 100.0]);

        let mut objects = vec![];

        for _ in 0..10000 {
            let random_x = rand::random::<f64>() * 100.0;
            let random_y = rand::random::<f64>() * 100.0;

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

        quad_tree.root.bulk_insert(objects);

        let mut total_distance = 0.0;
        let mut total_nones = 0;
        // test the find closest object to point
        for _ in 0..100000 {
            let random_x = rand::random::<f64>() * 100.0;
            let random_y = rand::random::<f64>() * 100.0;

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

            let closest_object = quad_tree.find_closest_object_to_point(&object);

            // none should be none in the final result this is just a preliminary test
            if closest_object.is_some() {
                let distance = (closest_object.unwrap().transform.x - object.transform.x).abs()
                    + (closest_object.unwrap().transform.y - object.transform.y).abs();
                total_distance += distance;
            } else {
                total_nones += 1;
            }
        }

        // if this is a really big number, then we are doing something wrong
        println!("Average distance: {}", total_distance / 100000.0);
        println!(
            "None percentage: {}%",
            (total_nones as f64 / 100000.0) * 100.0
        );
    }
}
