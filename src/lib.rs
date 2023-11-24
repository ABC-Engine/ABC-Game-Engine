#![doc = include_str!("../README.md")]

use colored::Colorize;
use crossterm::cursor;
mod input;
mod shape_renderer;
pub use shape_renderer::*;
mod load_texture;
use core::ops::Add;
pub use crossterm::event::KeyCode;
use dioxus_debug_cell::RefCell; // better debugging, acts normal in release mode
pub use input::*;
pub use load_texture::*;
use rand::Rng;
//use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
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

// rectangle with texture
#[derive(Clone)]
pub struct Image {
    // height and width are in texture
    pub texture: Texture,
}

#[derive(Clone, Copy)]
pub struct NoSprite;

/// Object is a trait that is implemented by objects that can be rendered
pub trait Object {
    // TODO: find a way to make it so that get_sprite and get_transform can be called without having to cast to a trait object
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String) {}
    // So that it default accesses the transform and sprite variables of the object
    fn get_sprite(&self) -> &Sprite {
        &Sprite::NoSprite(NoSprite)
    }
    fn get_transform(&self) -> &Transform;
    fn get_children(&self) -> &[Rc<RefCell<Box<(dyn Object + 'static)>>>] {
        &[]
    }
    fn as_any(&self) -> &dyn std::any::Any;
    /// Update is a trait that is implemented by objects that need to be updated every frame
    fn update(&mut self) {}
}

/// Transform is a struct that holds the position, rotation, and scale of an object
#[derive(Clone, Copy)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub scale: f32,
    /// origin relative to the position of the object
    pub origin_x: f32,
    /// origin relative to the position of the object
    pub origin_y: f32,
}

impl Transform {
    pub fn default() -> Transform {
        Transform {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        }
    }
}

/// a is the parent
impl<'a, 'b> Add<&'b Transform> for &'a Transform {
    type Output = Transform;

    fn add(self, other: &'b Transform) -> Transform {
        Transform {
            x: self.x + other.x,
            y: self.y + other.y,
            rotation: self.rotation + other.rotation,
            scale: self.scale * other.scale,
            origin_x: self.origin_x - other.x as f32,
            origin_y: self.origin_y - other.y as f32,
        }
    }
}

/// Sprite is an enum that can be either a circle or a rectangle
#[derive(Clone)]
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

// Define a type for the tasks to be stored in the queue
pub type Task = Box<dyn FnOnce() -> ()>;

// Define a struct for the task queue
struct TaskQueue {
    task_queue: Vec<Task>,
}

impl TaskQueue {
    // Create a new empty task queue
    fn new() -> Self {
        TaskQueue {
            task_queue: Vec::new(),
        }
    }

    // Add a task to the queue
    fn enqueue(&mut self, task: Task) {
        self.task_queue.push(task);
    }

    // Execute all tasks in the queue
    fn execute_all(&mut self) {
        while let Some(task) = self.task_queue.pop() {
            task();
        }
    }
}

pub struct Scene {
    pub inner_scene: Rc<RefCell<InnerScene>>,
}

/// Scene is responsible for holding all objects and the background color
pub struct InnerScene {
    pub objects: HashMap<String, Rc<RefCell<Box<dyn Object>>>>,
    background_color: Color,
    is_random_chars: bool,
    character: char,
    task_queue: TaskQueue,
}

impl Scene {
    pub fn new() -> Scene {
        let inner_scene = InnerScene {
            objects: HashMap::new(),
            background_color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 1.0,
            },
            is_random_chars: false,
            character: '=',
            task_queue: TaskQueue::new(),
        };
        Scene {
            inner_scene: Rc::new(RefCell::new(inner_scene)),
        }
    }

    pub fn queue(&mut self, task: Task) {
        self.inner_scene.borrow_mut().task_queue.enqueue(task);
    }

    /// if alpha is 0, then the background is spaces
    pub fn set_background_color(&mut self, color: Color) {
        self.inner_scene.borrow_mut().background_color = color;
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
        self.inner_scene
            .borrow_mut()
            .objects
            .insert(name.clone(), Rc::new(RefCell::new(Box::new(object))));
        name
    }

    /// makes the characters that are rendered random
    pub fn set_random_chars(&mut self, is_random_chars: bool) {
        self.inner_scene.borrow_mut().is_random_chars = is_random_chars;
    }

    /// sets the character that will be rendered for each pixel --only works if is_random_chars is false
    pub fn set_character(&mut self, character: char) {
        self.inner_scene.borrow_mut().character = character;
    }

    /// returns an option with Rc<RefCell<Box<dyn Object>>> if the object is found in the scene
    /// use sparingly, this is an O(n) operation
    pub fn find_object(&self, object: &String) -> Option<Rc<RefCell<Box<(dyn Object + 'static)>>>> {
        self.inner_scene.borrow().objects.get(object).cloned()
    }

    pub fn remove_object(&mut self, object: &String) -> bool {
        self.inner_scene
            .borrow_mut()
            .objects
            .remove(object)
            .is_some()
    }

    /// called by render automatically, use this for debugging
    pub fn update_objects(&self) {
        let objects = self.inner_scene.borrow().objects.clone();
        self.inner_scene.borrow_mut().task_queue.execute_all();
        for object in objects {
            let mut borrowed_object = object.1.borrow_mut();
            borrowed_object.update();
            if borrowed_object.get_children().len() > 0 {
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
    pub fn render(&self, scene: Rc<RefCell<Scene>>) {
        // update objects
        {
            scene.borrow().update_objects();
        }

        {
            let borrowed_scene = scene.borrow();
            let borrowed_inner_scene = borrowed_scene.inner_scene.borrow();

            let mut pixel_grid =
                vec![
                    vec![borrowed_inner_scene.background_color; self.width as usize];
                    self.height as usize
                ];

            self.render_objects(
                // not sure how slow this is
                &borrowed_inner_scene
                    .objects
                    .values()
                    .cloned()
                    .collect::<Vec<_>>(),
                &mut pixel_grid,
                Transform::default(),
            );
            self.render_pixel_grid(&pixel_grid, &borrowed_inner_scene);
        }
    }

    fn render_objects(
        &self,
        objects: &[Rc<RefCell<Box<(dyn Object + 'static)>>>],
        pixel_grid: &mut Vec<Vec<Color>>,
        transform_offset: Transform,
    ) {
        // could possible be done multithreaded and combine layers afterward
        for object in objects {
            let borrowed_object = object.borrow_mut();
            // check if object is circle or rectangle
            match borrowed_object.get_sprite() {
                Sprite::Circle(circle) => render_circle(
                    &circle,
                    &(borrowed_object.get_transform() + &transform_offset),
                    pixel_grid,
                    self.stretch,
                ),
                Sprite::Rectangle(rectangle) => render_rectangle(
                    &rectangle,
                    &(borrowed_object.get_transform() + &transform_offset),
                    pixel_grid,
                    self.stretch,
                ),
                Sprite::Image(image) => render_texture(
                    &image.texture,
                    &(borrowed_object.get_transform() + &transform_offset),
                    pixel_grid,
                    self.stretch,
                ),
                Sprite::NoSprite(NoSprite) => {}
            }
            if borrowed_object.get_children().len() > 0 {
                self.render_objects(
                    borrowed_object.get_children(),
                    pixel_grid,
                    borrowed_object.get_transform() + &transform_offset,
                );
            }
        }
    }

    pub fn render_pixel_grid(&self, pixel_grid: &Vec<Vec<Color>>, scene: &InnerScene) {
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
