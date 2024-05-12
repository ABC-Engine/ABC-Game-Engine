use rapier2d::prelude::*;
use ABC_ECS::EntitiesAndComponents;
use ABC_ECS::Entity;
use ABC_ECS::System;

use crate::Transform;

pub struct RapierPhysicsSystem {
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    physics_hooks: (),
    event_handler: (),
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
}

impl RapierPhysicsSystem {
    pub fn new() -> RapierPhysicsSystem {
        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.81];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();

        // this was broken in the example code, I think this is the correct way to do it
        let broad_phase = BroadPhaseMultiSap::new();

        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();

        RapierPhysicsSystem {
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            physics_hooks,
            event_handler,
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
        }
    }

    fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &self.event_handler,
        );
    }
}

impl System for RapierPhysicsSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        get_all_rigid_bodies_and_colliders(
            entities_and_components,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            Transform::default(),
        );

        self.step();

        // if anything is changed between the physics system and the set_all_rigid_bodies_and_colliders call, it will break it don't do that

        set_all_rigid_bodies_and_colliders(
            entities_and_components,
            &self.rigid_body_set,
            &self.collider_set,
            Transform::default(),
        );
    }
}

// just so that the user can't accidentally mess with up the internals of the physics system
struct RigidBodyHandle(rapier2d::prelude::RigidBodyHandle);
struct ColliderHandle(rapier2d::prelude::ColliderHandle);

/// This function gets all rigid bodies and colliders in the world and inserts them into the given sets
fn get_all_rigid_bodies_and_colliders(
    world: &mut EntitiesAndComponents,
    out_rigid_body_set: &mut RigidBodySet,
    out_collider_set: &mut ColliderSet,
    transform_offset: Transform,
) {
    let mut rigidbody_entities = world
        .get_entities_with_component::<RigidBody>()
        .into_iter()
        .copied()
        .collect::<Vec<Entity>>();

    let collider_entities = world
        .get_entities_with_component::<Collider>()
        .into_iter()
        .copied()
        .collect::<Vec<Entity>>();

    rigidbody_entities.extend(collider_entities);
    rigidbody_entities.sort();
    rigidbody_entities.dedup();

    for rigidbody_entity in rigidbody_entities {
        let (rigidbody, transform, rigidbody_handle) =
            world.try_get_components::<(RigidBody, Transform, RigidBodyHandle)>(rigidbody_entity);

        match (rigidbody, transform, rigidbody_handle) {
            (Some(ecs_rigidbody), Some(transform), Some(rigidbody_handle)) => {
                // the entity has a handle, which means it is already in the set, so we just need to update the rigidbody
                // get the rigidbody from the handle
                let rigidbody = out_rigid_body_set
                    .get_mut(rigidbody_handle.0)
                    .expect("failed to get rigidbody from handle found in entity");

                // update the rigidbody transform
                rigidbody.set_position(
                    abc_transform_to_rapier_transform(&*transform + &transform_offset),
                    false,
                );

                rigidbody.copy_from(&ecs_rigidbody.clone());
            }
            (Some(ecs_rigidbody), Some(transform), None) => {
                // if the rigidbody doesn't have a handle, insert it into the set, and add a handle to the entity
                let mut rigidbody = ecs_rigidbody.clone();
                rigidbody.set_position(
                    abc_transform_to_rapier_transform(&*transform + &transform_offset),
                    false,
                );
                let handle = out_rigid_body_set.insert(rigidbody);
                // add a handle to the rigidbody to the entity
                world.add_component_to(rigidbody_entity, RigidBodyHandle(handle));
                println!("rigidbody inserted");
            }
            _ => {
                // log warning that rigidbody is missing transform
            }
        }
    }

    let collider_entities = world
        .get_entities_with_component::<Collider>()
        .into_iter()
        .copied()
        .collect::<Vec<Entity>>();
    for collider_entity in collider_entities {
        let (collider, transform, collider_handle, rb_handle) =
            world.try_get_components::<(Collider, Transform, ColliderHandle, RigidBodyHandle)>(
                collider_entity,
            );

        match (collider, transform, collider_handle) {
            (Some(ecs_collider), Some(transform), Some(collider_handle)) => {
                // the entity has a handle, which means it is already in the set, so we just need to update the collider
                // get the collider from the handle
                let collider = out_collider_set
                    .get_mut(collider_handle.0)
                    .expect("failed to get collider from handle found in entity");

                collider.copy_from(&ecs_collider.clone());
            }
            (Some(collider), Some(transform), None) => {
                // if the collider doesn't have a handle, insert it into the set, and add a handle to the entity
                let mut collider = collider.clone();

                let new_collider_handle = if let Some(rigidbody_handle) = rb_handle {
                    out_collider_set.insert_with_parent(
                        collider,
                        rigidbody_handle.0,
                        out_rigid_body_set,
                    )
                } else {
                    out_collider_set.insert(collider)
                };
                // add a handle to the collider to the entity
                world.add_component_to(collider_entity, ColliderHandle(new_collider_handle));
            }
            _ => {
                // log warning that collider is missing transform
            }
        }
    }

    // recursively get all children
    let entities_with_children = world
        .get_entities_with_component::<EntitiesAndComponents>()
        .into_iter()
        .copied()
        .collect::<Vec<Entity>>();

    for entity in entities_with_children {
        let (children, transform) =
            world.try_get_components_mut::<(EntitiesAndComponents, Transform)>(entity);

        let mut transform_offset = transform_offset;
        if let Some(transform) = transform {
            transform_offset = &*transform + &transform_offset;
        }

        get_all_rigid_bodies_and_colliders(
            children.expect("failed to get children, this is a bug"),
            out_rigid_body_set,
            out_collider_set,
            transform_offset,
        );
    }
}

/// This function updates the transforms of all rigid bodies and colliders in the world
/// it uses the same logic as get_all_rigid_bodies_and_colliders, but instead of inserting into the sets, it updates the transforms
fn set_all_rigid_bodies_and_colliders(
    world: &mut EntitiesAndComponents,
    rigid_body_set: &RigidBodySet,
    collider_set: &ColliderSet,
    transform_offset: Transform,
) {
    {
        // we have to do this dance to avoid borrowing issues...
        let rigidbody_entities = world
            .get_entities_with_component::<RigidBodyHandle>()
            .into_iter()
            .copied()
            .collect::<Vec<Entity>>();
        for rigidbody_entity in rigidbody_entities {
            let (rigidbody, transform, rigidbody_handle) =
                world.try_get_components_mut::<(RigidBody, Transform, RigidBodyHandle)>(
                    rigidbody_entity,
                );

            match (rigidbody, transform, rigidbody_handle) {
                (Some(ecs_rigidbody), Some(transform), Some(rigidbody_handle)) => {
                    let rigidbody = rigid_body_set.get(
                        rigidbody_handle
                            .0,
                    ).expect("failed to get rigidbody from handle found in entity, please report this as a bug");

                    update_abc_transform_from_rapier_transform(
                        transform,
                        transform_offset,
                        *rigidbody.position(),
                    );

                    *ecs_rigidbody = rigidbody.clone();
                }
                _ => {
                    // log warning that rigidbody is missing transform
                    //println!("components that you have transform: {:?}, rigidbody: {:?}, rigidbody_handle: {:?}", transform.is_some(), rigidbody.is_some(), rigidbody_handle.is_some());
                }
            }
        }
    }

    {
        // we have to do this dance to avoid borrowing issues...
        let collider_entities = world
            .get_entities_with_component::<Collider>()
            .into_iter()
            .copied()
            .collect::<Vec<Entity>>();

        for collider_entity in collider_entities {
            let (collider, transform, collider_handle) = world
                .try_get_components_mut::<(Collider, Transform, ColliderHandle)>(collider_entity);
            if let (Some(ecs_collider), Some(transform), Some(collider_handle)) =
                (collider, transform, collider_handle)
            {
                let collider = collider_set
                    .get(
                        collider_handle
                            .0,
                    ).expect("failed to get collider from handle found in entity, please report this as a bug");

                *ecs_collider = collider.clone();
            } else {
                // log warning that collider is missing transform
            }
        }
    }

    // recursively get all children
    // we have to do this dance to avoid borrowing issues...
    let entities_with_children = world
        .get_entities_with_component::<EntitiesAndComponents>()
        .into_iter()
        .copied()
        .collect::<Vec<Entity>>();

    for entity in entities_with_children {
        let (children, transform) =
            world.try_get_components_mut::<(EntitiesAndComponents, Transform)>(entity);

        let mut transform_offset = transform_offset;
        if let Some(transform) = transform {
            transform_offset = &*transform + &transform_offset;
        }

        set_all_rigid_bodies_and_colliders(
            children.expect("failed to get children, this is a bug"),
            rigid_body_set,
            collider_set,
            transform_offset,
        );
    }
}

fn abc_transform_to_rapier_transform(transform: Transform) -> Isometry<Real> {
    Isometry::new(
        vector![transform.x as f32, transform.y as f32],
        transform.rotation as f32,
    )
}

fn update_abc_transform_from_rapier_transform(
    transform: &mut Transform,
    offset: Transform, // offset is the parent transform
    rapier_transform: Isometry<Real>,
) {
    // we subtract the offset to get the local transform
    transform.x = rapier_transform.translation.x as f64 - offset.x;
    transform.y = rapier_transform.translation.y as f64 - offset.y;
    transform.rotation = rapier_transform.rotation.angle() as f64 - offset.rotation;
}

/*
let mut rigid_body_set = RigidBodySet::new();
let mut collider_set = ColliderSet::new();

/* Create the ground. */
let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
collider_set.insert(collider);

/* Create the bouncing ball. */
let rigid_body = RigidBodyBuilder::dynamic()
    .translation(vector![0.0, 10.0])
    .build();
let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
let ball_body_handle = rigid_body_set.insert(rigid_body);
collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);
*/
