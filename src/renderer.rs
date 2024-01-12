use crate::camera::Camera;
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
use ABC_ECS::{EntitiesAndComponents, Entity, TryComponentsRef};

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

pub enum RendererType {
    Ascii,
    Canvas,
}

struct RendererParams {
    // width and height are determined by the camera,
    // but needs to be on the renderer for buffer size
    width: u32,
    height: u32,
    stretch: f32,
    pixel_scale: u16,
    renderer_type: RendererType,
}

/// Renderer is responsible for rendering the scene
pub struct Renderer {
    renderer_params: RendererParams,
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
                renderer_type: RendererType::Ascii,
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

    pub fn set_renderer_type(&mut self, renderer_type: RendererType) {
        self.renderer_params.renderer_type = renderer_type;
    }

    ///  Renders the scene
    pub fn render(&mut self, scene: &mut EntitiesAndComponents, scene_params: &SceneParams) {
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
                let camera_component = scene
                    .try_get_component::<Camera>(camera_entity)
                    .expect("renderer could not find a camera");

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

                    self.render_objects(scene, &mut pixel_grid, opposite_camera_transform.clone());
                    break;
                }
            }
        }

        match self.renderer_params.renderer_type {
            RendererType::Ascii => {
                ascii_renderer::render_pixel_grid(self, &pixel_grid, scene_params)
            }
            RendererType::Canvas => {
                canvas_renderer::render_pixel_grid(self, &pixel_grid, scene_params)
            }
        }
    }

    // not thread safe
    fn render_objects(
        &self,
        entities_and_components: &mut EntitiesAndComponents,
        pixel_grid: &mut Vec<Vec<Color>>,
        transform_offset: Transform,
    ) {
        let mut entity_depth_array = vec![];
        {
            let entities_with_sprite = entities_and_components
                .get_entities_with_component::<Sprite>()
                .cloned()
                .collect::<Vec<Entity>>();

            // sort entities by z value
            // skips entities without a sprite and transform or children
            for entity in entities_with_sprite {
                let (sprite, transform) = entities_and_components
                    .try_get_components_mut::<(Sprite, Transform)>(
                        entity, // can't fail unless done multithreaded in the future
                    );
                match (sprite, transform) {
                    (Some(_), Some(transform)) => {
                        entity_depth_array.push(EntityDepthItem {
                            entity,
                            depth: transform_offset.z + transform.z,
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
                    .try_get_components_mut::<(Transform, EntitiesAndComponents)>(entity);

                match (transform, children) {
                    (Some(transform), Some(_)) => {
                        entity_depth_array.push(EntityDepthItem {
                            entity,
                            depth: transform_offset.z + transform.z,
                        });
                    }
                    (None, Some(_)) => {
                        entity_depth_array.push(EntityDepthItem {
                            entity,
                            depth: transform_offset.z,
                        });
                    }
                    _ => (),
                }
            }
        }
        entity_depth_array.sort();

        // could possibly be done multithreaded and combine layers afterward
        for entity_depth_item in entity_depth_array {
            let entity = entity_depth_item.entity;

            let (sprite, mask, transform) = entities_and_components
                .try_get_components_mut::<(Sprite, Mask, Transform)>(entity_depth_item.entity);
            {
                // if the object doesn't have a sprite or transform, don't render it
                match (sprite, mask, transform) {
                    (Some(sprite), None, Some(transform)) => {
                        let transform = &transform.clone();
                        // check if object is circle or rectangle
                        match sprite {
                            Sprite::Circle(circle) => render_circle(
                                &circle,
                                &(transform + &transform_offset),
                                pixel_grid,
                                self.renderer_params.stretch,
                            ),
                            Sprite::Rectangle(rectangle) => render_rectangle(
                                &rectangle,
                                &(transform + &transform_offset),
                                pixel_grid,
                                self.renderer_params.stretch,
                            ),
                            Sprite::Image(image) => render_texture(
                                &image.texture,
                                &(transform + &transform_offset),
                                pixel_grid,
                                self.renderer_params.stretch,
                            ),
                            Sprite::Animation(animation) => {
                                update_animation(animation);
                                let current_frame = &animation.frames[animation.current_frame];
                                render_texture(
                                    &current_frame.texture,
                                    &(transform + &transform_offset),
                                    pixel_grid,
                                    self.renderer_params.stretch,
                                );
                            }
                        }
                    }
                    (Some(sprite), Some(mask), Some(transform)) => {
                        let transform = &transform.clone();
                        // check if object is circle or rectangle
                        match sprite {
                            Sprite::Circle(circle) => render_circle_with_mask(
                                &circle,
                                &(transform + &transform_offset),
                                pixel_grid,
                                self.renderer_params.stretch,
                                mask,
                            ),
                            Sprite::Rectangle(rectangle) => render_rectangle_with_mask(
                                &rectangle,
                                &(transform + &transform_offset),
                                pixel_grid,
                                self.renderer_params.stretch,
                                mask,
                            ),
                            Sprite::Image(image) => render_texture_with_mask(
                                &image.texture,
                                &(transform + &transform_offset),
                                pixel_grid,
                                self.renderer_params.stretch,
                                mask,
                            ),
                            Sprite::Animation(animation) => {
                                update_animation(animation);
                                let current_frame = &animation.frames[animation.current_frame];
                                render_texture(
                                    &current_frame.texture,
                                    &(transform + &transform_offset),
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

            // if the object has a transform and children, render the children with the transform as an offset
            if let (Some(children), Some(transform)) = entities_and_components
                .try_get_components_mut::<(EntitiesAndComponents, Transform)>(
                    entity, // again, can't fail unless done multithreaded in the future
                )
            {
                let transform = &*transform;
                self.render_objects(children, pixel_grid, transform + &transform_offset);
            }
            // if the object has children but no transform, render the children without any offset
            else if let Some(children) = entities_and_components
                .try_get_component_mut::<EntitiesAndComponents>(
                    entity, // again, can't fail unless done multithreaded in the future
                )
            {
                self.render_objects(children, pixel_grid, transform_offset);
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

struct EntityDepthItem {
    entity: Entity,
    depth: f64,
}

impl Eq for EntityDepthItem {}

impl PartialEq for EntityDepthItem {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth
    }
}

impl PartialOrd for EntityDepthItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.depth.partial_cmp(&other.depth)
    }
}

impl Ord for EntityDepthItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.depth
            .partial_cmp(&other.depth)
            .expect("failed to compare entity depth")
    }
}
