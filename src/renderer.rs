use crate::camera::{self, Camera};
use crate::{
    render_circle, render_circle_with_mask, render_rectangle, render_rectangle_with_mask,
    render_texture, render_texture_with_mask, Color, Scene, SceneParams, Transform,
};
use colored::Colorize;
use crossterm::cursor;
use rand::Rng;
use std::{
    io::Write,
    time::{Duration, Instant},
    vec,
};
use ABC_ECS::{EntitiesAndComponents, Entity, System, TryComponentsRef};

use self::mask::Mask;
pub mod ascii_renderer;
pub mod canvas_renderer;
pub mod mask;

#[derive(Clone, Copy)]
pub struct Circle {
    pub radius: f64,
    pub color: Color,
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

#[derive(Clone)]
pub struct Texture {
    pub pixels: Vec<Vec<Color>>, // not sure how inefficient this is but it will do for now
}

// rectangle with texture
#[derive(Clone)]
pub struct Image {
    // height and width are in texture
    pub texture: Texture,
}

#[derive(Clone)]
pub struct Animation {
    pub frames: Vec<Image>,
    pub current_frame: usize,
    pub frame_time: Duration,
    pub current_frame_start_time: Instant,
    pub loop_animation: bool,
    pub finished: bool,
}

/// Sprite is an enum that can be either a circle or a rectangle
#[derive(Clone)]
pub enum Sprite {
    Circle(Circle),
    Rectangle(Rectangle),
    Image(Image),
    Animation(Animation),
}

impl From<Circle> for Sprite {
    fn from(circle: Circle) -> Self {
        Sprite::Circle(circle)
    }
}

impl From<Rectangle> for Sprite {
    fn from(rectangle: Rectangle) -> Self {
        Sprite::Rectangle(rectangle)
    }
}

impl From<Image> for Sprite {
    fn from(image: Image) -> Self {
        Sprite::Image(image)
    }
}

struct RendererParams {
    // width and height are determined by the camera,
    // but needs to be on the renderer for buffer size
    width: u32,
    height: u32,
    stretch: f32,
    pixel_scale: u16,
}

/// Renderer is responsible for rendering the scene
pub struct Renderer {
    renderer_params: RendererParams,
    scene_params: SceneParams,
    // used for diffing
    // will be empty if no previous frame
    last_pixel_grid: Vec<Vec<Color>>,
    handle: std::io::BufWriter<std::io::StdoutLock<'static>>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let stdout = std::io::stdout().lock();
        let mut handle = std::io::BufWriter::with_capacity(8192, stdout);
        crossterm::queue!(
            handle,
            cursor::Hide,
            crossterm::terminal::SetSize(160 as u16, 160 as u16),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        )
        .expect("Error: failed to set terminal size");

        Renderer {
            renderer_params: RendererParams {
                width: 160,
                height: 160,
                stretch: 2.3,
                pixel_scale: 1,
            },
            scene_params: SceneParams {
                background_color: Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 1.0,
                },
                is_random_chars: false,
                character: '=',
            },
            last_pixel_grid: vec![],
            handle,
        }
    }

    pub fn set_stretch(&mut self, stretch: f32) {
        self.renderer_params.stretch = stretch;
    }

    pub fn set_pixel_scale(&mut self, pixel_scale: u16) {
        self.renderer_params.pixel_scale = pixel_scale;

        crossterm::queue!(
            self.handle,
            cursor::Hide,
            crossterm::terminal::SetSize(
                self.renderer_params.width as u16 * self.renderer_params.pixel_scale,
                self.renderer_params.height as u16 * self.renderer_params.pixel_scale
            ),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        )
        .expect("Error: failed to set terminal size");
    }

    pub fn set_scene_params(&mut self, scene_params: SceneParams) {
        self.scene_params = scene_params;
    }

    pub fn get_scene_params(&self) -> SceneParams {
        self.scene_params
    }

    ///  Renders the scene
    pub fn render(&mut self, scene: &mut EntitiesAndComponents) {
        let scene_params;
        {
            scene_params = self.scene_params.clone();
        }

        let mut pixel_grid =
            vec![
                vec![scene_params.background_color; self.renderer_params.width as usize];
                self.renderer_params.height as usize
            ];

        let camera_entities = scene
            .get_entities_with_component::<Camera>()
            .cloned()
            .collect::<Vec<Entity>>();
        if camera_entities.len() == 0 {
            panic!("renderer could not find a camera");
        } else {
            // this will not panic if no active camera is found
            for camera_entity in camera_entities {
                let camera_component: Camera;
                {
                    let camera_component_ref = scene
                        .try_get_component::<Camera>(camera_entity)
                        .expect("renderer could not find a camera");
                    camera_component = (&**camera_component_ref).clone();
                }

                if camera_component.is_active == true {
                    if self.renderer_params.width != camera_component.width as u32
                        || self.renderer_params.height != camera_component.height as u32
                    {
                        self.renderer_params.width = camera_component.width as u32;
                        self.renderer_params.height = camera_component.height as u32;
                        crossterm::queue!(
                            self.handle,
                            cursor::Hide,
                            crossterm::terminal::SetSize(
                                self.renderer_params.width as u16
                                    * self.renderer_params.pixel_scale,
                                self.renderer_params.height as u16
                                    * self.renderer_params.pixel_scale
                            ),
                            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                        )
                        .expect("Error: failed to set terminal size");

                        pixel_grid = vec![
                            vec![
                                scene_params.background_color;
                                self.renderer_params.width as usize
                            ];
                            self.renderer_params.height as usize
                        ];
                    }

                    let camera_transform = scene
                        .try_get_components::<(Transform,)>(camera_entity)
                        .0
                        .expect("active camera does not have a transform!");

                    let opposite_camera_transform = Transform {
                        x: -camera_transform.x + self.renderer_params.width as f64 / 2.0,
                        y: -camera_transform.y + self.renderer_params.height as f64 / 2.0,
                        z: 0.0,
                        rotation: -camera_transform.rotation,
                        scale: 1.0 / camera_transform.scale,
                        origin_x: 0.0,
                        origin_y: 0.0,
                    };

                    self.render_objects(
                        scene,
                        &mut pixel_grid,
                        opposite_camera_transform.clone(),
                        &camera_component,
                    );
                    break;
                }
            }
        }

        ascii_renderer::render_pixel_grid(self, &pixel_grid, &scene_params);
    }

    // not thread safe
    fn render_objects(
        &self,
        entities_and_components: &mut EntitiesAndComponents,
        pixel_grid: &mut Vec<Vec<Color>>,
        camera_offset: Transform,
        camera: &Camera,
    ) {
        let mut entity_depth_array = vec![];

        collect_renderable_entities(
            &entities_and_components,
            vec![],
            &camera_offset,
            &mut entity_depth_array,
        );

        entity_depth_array.sort();

        // could possibly be done multithreaded and combine layers afterward
        for entity_depth_item in entity_depth_array {
            let entities = entity_depth_item.entity;

            let (current_entities_and_components, entity) =
                get_entities_and_components_from_entity_list(entities_and_components, entities);

            let (sprite, mask, transform) = current_entities_and_components
                .try_get_components_mut::<(Sprite, Mask, Transform)>(entity);
            {
                // if the object doesn't have a sprite or transform, don't render it
                match (sprite, mask, transform) {
                    (Some(sprite), None, Some(_)) => {
                        let transform = &(entity_depth_item.transform);

                        if !camera::object_is_in_view_of_camera(camera, transform, sprite) {
                            continue;
                        }

                        // check if object is circle or rectangle
                        match sprite {
                            Sprite::Circle(circle) => render_circle(
                                &circle,
                                &transform,
                                pixel_grid,
                                self.renderer_params.stretch,
                            ),
                            Sprite::Rectangle(rectangle) => render_rectangle(
                                &rectangle,
                                &transform,
                                pixel_grid,
                                self.renderer_params.stretch,
                            ),
                            Sprite::Image(image) => render_texture(
                                &image.texture,
                                &transform,
                                pixel_grid,
                                self.renderer_params.stretch,
                            ),
                            Sprite::Animation(animation) => {
                                update_animation(animation);
                                let current_frame = &animation.frames[animation.current_frame];
                                render_texture(
                                    &current_frame.texture,
                                    &transform,
                                    pixel_grid,
                                    self.renderer_params.stretch,
                                );
                            }
                        }
                    }
                    (Some(sprite), Some(mask), Some(transform)) => {
                        let transform = &(&transform.clone() + &entity_depth_item.transform);
                        // check if object is circle or rectangle
                        match sprite {
                            Sprite::Circle(circle) => render_circle_with_mask(
                                &circle,
                                transform,
                                pixel_grid,
                                self.renderer_params.stretch,
                                mask,
                            ),
                            Sprite::Rectangle(rectangle) => render_rectangle_with_mask(
                                &rectangle,
                                transform,
                                pixel_grid,
                                self.renderer_params.stretch,
                                mask,
                            ),
                            Sprite::Image(image) => render_texture_with_mask(
                                &image.texture,
                                transform,
                                pixel_grid,
                                self.renderer_params.stretch,
                                mask,
                            ),
                            Sprite::Animation(animation) => {
                                update_animation(animation);
                                let current_frame = &animation.frames[animation.current_frame];
                                render_texture(
                                    &current_frame.texture,
                                    transform,
                                    pixel_grid,
                                    self.renderer_params.stretch,
                                );
                            }
                        }
                    }
                    // can no longer render an object with a sprite but no transform
                    // because the transform is used as an offset
                    (Some(_), None, None) => (),
                    _ => (),
                }
            }
        }
    }
}

fn update_animation(animation: &mut Animation) {
    if !animation.finished && animation.current_frame_start_time.elapsed() >= animation.frame_time {
        animation.current_frame_start_time = Instant::now();
        animation.current_frame += 1;
        if animation.current_frame >= animation.frames.len() {
            if animation.loop_animation {
                animation.current_frame = 0;
            } else {
                animation.finished = true;
            }
        }
    }
}

/// A recursive function that collects all renderable entities in the scene
fn collect_renderable_entities(
    entities_and_components: &EntitiesAndComponents,
    // the list of parent entities to get to the EntitiesAndComponents that is passed, starting with the root
    parent_entities: Vec<Entity>,
    transform_offset: &Transform,
    out_list: &mut Vec<EntityDepthItem>,
) {
    let entities_with_sprite = entities_and_components
        .get_entities_with_component::<Sprite>()
        .cloned()
        .collect::<Vec<Entity>>();

    for entity in entities_with_sprite {
        let (sprite, transform) =
            entities_and_components.try_get_components::<(Sprite, Transform)>(entity);

        match (sprite, transform) {
            (Some(_), Some(transform)) => {
                let mut new_parents = parent_entities.clone();
                new_parents.push(entity);
                out_list.push(EntityDepthItem {
                    entity: new_parents,
                    transform: transform + transform_offset,
                });
            }
            _ => (),
        }
    }

    let entities_with_children = entities_and_components
        .get_entities_with_component::<EntitiesAndComponents>()
        .cloned()
        .collect::<Vec<Entity>>();

    for entity in entities_with_children {
        let (transform, children) = entities_and_components
            .try_get_components::<(Transform, EntitiesAndComponents)>(entity);

        match (transform, children) {
            (Some(transform), Some(children)) => {
                let mut new_parents = parent_entities.clone();
                new_parents.push(entity);
                collect_renderable_entities(
                    children,
                    new_parents,
                    &(transform_offset + transform),
                    out_list,
                )
            }
            (None, Some(children)) => {
                let mut new_parents = parent_entities.clone();
                new_parents.push(entity);
                collect_renderable_entities(children, new_parents, transform_offset, out_list)
            }
            _ => (),
        }
    }
}

/// takes a Vec<Entity> and returns the EntitiesAndComponents and Entity that it points to
fn get_entities_and_components_from_entity_list(
    entities_and_components: &mut EntitiesAndComponents,
    mut entity_list: Vec<Entity>,
) -> (&mut EntitiesAndComponents, Entity) {
    if entity_list.len() == 0 {
        panic!("entity list is empty, this should never happen, please report this as a bug");
    }
    if entity_list.len() == 1 {
        return (entities_and_components, entity_list[0]);
    }

    let mut current_entities_and_components = entities_and_components;
    let mut current_entity = entity_list[0];
    // the last entity in the list is the one we want to return, and it's not a parent so no need to check for children
    let last_entity = entity_list.pop().unwrap();

    for entity in entity_list {
        let children = current_entities_and_components
            .try_get_components_mut::<(EntitiesAndComponents,)>(current_entity)
            .0
            .expect(
                "failed to get children, this should never happen, please report this as a bug",
            );

        current_entities_and_components = children;
        current_entity = entity;
    }
    (current_entities_and_components, last_entity)
}

struct EntityDepthItem {
    /// ordered by child depth, so entity1 has entity2 as a child which has entity3 as a child
    /// entity1 will not be rendered as part of the pass for this object just entity3.
    /// entity1 and entity 2 will have its own pass
    entity: Vec<Entity>,
    transform: Transform,
}

impl Eq for EntityDepthItem {}

impl PartialEq for EntityDepthItem {
    fn eq(&self, other: &Self) -> bool {
        self.transform.z == other.transform.z
    }
}

impl PartialOrd for EntityDepthItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.transform.z.partial_cmp(&other.transform.z)
    }
}

impl Ord for EntityDepthItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.transform
            .z
            .partial_cmp(&other.transform.z)
            .expect("failed to compare entity depth")
    }
}
