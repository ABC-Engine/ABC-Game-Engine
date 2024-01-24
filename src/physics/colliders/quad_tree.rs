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

struct QuadTreeNode<T> {
    // lower left corner
    pos: [f64; 2],
    width: f64,
    child: Option<QuadTreeBranch<T>>,
    max_leaf_objects: usize,
}

impl<T> QuadTreeNode<T> {
    fn new(pos: [f64; 2], width: f64) -> Self {
        Self {
            pos,
            width,
            child: Some(QuadTreeBranch::Leaf(Leaf {
                objects: Vec::new(),
            })),
            max_leaf_objects: 1,
        }
    }

    fn insert(&mut self, object: QuadTreeObject<T>) {
        /*match (self.children_nodes.is_empty(), self.objects.is_empty()) {
            (true, true) => {
                self.objects.push(Leaf::new(object));
                return;
            }
            (true, false) => {
                // this is currently a leaf node, we need to split the node
                let mut objects = self.objects.objects.drain(..).collect::<Vec<_>>();
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
        }*/
        match self.child {
            Some(QuadTreeBranch::Node(ref mut nodes)) => {
                let quadrant = find_quadrant(self.pos, self.width, &object);

                let child = &mut nodes[quadrant as usize];
                child.insert(object);
            }
            Some(QuadTreeBranch::Leaf(ref mut leaf)) => {
                if leaf.objects.len() < self.max_leaf_objects {
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
                /*println!(
                    "chose quadrant {:?} for object at [{:.2}, {:.2}] with square at range [{} - {}, {} - {}]",
                    quadrant,
                    object.transform.x,
                    object.transform.y,
                    self.pos[0],
                    self.pos[0] + self.width,
                    self.pos[1],
                    self.pos[1] + self.width
                );*/

                let child = &nodes[quadrant as usize];
                child.find_closest_object_to_point(object)
            }
            Some(QuadTreeBranch::Leaf(ref leaf)) => {
                // temporary implementation, just return the first object
                return Some(&leaf.objects[0]);
            }
            None => {
                return None;
            }
        }
    }

    fn add_children_nodes(&mut self) {
        let new_width = self.width / 2.0;

        let offsets = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];

        let mut children_nodes: [QuadTreeNode<T>; 4] = unsafe {
            // this is safe because we are initializing all the elements, and is faster becase we don't need to write an arbitrary value
            let mut nodes: [std::mem::MaybeUninit<QuadTreeNode<T>>; 4] = [
                std::mem::MaybeUninit::uninit(),
                std::mem::MaybeUninit::uninit(),
                std::mem::MaybeUninit::uninit(),
                std::mem::MaybeUninit::uninit(),
            ];

            for (i, offset) in offsets.into_iter().enumerate() {
                let new_pos = [
                    self.pos[0] + offset[0] * new_width,
                    self.pos[1] + offset[1] * new_width,
                ];
                nodes[i]
                    .as_mut_ptr()
                    .write(QuadTreeNode::new(new_pos, new_width));
            }
            std::mem::transmute(nodes)
        };

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
}

struct QuadTree<T> {
    root: QuadTreeNode<T>,
}

impl<T> QuadTree<T> {
    fn new(pos: [f64; 2], width: f64) -> Self {
        Self {
            root: QuadTreeNode {
                pos,
                width,
                child: None,
                max_leaf_objects: 1,
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
        let mut quad_tree = QuadTree::new([0.0, 0.0], 100.0);

        let mut objects = vec![];
        let mut objects_to_search = vec![];

        for _ in 0..100 {
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
}
