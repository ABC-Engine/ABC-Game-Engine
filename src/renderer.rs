use crate::{
    render_circle, render_rectangle, render_texture, Color, Scene, SceneParams, Sprite, Transform,
};
use colored::Colorize;
use crossterm::cursor;
use rand::Rng;
use std::{io::Write, vec};
use ABC_ECS::{EntitiesAndComponents, Entity, TryComponentsRef};

/// Renderer is responsible for rendering the scene
pub struct Renderer {
    width: u32,
    height: u32,
    stretch: f32,
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
        }
    }

    pub fn set_stretch(&mut self, stretch: f32) {
        self.stretch = stretch;
    }

    ///  Renders the scene
    pub fn render(&mut self, scene: &EntitiesAndComponents, scene_params: &SceneParams) {
        let mut pixel_grid =
            vec![vec![scene_params.background_color; self.width as usize]; self.height as usize];

        self.render_objects(scene, &mut pixel_grid, Transform::default());
        self.render_pixel_grid(&pixel_grid, scene_params);
    }

    fn render_objects(
        &self,
        entities_and_components: &EntitiesAndComponents,
        pixel_grid: &mut Vec<Vec<Color>>,
        transform_offset: Transform,
    ) {
        // could possibly be done multithreaded and combine layers afterward
        for i in 0..entities_and_components.get_entity_count() {
            let (sprite, transform) = entities_and_components
                .try_get_components::<(Sprite, Transform)>(
                    entities_and_components.get_nth_entity(i).unwrap(), // can't fail unless done multithreaded in the future
                );
            // if the object doesn't have a sprite or transform, don't render it
            if let (Some(sprite), Some(transform)) = (sprite, transform) {
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
                }
            } else if let Some(sprite) = sprite {
                match sprite {
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
                }
            }

            // if the object has a transform and children, render the children with the transform as an offset
            if let (Some(children), Some(transform)) = (
                entities_and_components.try_get_component::<EntitiesAndComponents>(
                    entities_and_components.get_nth_entity(i).unwrap(), // again, can't fail unless done multithreaded in the future
                ),
                transform,
            ) {
                self.render_objects(&children, pixel_grid, transform + &transform_offset);
            }
            // if the object has children but no transform, render the children without any offset
            else if let Some(children) = entities_and_components
                .try_get_component::<EntitiesAndComponents>(
                    entities_and_components.get_nth_entity(i).unwrap(), // again, can't fail unless done multithreaded in the future
                )
            {
                self.render_objects(&children, pixel_grid, transform_offset);
            }
        }
    }

    pub fn render_pixel_grid(&mut self, pixel_grid: &Vec<Vec<Color>>, scene_params: &SceneParams) {
        /*crossterm::queue!(handle, cursor::Hide, cursor::MoveTo(0, 0))
            .expect("Error: failed to move cursor to 0, 0");
        crossterm::queue!(
            handle,
            crossterm::terminal::SetSize(pixel_grid.len() as u16, pixel_grid[0].len() as u16)
        )
        .expect("failed to set terminal size");*/

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
                    write!(self.handle, "{}\x08", " ").expect("failed to write white space");
                } else {
                    if scene_params.is_random_chars {
                        pixel_character +=
                            &char::from(rand::thread_rng().gen_range(33..126)).to_string();
                    } else {
                        pixel_character += &scene_params.character.to_string();
                    }

                    write!(
                        self.handle,
                        "{}\x08",
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
