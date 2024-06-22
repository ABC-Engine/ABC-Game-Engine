use std::ops::Deref;
use std::ops::DerefMut;

use rapier2d::parry::query::NonlinearRigidMotion;
pub use rapier2d::prelude::ColliderHandle as RapierColliderHandle;
pub use rapier2d::prelude::RigidBodyHandle as RapierRigidBodyHandle;
use rapier2d::prelude::*;
use tracing::Level;
use ABC_ECS::EntitiesAndComponents;
use ABC_ECS::Entity;
use ABC_ECS::Resource;
use ABC_ECS::System;

use crate::delta_time;
use crate::Transform;
use tracing::event;

/// created with the rapier physics system do not create this manually
/// this is a wrapper around the rapier physics pipeline that allows for easy access to the physics engine
/// all of the docs are 95% copy pasted from the rapier docs
pub struct RapierPhysicsInfo {
    query_pipeline: QueryPipeline,
    rigid_body_handle_map: std::collections::HashMap<RigidBodyHandle, Entity>,
    collider_handle_map: std::collections::HashMap<ColliderHandle, Entity>,
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    time_started: std::time::Instant,
    time_elapsed_before_this_frame: std::time::Duration,
}

impl RapierPhysicsInfo {
    pub fn set_gravity(&mut self, gravity: Vector<Real>) {
        self.gravity = gravity;
    }

    /// Find the associated entity with a rigid body handle.
    pub fn get_associated_entity_with_rigid_body_handle(
        &self,
        handle: RigidBodyHandle,
    ) -> Option<Entity> {
        self.rigid_body_handle_map.get(&handle).cloned()
    }

    /// Find the associated entity with a collider handle.
    pub fn get_associated_entity_with_collider_handle(
        &self,
        handle: ColliderHandle,
    ) -> Option<Entity> {
        self.collider_handle_map.get(&handle).cloned()
    }

    /// Find the closest intersection between a ray and a set of collider.
    ///
    /// # Parameters
    /// * `ray`: the ray to cast.
    /// * `max_toi`: the maximum time-of-impact that can be reported by this cast. This effectively
    ///   limits the length of the ray to `ray.dir.norm() * max_toi`. Use `Real::MAX` for an unbounded ray.
    /// * `solid`: if this is `true` an impact at time 0.0 (i.e. at the ray origin) is returned if
    ///            it starts inside of a shape. If this `false` then the ray will hit the shape's boundary
    ///            even if its starts inside of it.
    /// * `filter`: set of rules used to determine which collider is taken into account by this scene query.
    pub fn cast_ray(
        &self,
        ray: &Ray,
        max_toi: Real,
        solid: bool,
        filter: QueryFilter,
    ) -> Option<(Entity, Real)> {
        let intersection = self.query_pipeline.cast_ray(
            &self.rigid_body_set,
            &self.collider_set,
            &ray,
            max_toi,
            solid,
            filter,
        );

        match intersection {
            Some((handle, toi)) => {
                let entity = self
                    .get_associated_entity_with_collider_handle(ColliderHandle(handle))
                    .expect("failed to get entity associated with collider handle, this is a bug");
                Some((entity, toi))
            }
            None => None,
        }
    }

    /// Find the closest intersection between a ray and a set of collider.
    ///
    /// # Parameters
    /// * `ray`: the ray to cast.
    /// * `max_toi`: the maximum time-of-impact that can be reported by this cast. This effectively
    ///   limits the length of the ray to `ray.dir.norm() * max_toi`. Use `Real::MAX` for an unbounded ray.
    /// * `solid`: if this is `true` an impact at time 0.0 (i.e. at the ray origin) is returned if
    ///            it starts inside of a shape. If this `false` then the ray will hit the shape's boundary
    ///            even if its starts inside of it.
    /// * `filter`: set of rules used to determine which collider is taken into account by this scene query.
    pub fn cast_ray_and_get_normal(
        &self,
        ray: &Ray,
        max_toi: Real,
        solid: bool,
        filter: QueryFilter,
    ) -> Option<(Entity, RayIntersection)> {
        let intersection = self.query_pipeline.cast_ray_and_get_normal(
            &self.rigid_body_set,
            &self.collider_set,
            ray,
            max_toi,
            solid,
            filter,
        );

        match intersection {
            Some((handle, intersection)) => {
                let entity = self
                    .get_associated_entity_with_collider_handle(ColliderHandle(handle))
                    .expect("failed to get entity associated with collider handle, this is a bug");
                Some((entity, intersection))
            }
            None => None,
        }
    }

    /// Find the all intersections between a ray and a set of collider and passes them to a callback.
    ///
    /// # Parameters
    /// * `ray`: the ray to cast.
    /// * `max_toi`: the maximum time-of-impact that can be reported by this cast. This effectively
    ///   limits the length of the ray to `ray.dir.norm() * max_toi`. Use `Real::MAX` for an unbounded ray.
    /// * `solid`: if this is `true` an impact at time 0.0 (i.e. at the ray origin) is returned if
    ///            it starts inside of a shape. If this `false` then the ray will hit the shape's boundary
    ///            even if its starts inside of it.
    /// * `filter`: set of rules used to determine which collider is taken into account by this scene query.
    /// * `callback`: function executed on each collider for which a ray intersection has been found.
    ///               There is no guarantees on the order the results will be yielded. If this callback returns `false`,
    ///               this method will exit early, ignore any further raycast.
    pub fn intersections_with_ray<'a>(
        &self,
        ray: &Ray,
        max_toi: Real,
        solid: bool,
        filter: QueryFilter,
        mut callback: impl FnMut(Entity, RayIntersection) -> bool,
    ) {
        self.query_pipeline.intersections_with_ray(
            &self.rigid_body_set,
            &self.collider_set,
            ray,
            max_toi,
            solid,
            filter,
            |handle, intersection| {
                let entity = self
                    .get_associated_entity_with_collider_handle(ColliderHandle(handle))
                    .expect("failed to get entity associated with collider handle, this is a bug");
                callback(entity, intersection)
            },
        );
    }

    /// Gets the handle of up to one collider intersecting the given shape.
    ///
    /// # Parameters
    /// * `shape_pos` - The position of the shape used for the intersection test.
    /// * `shape` - The shape used for the intersection test.
    /// * `filter`: set of rules used to determine which collider is taken into account by this scene query.
    pub fn intersection_with_shape(
        &self,
        shape_pos: &Isometry<Real>,
        shape: &dyn Shape,
        filter: QueryFilter,
    ) -> Option<Entity> {
        let intersection = self.query_pipeline.intersection_with_shape(
            &self.rigid_body_set,
            &self.collider_set,
            shape_pos,
            shape,
            filter,
        );

        match intersection {
            Some(handle) => {
                let entity =
                    self.get_associated_entity_with_collider_handle(ColliderHandle(handle));
                entity
            }
            None => None,
        }
    }

    /// Find the projection of a point on the closest collider.
    ///
    /// # Parameters
    /// * `colliders` - The set of colliders taking part in this pipeline.
    /// * `point` - The point to project.
    /// * `solid` - If this is set to `true` then the collider shapes are considered to
    ///   be plain (if the point is located inside of a plain shape, its projection is the point
    ///   itself). If it is set to `false` the collider shapes are considered to be hollow
    ///   (if the point is located inside of an hollow shape, it is projected on the shape's
    ///   boundary).
    /// * `filter`: set of rules used to determine which collider is taken into account by this scene query.
    pub fn project_point(
        &self,
        point: &Point<Real>,
        solid: bool,
        filter: QueryFilter,
    ) -> Option<(Entity, PointProjection)> {
        let projection = self.query_pipeline.project_point(
            &self.rigid_body_set,
            &self.collider_set,
            point,
            solid,
            filter,
        );

        match projection {
            Some((handle, projection)) => {
                let entity = self
                    .get_associated_entity_with_collider_handle(ColliderHandle(handle))
                    .expect("failed to get entity associated with collider handle, this is a bug");
                Some((entity, projection))
            }
            None => None,
        }
    }

    /// Find all the colliders containing the given point.
    ///
    /// # Parameters
    /// * `point` - The point used for the containment test.
    /// * `filter`: set of rules used to determine which collider is taken into account by this scene query.
    /// * `callback` - A function called with each collider with a shape
    ///                containing the `point`.
    pub fn intersections_with_point(
        &self,
        point: &Point<Real>,
        filter: QueryFilter,
        mut callback: impl FnMut(Entity) -> bool,
    ) {
        self.query_pipeline.intersections_with_point(
            &self.rigid_body_set,
            &self.collider_set,
            point,
            filter,
            |handle| {
                let entity = self
                    .get_associated_entity_with_collider_handle(ColliderHandle(handle))
                    .expect("failed to get entity associated with collider handle, this is a bug");
                callback(entity)
            },
        );
    }

    /// Find the projection of a point on the closest collider.
    ///
    /// The results include the ID of the feature hit by the point.
    ///
    /// # Parameters
    /// * `point` - The point to project.
    /// * `solid` - If this is set to `true` then the collider shapes are considered to
    ///   be plain (if the point is located inside of a plain shape, its projection is the point
    ///   itself). If it is set to `false` the collider shapes are considered to be hollow
    ///   (if the point is located inside of an hollow shape, it is projected on the shape's
    ///   boundary).
    /// * `filter`: set of rules used to determine which collider is taken into account by this scene query.
    pub fn project_point_and_get_feature(
        &self,
        point: &Point<Real>,
        filter: QueryFilter,
    ) -> Option<(Entity, PointProjection, FeatureId)> {
        let projection = self.query_pipeline.project_point_and_get_feature(
            &self.rigid_body_set,
            &self.collider_set,
            point,
            filter,
        );

        match projection {
            Some((handle, projection, feature)) => {
                let entity = self
                    .get_associated_entity_with_collider_handle(ColliderHandle(handle))
                    .expect("failed to get entity associated with collider handle, this is a bug");
                Some((entity, projection, feature))
            }
            None => None,
        }
    }

    /// Finds all handles of all the colliders with an Aabb intersecting the given Aabb.
    pub fn colliders_with_aabb_intersecting_aabb(
        &self,
        aabb: &Aabb,
        mut callback: impl FnMut(&Entity) -> bool,
    ) {
        self.query_pipeline
            .colliders_with_aabb_intersecting_aabb(aabb, |handle| {
                let entity = self
                    .get_associated_entity_with_collider_handle(ColliderHandle(*handle))
                    .expect("failed to get entity associated with collider handle, this is a bug");
                callback(&entity)
            });
    }

    /// Casts a shape at a constant linear velocity and retrieve the first collider it hits.
    ///
    /// This is similar to ray-casting except that we are casting a whole shape instead of just a
    /// point (the ray origin). In the resulting `TOI`, witness and normal 1 refer to the world
    /// collider, and are in world space.
    ///
    /// # Parameters
    /// * `shape_pos` - The initial position of the shape to cast.
    /// * `shape_vel` - The constant velocity of the shape to cast (i.e. the cast direction).
    /// * `shape` - The shape to cast.
    /// * `max_toi` - The maximum time-of-impact that can be reported by this cast. This effectively
    ///   limits the distance traveled by the shape to `shapeVel.norm() * maxToi`.
    /// * `stop_at_penetration` - If set to `false`, the linear shape-cast won’t immediately stop if
    ///   the shape is penetrating another shape at its starting point **and** its trajectory is such
    ///   that it’s on a path to exist that penetration state.
    /// * `filter`: set of rules used to determine which collider is taken into account by this scene query.
    pub fn cast_shape(
        &self,
        shape_pos: &Isometry<Real>,
        shape_vel: &Vector<Real>,
        shape: &dyn Shape,
        options: rapier2d::parry::query::ShapeCastOptions,
        filter: QueryFilter,
    ) -> Option<(Entity, ShapeCastHit)> {
        let intersection = self.query_pipeline.cast_shape(
            &self.rigid_body_set,
            &self.collider_set,
            shape_pos,
            shape_vel,
            shape,
            options,
            filter,
        );

        match intersection {
            Some((handle, intersection)) => {
                let entity = self
                    .get_associated_entity_with_collider_handle(ColliderHandle(handle))
                    .expect("failed to get entity associated with collider handle, this is a bug");
                Some((entity, intersection))
            }
            None => None,
        }
    }

    /// Casts a shape with an arbitrary continuous motion and retrieve the first collider it hits.
    ///
    /// In the resulting `TOI`, witness and normal 1 refer to the world collider, and are in world
    /// space.
    ///
    /// # Parameters
    /// * `shape_motion` - The motion of the shape.
    /// * `shape` - The shape to cast.
    /// * `start_time` - The starting time of the interval where the motion takes place.
    /// * `end_time` - The end time of the interval where the motion takes place.
    /// * `stop_at_penetration` - If the casted shape starts in a penetration state with any
    ///    collider, two results are possible. If `stop_at_penetration` is `true` then, the
    ///    result will have a `toi` equal to `start_time`. If `stop_at_penetration` is `false`
    ///    then the nonlinear shape-casting will see if further motion with respect to the penetration normal
    ///    would result in tunnelling. If it does not (i.e. we have a separating velocity along
    ///    that normal) then the nonlinear shape-casting will attempt to find another impact,
    ///    at a time `> start_time` that could result in tunnelling.
    /// * `filter`: set of rules used to determine which collider is taken into account by this scene query.
    pub fn nonlinear_cast_shape(
        &self,
        shape_motion: &NonlinearRigidMotion,
        shape: &dyn Shape,
        start_time: Real,
        end_time: Real,
        stop_at_penetration: bool,
        filter: QueryFilter,
    ) -> Option<(Entity, ShapeCastHit)> {
        let intersection = self.query_pipeline.nonlinear_cast_shape(
            &self.rigid_body_set,
            &self.collider_set,
            shape_motion,
            shape,
            start_time,
            end_time,
            stop_at_penetration,
            filter,
        );

        match intersection {
            Some((handle, intersection)) => {
                let entity = self
                    .get_associated_entity_with_collider_handle(ColliderHandle(handle))
                    .expect("failed to get entity associated with collider handle, this is a bug");
                Some((entity, intersection))
            }
            None => None,
        }
    }

    /// Retrieve all the colliders intersecting the given shape.
    ///
    /// # Parameters
    /// * `shapePos` - The position of the shape to test.
    /// * `shapeRot` - The orientation of the shape to test.
    /// * `shape` - The shape to test.
    /// * `filter`: set of rules used to determine which collider is taken into account by this scene query.
    /// * `callback` - A function called with the handles of each collider intersecting the `shape`.
    pub fn intersections_with_shape(
        &self,
        shape_pos: &Isometry<Real>,
        shape: &dyn Shape,
        filter: QueryFilter,
        mut callback: impl FnMut(Entity) -> bool,
    ) {
        self.query_pipeline.intersections_with_shape(
            &self.rigid_body_set,
            &self.collider_set,
            shape_pos,
            shape,
            filter,
            |handle| {
                let entity = self
                    .get_associated_entity_with_collider_handle(ColliderHandle(handle))
                    .expect("failed to get entity associated with collider handle, this is a bug");
                callback(entity)
            },
        );
    }
}

impl Deref for RapierPhysicsInfo {
    type Target = QueryPipeline;

    fn deref(&self) -> &Self::Target {
        &self.query_pipeline
    }
}

impl DerefMut for RapierPhysicsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.query_pipeline
    }
}

impl Resource for RapierPhysicsInfo {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct RapierPhysicsSystem {}

impl RapierPhysicsSystem {
    pub fn new(world: &mut EntitiesAndComponents) -> RapierPhysicsSystem {
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
        let physics_hooks = ();
        let event_handler = ();
        let query_pipeline = QueryPipeline::new();

        let rapier_physics_info = RapierPhysicsInfo {
            query_pipeline,
            rigid_body_handle_map: std::collections::HashMap::new(),
            collider_handle_map: std::collections::HashMap::new(),
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            physics_hooks,
            event_handler,
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            time_started: std::time::Instant::now(),
            time_elapsed_before_this_frame: std::time::Duration::new(0, 0),
        };
        // add the physics info to the world
        world.add_resource(rapier_physics_info);

        RapierPhysicsSystem {}
    }

    fn step(&mut self, world: &mut EntitiesAndComponents) {
        let physics_info = &mut world
            .get_resource_mut::<RapierPhysicsInfo>()
            .expect("failed to get rapier physics info, report this as a bug");
        let query_pipeline = &mut physics_info.query_pipeline;

        physics_info.physics_pipeline.step(
            &physics_info.gravity,
            &physics_info.integration_parameters,
            &mut physics_info.island_manager,
            &mut physics_info.broad_phase,
            &mut physics_info.narrow_phase,
            &mut physics_info.rigid_body_set,
            &mut physics_info.collider_set,
            &mut physics_info.impulse_joint_set,
            &mut physics_info.multibody_joint_set,
            &mut physics_info.ccd_solver,
            Some(query_pipeline),
            &physics_info.physics_hooks,
            &physics_info.event_handler,
        );
    }
}

impl System for RapierPhysicsSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let is_fixed_timestep;
        {
            let physics_info = entities_and_components
                .get_resource_mut::<RapierPhysicsInfo>()
                .expect("failed to get rapier physics info, report this as a bug");
            let time_started = &mut physics_info.time_started;
            let time_elapsed_before_this_frame = &mut physics_info.time_elapsed_before_this_frame;

            let time_since_last_step = time_started.elapsed() - *time_elapsed_before_this_frame;
            if time_since_last_step.as_secs_f64() >= 1.0 / 60.0 {
                *time_elapsed_before_this_frame = time_started.elapsed();
                is_fixed_timestep = true;
            } else {
                is_fixed_timestep = false;
            }
        }

        // set the delta time scale
        {
            let delta_time_scale = entities_and_components
                .get_resource::<delta_time::DeltaTime>()
                .expect("failed to get delta time, report this as a bug")
                .get_time_scale();

            let physics_info = entities_and_components
                .get_resource_mut::<RapierPhysicsInfo>()
                .expect("failed to get rapier physics info, report this as a bug");
            physics_info.integration_parameters.dt = (1.0 / 60.0) * delta_time_scale as f32;
        }

        {
            let physics_info;
            {
                let physics_info_ref = entities_and_components
                    .get_resource_mut::<RapierPhysicsInfo>()
                    .expect("failed to get rapier physics info, report this as a bug");

                let physics_info_ptr = physics_info_ref as *mut RapierPhysicsInfo;
                unsafe {
                    // SAFETY: we don't access physics info anywhere else in this function, so this is safe
                    // and physics info doesn't intersect with anythign else in the world
                    physics_info = &mut *physics_info_ptr;
                }
            }

            let mut rb_handles_found_this_frame = vec![];
            let mut collider_handles_found_this_frame = vec![];
            get_all_rigid_bodies_and_colliders(
                physics_info,
                entities_and_components,
                &mut rb_handles_found_this_frame,
                &mut collider_handles_found_this_frame,
            );

            handle_removed_entities(
                physics_info,
                &mut rb_handles_found_this_frame,
                &mut collider_handles_found_this_frame,
            );
        }

        if is_fixed_timestep {
            self.step(entities_and_components);
        }

        {
            let physics_info = &mut entities_and_components
                .get_resource_mut::<RapierPhysicsInfo>()
                .expect("failed to get rapier physics info, report this as a bug");

            let query_pipeline = &mut physics_info.query_pipeline;
            query_pipeline.update(&physics_info.rigid_body_set, &physics_info.collider_set);
        }

        // if anything is changed between the physics system and the set_all_rigid_bodies_and_colliders call, it will break it don't do that

        // it's not that this is a bad idea, it's just that it's not necessary so no need to waste performance unless a step is taken
        if is_fixed_timestep {
            let physics_info;
            {
                let physics_info_ref = entities_and_components
                    .get_resource::<RapierPhysicsInfo>()
                    .expect("failed to get rapier physics info, report this as a bug");

                let physics_info_ptr = physics_info_ref as *const RapierPhysicsInfo;
                unsafe {
                    // SAFETY: we don't access physics info anywhere else in this function, so this is safe
                    // and physics info doesn't intersect with anythign else in the world
                    physics_info = &*physics_info_ptr;
                }
            }
            set_all_rigid_bodies_and_colliders(physics_info, entities_and_components);
        }
    }
}

// just so that the user can't accidentally mess with up the internals of the physics system
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// the handle to a rigidbody in the physics world, do not add this to an entity manually. you will break the physics system
pub struct RigidBodyHandle(pub RapierRigidBodyHandle);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// the handle to a collider in the physics world, do not add this to an entity manually. you will break the physics system
pub struct ColliderHandle(pub RapierColliderHandle);

// a tag to temporarily store in an entity that the rigidbody has changed
struct RBHandleChanged;

impl From<rapier2d::prelude::RigidBodyHandle> for RigidBodyHandle {
    fn from(handle: rapier2d::prelude::RigidBodyHandle) -> Self {
        RigidBodyHandle(handle)
    }
}

impl From<rapier2d::prelude::ColliderHandle> for ColliderHandle {
    fn from(handle: rapier2d::prelude::ColliderHandle) -> Self {
        ColliderHandle(handle)
    }
}

/// This function gets all rigid bodies and colliders in the world and inserts them into the given sets
/// it promises to not access the physics info in any way other than the given reference
fn get_all_rigid_bodies_and_colliders(
    physics_info: &mut RapierPhysicsInfo,
    world: &mut EntitiesAndComponents,
    rb_handles_found: &mut Vec<RigidBodyHandle>,
    collider_handles_found: &mut Vec<ColliderHandle>,
) {
    let rigidbody_entities = world
        .get_entities_with_component::<RigidBody>()
        .into_iter()
        .copied()
        .collect::<Vec<Entity>>();

    for rigidbody_entity in rigidbody_entities {
        update_rb(physics_info, world, rigidbody_entity);

        // the entity should have a handle now
        if let Some(rb_handle) = world
            .try_get_components::<(RigidBodyHandle,)>(rigidbody_entity)
            .0
        {
            rb_handles_found.push(rb_handle.clone());
        }
    }

    let collider_entities = world
        .get_entities_with_component::<Collider>()
        .into_iter()
        .copied()
        .collect::<Vec<Entity>>();

    for collider_entity in collider_entities {
        update_collider(physics_info, world, collider_entity);

        // the entity should have a handle now
        if let Some(collider_handle) = world
            .try_get_components::<(ColliderHandle,)>(collider_entity)
            .0
        {
            collider_handles_found.push(collider_handle.clone());
        }
    }
}

fn handle_removed_entities(
    physics_info: &mut RapierPhysicsInfo,
    rb_handles_found: &mut Vec<RigidBodyHandle>,
    collider_handles_found: &mut Vec<ColliderHandle>,
) {
    // handle cases where entities are removed or rb's are removed
    {
        let mut rigidbody_entities_in_physics_copy = physics_info.rigid_body_handle_map.clone();

        for rb_handle in rb_handles_found {
            rigidbody_entities_in_physics_copy.remove(&rb_handle);
        }

        for (rb_handle, rb_entity) in rigidbody_entities_in_physics_copy {
            physics_info.rigid_body_set.remove(
                rb_handle.0,
                &mut physics_info.island_manager,
                &mut physics_info.collider_set,
                &mut physics_info.impulse_joint_set,
                &mut physics_info.multibody_joint_set,
                false,
            );

            physics_info.rigid_body_handle_map.remove(&rb_handle);
        }

        let mut collider_entities_in_physics_copy = physics_info.collider_handle_map.clone();

        for collider_handle in collider_handles_found.clone() {
            collider_entities_in_physics_copy.remove(&collider_handle);
        }

        for (collider_handle, collider_path) in collider_entities_in_physics_copy {
            physics_info.collider_set.remove(
                collider_handle.0,
                &mut physics_info.island_manager,
                &mut physics_info.rigid_body_set,
                false,
            );

            physics_info.collider_handle_map.remove(&collider_handle);
        }
    }
}

/// this fn promises to not access the physics info in any way other than the given reference
fn update_rb(
    physics_info: &mut RapierPhysicsInfo,
    world: &mut EntitiesAndComponents,
    rigidbody_entity: Entity,
) {
    let transform = crate::get_transform_recursive(rigidbody_entity, world, Transform::default());

    let (rigidbody, rigidbody_handle) =
        world.try_get_components_mut::<(RigidBody, RigidBodyHandle)>(rigidbody_entity);

    let out_rigid_body_set = &mut physics_info.rigid_body_set;
    let out_rigid_body_entity_map = &mut physics_info.rigid_body_handle_map;

    match (rigidbody, rigidbody_handle) {
        (Some(ecs_rigidbody), Some(rigidbody_handle)) => {
            // the entity has a handle, which means it is already in the set, so we just need to update the rigidbody
            // get the rigidbody from the handle
            let rigidbody = out_rigid_body_set.get_mut(rigidbody_handle.0);
            if let Some(rigidbody) = rigidbody {
                // i think this is better than copying and then changing the non-ecs-rb
                ecs_rigidbody.set_position(abc_transform_to_rapier_transform(transform), false);

                rigidbody.copy_from(&ecs_rigidbody.clone());
            } else {
                let (new_rb_handle, rb_handle_changed) = add_new_rb(
                    rigidbody_entity,
                    ecs_rigidbody,
                    out_rigid_body_set,
                    out_rigid_body_entity_map,
                    transform,
                );

                // add a handle to the rigidbody to the entity, overwriting the old handle
                world.add_component_to(rigidbody_entity, new_rb_handle);
                world.add_component_to(rigidbody_entity, rb_handle_changed);
            }
        }
        (Some(ecs_rigidbody), None) => {
            // if the rigidbody doesn't have a handle, insert it into the set, and add a handle to the entity

            let (new_rb_handle, rb_handle_changed) = add_new_rb(
                rigidbody_entity,
                ecs_rigidbody,
                out_rigid_body_set,
                out_rigid_body_entity_map,
                transform,
            );

            world.add_component_to(rigidbody_entity, new_rb_handle);
            world.add_component_to(rigidbody_entity, rb_handle_changed);
        }
        _ => {}
    }
}

fn add_new_rb(
    entity: Entity,
    rigidbody: &mut RigidBody,
    out_rigid_body_set: &mut RigidBodySet,
    out_rigid_body_entity_map: &mut std::collections::HashMap<RigidBodyHandle, Entity>,
    transform: Transform,
) -> (RigidBodyHandle, RBHandleChanged) {
    rigidbody.set_position(abc_transform_to_rapier_transform(transform), true);

    // insert the rigidbody into the set
    let new_rb_handle = out_rigid_body_set.insert(rigidbody.clone());

    // add new one to the map
    out_rigid_body_entity_map.insert(RigidBodyHandle(new_rb_handle), entity);

    (RigidBodyHandle(new_rb_handle), RBHandleChanged)
}

fn update_collider(
    physics_info: &mut RapierPhysicsInfo,
    world: &mut EntitiesAndComponents,
    entity: Entity,
) {
    let (collider, transform, collider_handle, rb_handle, handle_has_changed) = world
        .try_get_components::<(
            Collider,
            Transform,
            ColliderHandle,
            RigidBodyHandle,
            RBHandleChanged,
        )>(entity);

    let collider_set = &mut physics_info.collider_set;
    let rigid_body_set = &mut physics_info.rigid_body_set;

    let out_collider_entity_map = &mut physics_info.collider_handle_map;

    match (collider, transform, collider_handle) {
        (Some(ecs_collider), Some(_), Some(collider_handle)) => {
            if let Some(_) = handle_has_changed {
                let new_collider_handle = add_new_collider(
                    entity,
                    ecs_collider,
                    rb_handle,
                    rigid_body_set,
                    collider_set,
                    out_collider_entity_map,
                );

                // add a handle to the collider to the entity
                world.add_component_to(entity, new_collider_handle);
                world.remove_component_from::<RBHandleChanged>(entity);
            } else {
                // the entity has a handle, which means it is already in the set, so we just need to update the collider
                // get the collider from the handle
                let collider = collider_set.get_mut(collider_handle.0);

                if let Some(collider) = collider {
                    collider.copy_from(&ecs_collider.clone());
                } else {
                    // this means the handle is invalid, so we should insert the collider into the set
                    let new_collider_handle = add_new_collider(
                        entity,
                        ecs_collider,
                        rb_handle,
                        rigid_body_set,
                        collider_set,
                        out_collider_entity_map,
                    );

                    // add a handle to the collider to the entity
                    world.add_component_to(entity, new_collider_handle);
                }
            }
        }
        (Some(collider), Some(_), None) => {
            // if the collider doesn't have a handle, insert it into the set, and add a handle to the entity
            let new_collider_handle = add_new_collider(
                entity,
                collider,
                rb_handle,
                rigid_body_set,
                collider_set,
                out_collider_entity_map,
            );

            // add a handle to the collider to the entity
            world.add_component_to(entity, new_collider_handle);
        }
        _ => {
            // log warning that collider is missing transform
            event!(
                Level::WARN,
                "collider is missing transform, the collider will not be simulated without one"
            );
        }
    }
}

/// This function adds a new collider to the world and adds a handle to the entity
fn add_new_collider(
    entity: Entity,
    collider: &Collider,
    rb_handle: Option<&RigidBodyHandle>,
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    out_collider_entity_map: &mut std::collections::HashMap<ColliderHandle, Entity>,
) -> ColliderHandle {
    let collider = collider.clone();

    let new_collider_handle = if let Some(rigidbody_handle) = rb_handle {
        collider_set.insert_with_parent(collider, rigidbody_handle.0, rigid_body_set)
    } else {
        collider_set.insert(collider)
    };

    out_collider_entity_map.insert(ColliderHandle(new_collider_handle), entity);

    ColliderHandle(new_collider_handle)
}

/// This function updates the transforms of all rigid bodies and colliders in the world
fn set_all_rigid_bodies_and_colliders(
    physics_info: &RapierPhysicsInfo,
    world: &mut EntitiesAndComponents,
) {
    let rigid_body_set = &physics_info.rigid_body_set;
    let collider_set = &physics_info.collider_set;

    for (rb_handle, entity) in physics_info.rigid_body_handle_map.iter() {
        let transform_total = crate::get_transform_recursive(*entity, world, Transform::default());

        let (rigidbody, transform, rigidbody_handle) =
            world.try_get_components_mut::<(RigidBody, Transform, RigidBodyHandle)>(*entity);

        if transform.is_none() {
            // log warning that rigidbody is missing transform
            event!(
                Level::WARN,
                "rigidbody is missing transform, the rigidbody will not be simulated without one"
            );
            continue;
        }

        let transform = transform.unwrap();
        let transform_offset = &transform_total - &transform.clone();

        match (rigidbody, rigidbody_handle) {
            (Some(ecs_rigidbody), Some(rigidbody_handle)) => {
                let rigidbody = rigid_body_set.get(rigidbody_handle.0).expect("failed to get rigidbody from handle found in entity, please report this as a bug on abc game engine github page");
                update_abc_transform_from_rapier_transform(
                    transform,
                    transform_offset,
                    *rigidbody.position(),
                );

                *ecs_rigidbody = rigidbody.clone();
            }
            _ => {}
        }
    }

    for (collider_handle, entity) in physics_info.collider_handle_map.iter() {
        let (collider, transform, collider_handle) =
            world.try_get_components_mut::<(Collider, Transform, ColliderHandle)>(*entity);
        if let (Some(ecs_collider), Some(transform), Some(collider_handle)) =
            (collider, transform, collider_handle)
        {
            let collider = collider_set.get(collider_handle.0).expect(
                "failed to get collider from handle found in entity, please report this as a bug",
            );

            *ecs_collider = collider.clone();
        } else {
            // log warning that collider is missing transform
            event!(
                Level::WARN,
                "collider is missing transform, the collider will not be simulated without one"
            );
        }
    }
}

fn abc_transform_to_rapier_transform(transform: Transform) -> Isometry<Real> {
    let new_transform = Isometry::new(
        vector![transform.x as f32, transform.y as f32],
        transform.rotation as f32,
    );
    new_transform
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
