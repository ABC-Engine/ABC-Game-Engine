use crate::camera::Camera;
use crate::{
    render_circle, render_rectangle, render_texture, Color, Scene, SceneParams, Transform,
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

/// Renderer is responsible for rendering the scene
pub struct Renderer {
    // width and height are determined by the camera,
    // but needs to be on the renderer for buffer size
    width: u32,
    height: u32,
    stretch: f32,
    pixel_scale: u16,
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
            width: 160,
            height: 160,
            stretch: 2.3,
            last_pixel_grid: vec![],
            handle,
            pixel_scale: 1,
        }
    }

    pub fn set_stretch(&mut self, stretch: f32) {
        self.stretch = stretch;
    }

    pub fn set_pixel_scale(&mut self, pixel_scale: u16) {
        self.pixel_scale = pixel_scale;

        crossterm::queue!(
            self.handle,
            cursor::Hide,
            crossterm::terminal::SetSize(
                self.width as u16 * self.pixel_scale,
                self.height as u16 * self.pixel_scale
            ),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        )
        .expect("Error: failed to set terminal size");
    }

    ///  Renders the scene
    pub fn render(&mut self, scene: &mut EntitiesAndComponents, scene_params: &SceneParams) {
        let mut pixel_grid =
            vec![vec![scene_params.background_color; self.width as usize]; self.height as usize];

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
                    if self.width != camera_component.width as u32
                        || self.height != camera_component.height as u32
                    {
                        self.width = camera_component.width as u32;
                        self.height = camera_component.height as u32;
                        crossterm::queue!(
                            self.handle,
                            cursor::Hide,
                            crossterm::terminal::SetSize(
                                self.width as u16 * self.pixel_scale,
                                self.height as u16 * self.pixel_scale
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
                        x: -camera_transform.x + self.width as f64 / 2.0,
                        y: -camera_transform.y + self.height as f64 / 2.0,
                        z: 0.0,
                        rotation: -camera_transform.rotation,
                        scale: 1.0 / camera_transform.scale,
                        origin_x: -camera_transform.origin_x,
                        origin_y: -camera_transform.origin_y,
                    };

                    self.render_objects(scene, &mut pixel_grid, opposite_camera_transform.clone());
                    break;
                }
            }
        }
        self.render_pixel_grid(&pixel_grid, scene_params);
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
            for entity in entities_with_sprite {
                let (sprite, transform) = entities_and_components
                    .try_get_components_mut::<(Sprite, Transform)>(
                        entity, // can't fail unless done multithreaded in the future
                    );
                if let (Some(sprite), Some(transform)) = (sprite, transform) {
                    entity_depth_array.push(EntityDepthItem {
                        entity,
                        depth: transform.z,
                    });
                }
            }
        }
        entity_depth_array.sort();

        // could possibly be done multithreaded and combine layers afterward
        for entity_depth_item in entity_depth_array {
            let entity = entity_depth_item.entity;

            let (sprite, transform) = entities_and_components
                .try_get_components_mut::<(Sprite, Transform)>(entity_depth_item.entity);
            {
                // if the object doesn't have a sprite or transform, don't render it
                match (sprite, transform) {
                    (None, None) => continue,
                    (Some(sprite), Some(transform)) => {
                        let transform = &transform.clone();
                        // check if object is circle or rectangle
                        match sprite {
                            Sprite::Circle(circle) => render_circle(
                                &circle,
                                &(transform + &transform_offset),
                                pixel_grid,
                                self.stretch,
                            ),
                            Sprite::Rectangle(rectangle) => render_rectangle(
                                &rectangle,
                                &(transform + &transform_offset),
                                pixel_grid,
                                self.stretch,
                            ),
                            Sprite::Image(image) => render_texture(
                                &image.texture,
                                &(transform + &transform_offset),
                                pixel_grid,
                                self.stretch,
                            ),
                            Sprite::Animation(animation) => {
                                update_animation(animation);
                                let current_frame = &animation.frames[animation.current_frame];
                                render_texture(
                                    &current_frame.texture,
                                    &(transform + &transform_offset),
                                    pixel_grid,
                                    self.stretch,
                                );
                            }
                        }
                    }
                    // can no longer render an object with a sprite but no transform
                    // because the transform is used as an offset
                    (Some(sprite), None) => {}
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

    pub fn render_pixel_grid(&mut self, pixel_grid: &Vec<Vec<Color>>, scene_params: &SceneParams) {
        // if the pixel scale is greater than 1, scale the pixel grid
        // make sure that the pixel grid is not already scaled
        if self.pixel_scale != 1 && pixel_grid.len() <= self.width as usize {
            let mut scaled_pixel_grid =
                vec![
                    vec![Color::default(); (self.width * self.pixel_scale as u32) as usize];
                    (self.height * self.pixel_scale as u32) as usize
                ];

            for (x, row) in pixel_grid.into_iter().enumerate() {
                for (y, pixel) in row.into_iter().enumerate() {
                    for i in 0..self.pixel_scale {
                        for j in 0..self.pixel_scale {
                            scaled_pixel_grid[x * self.pixel_scale as usize + i as usize]
                                [y * self.pixel_scale as usize + j as usize] = *pixel;
                        }
                    }
                }
            }
            self.render_pixel_grid(&scaled_pixel_grid, scene_params);
            return;
        }

        let mut pixel_character = "".to_string();
        for (x, row) in pixel_grid.into_iter().enumerate() {
            for (y, pixel) in row.into_iter().enumerate() {
                // if the pixel is the same as the last pixel, don't render it
                if self.last_pixel_grid.len() != 0 && *pixel == self.last_pixel_grid[x][y] {
                    continue;
                }
                crossterm::queue!(self.handle, cursor::MoveTo(y as u16, x as u16))
                    .expect("Failed to move cursor");

                // \x08 is backspace
                if pixel.a == 0.0 {
                    write!(self.handle, "\x08{}", " ").expect("failed to write white space");
                } else {
                    if scene_params.is_random_chars {
                        pixel_character +=
                            &char::from(rand::thread_rng().gen_range(33..126)).to_string();
                    } else {
                        pixel_character += &scene_params.character.to_string();
                    }

                    write!(
                        self.handle,
                        "\x08{}",
                        pixel_character.truecolor(pixel.r, pixel.g, pixel.b)
                    )
                    .expect("failed to write pixel");
                    pixel_character.clear();
                }
            }
        }

        self.handle.flush().expect("failed to flush stdout");
        self.last_pixel_grid = pixel_grid.clone();
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
