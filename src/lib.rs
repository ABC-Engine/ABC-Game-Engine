#![doc = include_str!("../README.md")]

use colored::Colorize;
use crossterm::cursor;
mod shape_renderer;
pub use shape_renderer::*;
mod load_texture;
pub use load_texture::*;
use rand::Rng;
use std::io::Write;

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

/// Object is a trait that is implemented by objects that can be rendered
pub trait Object {
    // TODO: find a way to make it so that get_sprite and get_transform can be called without having to cast to a trait object
    // So that it default accesses the transform and sprite variables of the object
    fn get_sprite(&self) -> &Sprite;
    fn get_transform(&self) -> &Transform;
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

/// Sprite is an enum that can be either a circle or a rectangle
pub enum Sprite {
    Circle(Circle),
    Rectangle(Rectangle),
    Image(Image),
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
}

/// Scene is responsible for holding all objects and the background color
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub background_color: Color,
    pub is_random_chars: bool,
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
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn add_object(&mut self, object: impl Object + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn set_random_chars(&mut self, is_random_chars: bool) {
        self.is_random_chars = is_random_chars;
    }
}

impl Renderer {
    ///  calls the update method on all objects in the scene and then renders the scene
    pub fn render(&self, scene: &mut Scene) {
        for object in &mut scene.objects {
            // if the object has the update trait
            object.update();
        }
        let mut pixel_grid =
            vec![vec![scene.background_color; self.width as usize]; self.height as usize];
        // could possible be done multithreaded and combine layers afterward
        for object in &scene.objects {
            // check if object is circle or rectangle
            match object.get_sprite() {
                Sprite::Circle(circle) => render_circle(
                    &circle,
                    &object.get_transform(),
                    &mut pixel_grid,
                    self.stretch,
                ),
                Sprite::Rectangle(rectangle) => render_rectangle(
                    &rectangle,
                    &object.get_transform(),
                    &mut pixel_grid,
                    self.stretch,
                ),
                Sprite::Image(image) => render_texture(
                    &image.texture,
                    &object.get_transform(),
                    &mut pixel_grid,
                    self.stretch,
                ),
            }
        }
        self.render_pixel_grid(pixel_grid, scene);
    }

    pub fn set_stretch(&mut self, stretch: f32) {
        self.stretch = stretch;
    }

    fn render_pixel_grid(&self, pixel_grid: Vec<Vec<Color>>, scene: &Scene) {
        let mut stdout = std::io::stdout().lock();
        write!(stdout, "{}", cursor::Hide).unwrap();
        write!(
            stdout,
            "{}",
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();
        write!(stdout, "{}", cursor::MoveTo(0, 0)).unwrap();

        let mut pixel_character = "".to_string();
        for (x, row) in pixel_grid.into_iter().enumerate() {
            for (y, pixel) in row.into_iter().enumerate() {
                // doesn't work because it doesnt delete the previous character
                //write!(stdout, "{}", cursor::MoveTo(x as u16, y as u16)).unwrap();

                if pixel.a == 0.0 {
                    write!(stdout, "{}", " ").unwrap();
                } else {
                    if scene.is_random_chars {
                        pixel_character +=
                            &char::from(rand::thread_rng().gen_range(33..126)).to_string();
                    } else {
                        pixel_character += &"=".to_string();
                    }
                    write!(
                        stdout,
                        "{}",
                        pixel_character.truecolor(pixel.r, pixel.g, pixel.b)
                    )
                    .unwrap();
                    pixel_character.clear();
                }
            }
            write!(stdout, "\n").unwrap();
        }
        stdout.flush().unwrap();
    }

    pub fn new(width: u32, height: u32) -> Renderer {
        Renderer {
            width,
            height,
            stretch: 2.3,
        }
    }
}
