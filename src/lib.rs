#![doc = include_str!("../README.md")]

use colored::Colorize;
use crossterm::cursor;
mod input;
mod shape_renderer;
pub use shape_renderer::*;
mod load_texture;
pub use crossterm::event::KeyCode;
use dioxus_debug_cell::RefCell; // better debugging, acts normal in release mode
pub use input::*;
pub use load_texture::*;
use rand::Rng;
mod renderer;
pub use renderer::*;
use std::rc::Rc;
use std::{
    clone,
    io::{stderr, Write},
};
pub use ABC_ECS::{Component, EntitiesAndComponents, GameEngine, System};

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

impl Component for Transform {}

/// a is the parent
impl<'a, 'b> std::ops::Add<&'b Transform> for &'a Transform {
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
}

impl Component for Sprite {}

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

/// SceneParams is a struct that holds the background color, if the characters are random, and the character that will be displayed otherwise
pub struct SceneParams {
    background_color: Color,
    is_random_chars: bool,
    character: char,
}

impl SceneParams {
    pub fn new() -> SceneParams {
        SceneParams {
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

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_random_chars(&mut self, is_random_chars: bool) {
        self.is_random_chars = is_random_chars;
    }

    pub fn set_character(&mut self, character: char) {
        self.character = character;
    }
}

/// Scene is responsible for holding all objects and the background color
pub struct Scene {
    pub game_engine: GameEngine,
    pub scene_params: SceneParams,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            game_engine: GameEngine::new(),
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
        }
    }
}

/*pub struct InnerScene {
    pub objects: HashMap<String, Rc<RefCell<Box<dyn Object>>>>,
    background_color: Color,
    is_random_chars: bool,
    character: char,
    task_queue: TaskQueue,
}*/

/*impl Scene {
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
}*/
