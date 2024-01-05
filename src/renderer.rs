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
    pub fn new(width: u32, height: u32) -> Renderer {
        let stdout = std::io::stdout().lock();
        let mut handle = std::io::BufWriter::with_capacity(8192, stdout);
        crossterm::queue!(
            handle,
            cursor::Hide,
            crossterm::terminal::SetSize(width as u16, height as u16),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        )
        .expect("Error: failed to set terminal size");

        Renderer {
            width,
            height,
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

        self.render_objects(scene, &mut pixel_grid, Transform::default());
        self.render_pixel_grid(&pixel_grid, scene_params);
    }

    fn render_objects(
        &self,
        entities_and_components: &mut EntitiesAndComponents,
        pixel_grid: &mut Vec<Vec<Color>>,
        transform_offset: Transform,
    ) {
        // could possibly be done multithreaded and combine layers afterward
        for i in 0..entities_and_components.get_entity_count() {
            {
                let (sprite, transform) = entities_and_components
                    .try_get_components_mut::<(Sprite, Transform)>(
                        entities_and_components.get_nth_entity(i).unwrap(), // can't fail unless done multithreaded in the future
                    );
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
                    (Some(sprite), None) => match sprite {
                        Sprite::Circle(circle) => render_circle(
                            &circle,
                            &(&Transform::default() + &transform_offset),
                            pixel_grid,
                            self.stretch,
                        ),
                        Sprite::Rectangle(rectangle) => render_rectangle(
                            &rectangle,
                            &(&Transform::default() + &transform_offset),
                            pixel_grid,
                            self.stretch,
                        ),
                        Sprite::Image(image) => render_texture(
                            &image.texture,
                            &(&Transform::default() + &transform_offset),
                            pixel_grid,
                            self.stretch,
                        ),
                        Sprite::Animation(animation) => {
                            update_animation(animation);
                            let current_frame = &animation.frames[animation.current_frame];
                            render_texture(
                                &current_frame.texture,
                                &(&Transform::default() + &transform_offset),
                                pixel_grid,
                                self.stretch,
                            );
                        }
                    },
                    _ => (),
                }
            }

            // if the object has a transform and children, render the children with the transform as an offset
            if let (Some(children), Some(transform)) = entities_and_components
                .try_get_components_mut::<(EntitiesAndComponents, Transform)>(
                    entities_and_components.get_nth_entity(i).unwrap(), // again, can't fail unless done multithreaded in the future
                )
            {
                let transform = &*transform;
                self.render_objects(children, pixel_grid, transform + &transform_offset);
            }
            // if the object has children but no transform, render the children without any offset
            else if let Some(children) = entities_and_components
                .try_get_component_mut::<EntitiesAndComponents>(
                    entities_and_components.get_nth_entity(i).unwrap(), // again, can't fail unless done multithreaded in the future
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
