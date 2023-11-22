#![doc = include_str!("../README.md")]

use colored::Colorize;
use crossterm::cursor;
mod input;
mod shape_renderer;
pub use shape_renderer::*;
mod load_texture;
use core::ops::Add;
pub use crossterm::event::KeyCode;
pub use input::*;
pub use load_texture::*;
use rand::Rng;
use std::{
    clone,
    io::{stderr, Write},
};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

pub struct Circle {
    pub radius: f64,
    pub color: Color,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

// rectangle with texture
pub struct Image {
    // height and width are in texture
    pub texture: Texture,
}

pub struct NoSprite;

/// Object is a trait that is implemented by objects that can be rendered
// clone
pub trait Object {
    // TODO: find a way to make it so that get_sprite and get_transform can be called without having to cast to a trait object
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String) {}
    // So that it default accesses the transform and sprite variables of the object
    fn get_sprite(&self) -> &Sprite {
        &Sprite::NoSprite(NoSprite)
    }
    fn get_transform(&self) -> &Transform;
    fn get_children(&self) -> &[Box<dyn Object>] {
        &[]
    }
    fn as_any(&self) -> &dyn std::any::Any;
    /// Update is a trait that is implemented by objects that need to be updated every frame
    fn update(&mut self) {}
}

/// Transform is a struct that holds the position, rotation, and scale of an object
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub scale: f32,
}

impl Transform {
    pub fn default() -> Transform {
        Transform {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale: 1.0,
        }
    }
}

impl<'a, 'b> Add<&'b Transform> for &'a Transform {
    type Output = Transform;

    fn add(self, other: &'b Transform) -> Transform {
        Transform {
            x: self.x + other.x,
            y: self.y + other.y,
            rotation: self.rotation + other.rotation,
            scale: self.scale * other.scale,
        }
    }
}

/// Sprite is an enum that can be either a circle or a rectangle
pub enum Sprite {
    Circle(Circle),
    Rectangle(Rectangle),
    Image(Image),
    NoSprite(NoSprite),
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

impl From<NoSprite> for Sprite {
    fn from(no_sprite: NoSprite) -> Self {
        Sprite::NoSprite(no_sprite)
    }
}

/// Scene is responsible for holding all objects and the background color
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub background_color: Color,
    pub is_random_chars: bool,
    pub character: char,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            background_color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 1.0,
            },
            is_random_chars: false,
            character: '=',
        }
    }

    /// if alpha is 0, then the background is spaces
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// adds an object on top of the other objects returns the name of the object
    pub fn add_object(&mut self, mut object: impl Object + 'static) -> String {
        if self.find_object(object.get_name()).is_some() {
            let mut clone_number = 0;
            while self
                .find_object(&format!("{}({})", object.get_name(), clone_number))
                .is_some()
            {
                clone_number += 1;
            }

            let new_name = format!("{}({})", object.get_name(), clone_number);
            object.set_name(new_name.clone());
            if *object.get_name() != new_name {
                stderr()
                    .write(
                        format!("Error: if object is clonable set_name must be implemented",)
                            .as_bytes(),
                    )
                    .expect("failed to display error if this happens then idk what to tell you");
                panic!("Object name was already taken")
            }
        }

        let name = object.get_name().to_string();
        self.objects.push(Box::new(object));
        name
    }

    /// makes the characters that are rendered random
    pub fn set_random_chars(&mut self, is_random_chars: bool) {
        self.is_random_chars = is_random_chars;
    }

    /// sets the character that will be rendered for each pixel --only works if is_random_chars is false
    pub fn set_character(&mut self, character: char) {
        self.character = character;
    }

    /// use sparingly, this is an O(n) operation
    pub fn find_object(&mut self, object: &String) -> Option<&mut dyn Object> {
        for (index, object_in_scene) in self.objects.iter().enumerate() {
            if *object_in_scene.get_name() == *object {
                return Some(&mut *self.objects[index]);
            }
        }
        None
    }

    pub fn remove_object(&mut self, object: &String) -> bool {
        for (index, object_in_scene) in self.objects.iter().enumerate() {
            if *object_in_scene.get_name() == *object {
                self.objects.remove(index);
                return true;
            }
        }
        false
    }

    pub fn update_objects(&mut self) {
        for object in &mut self.objects {
            object.update();
            if object.get_children().len() > 0 {
                // TODO
            }
        }
    }
}

/// Renderer is responsible for rendering the scene
pub struct Renderer {
    width: u32,
    height: u32,
    stretch: f32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Renderer {
        let mut stdout = std::io::stdout().lock();
        crossterm::queue!(
            stdout,
            cursor::Hide,
            crossterm::terminal::SetSize(width as u16 * 2, height as u16 * 2),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        )
        .expect("Error: failed to set terminal size");

        Renderer {
            width,
            height,
            stretch: 2.3,
        }
    }

    pub fn set_stretch(&mut self, stretch: f32) {
        self.stretch = stretch;
    }

    ///  calls the update method on all objects in the scene and then renders the scene
    pub fn render(&self, scene: &mut Scene) {
        scene.update_objects();

        let mut pixel_grid =
            vec![vec![scene.background_color; self.width as usize]; self.height as usize];

        self.render_objects(&scene.objects, &mut pixel_grid, Transform::default());
        self.render_pixel_grid(&pixel_grid, scene);
    }

    fn render_objects(
        &self,
        objects: &[Box<dyn Object>],
        pixel_grid: &mut Vec<Vec<Color>>,
        transform_offset: Transform,
    ) {
        // could possible be done multithreaded and combine layers afterward
        for object in objects {
            // check if object is circle or rectangle
            match object.get_sprite() {
                Sprite::Circle(circle) => render_circle(
                    &circle,
                    &(object.get_transform() + &transform_offset),
                    pixel_grid,
                    self.stretch,
                ),
                Sprite::Rectangle(rectangle) => render_rectangle(
                    &rectangle,
                    &(object.get_transform() + &transform_offset),
                    pixel_grid,
                    self.stretch,
                ),
                Sprite::Image(image) => render_texture(
                    &image.texture,
                    &(object.get_transform() + &transform_offset),
                    pixel_grid,
                    self.stretch,
                ),
                Sprite::NoSprite(NoSprite) => {}
            }
            if object.get_children().len() > 0 {
                self.render_objects(
                    object.get_children(),
                    pixel_grid,
                    object.get_transform() + &transform_offset,
                );
            }
        }
    }

    pub fn render_pixel_grid(&self, pixel_grid: &Vec<Vec<Color>>, scene: &Scene) {
        let mut stdout = std::io::stdout().lock();
        crossterm::queue!(stdout, cursor::Hide, cursor::MoveTo(0, 0))
            .expect("Error: failed to move cursor to 0, 0");

        let mut pixel_character = "".to_string();
        for (x, row) in pixel_grid.into_iter().enumerate() {
            for (y, pixel) in row.into_iter().enumerate() {
                crossterm::queue!(stdout, cursor::MoveTo(y as u16, x as u16),)
                    .expect("Failed to move cursor");
                // \x08 is backspace
                if pixel.a == 0.0 {
                    write!(stdout, "{}\x08", " ").expect("failed to write white space");
                } else {
                    if scene.is_random_chars {
                        pixel_character +=
                            &char::from(rand::thread_rng().gen_range(33..126)).to_string();
                    } else {
                        pixel_character += &scene.character.to_string();
                    }

                    write!(
                        stdout,
                        "{}\x08",
                        pixel_character.truecolor(pixel.r, pixel.g, pixel.b)
                    )
                    .expect("failed to write pixel");
                    pixel_character.clear();
                }
            }
        }
        stdout.flush().expect("failed to flush stdout");
    }
}
