use colored::Colorize;

#[derive(Clone, Copy)]
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

/// Update is a trait that is implemented by objects that need to be updated every frame
pub trait Update {
    fn update(&mut self) {}
}

/// Object is a trait that is implemented by objects that can be rendered
pub trait Object: Update {
    fn get_sprite(&self) -> &Sprite;
    fn get_transform(&self) -> &Transform;
}

/// Transform is a struct that holds the position, rotation, and scale of an object
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub scale: f64,
}

/// Sprite is an enum that can be either a circle or a rectangle
pub enum Sprite {
    Circle(Circle),
    Rectangle(Rectangle),
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
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn add_object(&mut self, object: impl Object + 'static) {
        self.objects.push(Box::new(object));
    }
}

pub fn new_renderer(width: u32, height: u32) -> Renderer {
    Renderer {
        width,
        height,
        stretch: 2.3,
    }
}

impl Renderer {
    ///  calls the update method on all objects in the scene and then renders the scene
    pub fn render(&self, scene: &mut Scene) {
        for object in &mut scene.objects {
            object.update();
        }
        let mut pixel_grid =
            vec![vec![scene.background_color; self.width as usize]; self.height as usize];
        for object in &scene.objects {
            // check if object is circle or rectangle
            match object.get_sprite() {
                Sprite::Circle(circle) => render_circle(
                    &circle,
                    &object.get_transform(),
                    &mut pixel_grid,
                    &self.stretch,
                ),
                Sprite::Rectangle(rectangle) => render_rectangle(
                    &rectangle,
                    &object.get_transform(),
                    &mut pixel_grid,
                    &self.stretch,
                ),
            }
        }
        self.render_pixel_grid(pixel_grid);
    }

    pub fn set_stretch(&mut self, stretch: f32) {
        self.stretch = stretch;
    }

    fn render_pixel_grid(&self, pixel_grid: Vec<Vec<Color>>) {
        print!("\x1B[2J\x1B[1;1H"); // clear terminal and set cursor to 1,1 (supposedly, does not seem to be working on windows)
        let mut screen_string = String::new();
        for row in pixel_grid {
            for pixel in row {
                if pixel.a == 0.0 {
                    screen_string.push_str(&format!("{}", " "));
                } else {
                    screen_string
                        .push_str(&format!("{}", "=".truecolor(pixel.r, pixel.g, pixel.b)));
                }
            }
            screen_string.push_str("\n");
        }
        print!("{}", screen_string);
    }
}

pub fn render_circle(
    circle: &Circle,
    transform: &Transform,
    pixel_grid: &mut Vec<Vec<Color>>,
    stretch: &f32,
) {
    if circle.color.a == 0.0 {
        return;
    }

    let squared_radius = circle.radius.powi(2);
    for x in 0..pixel_grid[0].len() {
        for y in 0..pixel_grid.len() {
            let pixel = &mut pixel_grid[y][x];
            let adjusted_x = x as f32 / stretch;
            let dx = adjusted_x as f64 - transform.x;
            let dy = y as f64 - transform.y;
            let distance_squared = dx.powi(2) + dy.powi(2);
            if distance_squared <= squared_radius {
                if circle.color.a == 1.0 {
                    *pixel = circle.color;
                } else {
                    // there has to be a better way to do this but I'm not sure how...
                    pixel.r = (pixel.r as f32 * (1.0 - circle.color.a) as f32) as u8;
                    pixel.r += (circle.color.r as f32 * circle.color.a) as u8;
                    pixel.g = (pixel.g as f32 * (1.0 - circle.color.a) as f32) as u8;
                    pixel.g += (circle.color.g as f32 * circle.color.a) as u8;
                    pixel.b = (pixel.b as f32 * (1.0 - circle.color.a) as f32) as u8;
                    pixel.b += (circle.color.b as f32 * circle.color.a) as u8;
                    pixel.a = 1.0;
                }
            }
        }
    }
}

pub fn render_rectangle(
    rectangle: &Rectangle,
    transform: &Transform,
    pixel_grid: &mut Vec<Vec<Color>>,
    stretch: &f32,
) {
    if rectangle.color.a == 0.0 {
        return;
    }
    for x in 0..pixel_grid[0].len() {
        for y in 0..pixel_grid.len() {
            let pixel = &mut pixel_grid[y][x];
            let adjusted_x = x as f32 / stretch;
            if adjusted_x as f64 >= transform.x
                && adjusted_x as f64 <= transform.x + rectangle.width
                && y as f64 >= transform.y
                && y as f64 <= transform.y + rectangle.height
            {
                if rectangle.color.a == 1.0 {
                    *pixel = rectangle.color;
                } else {
                    pixel.r = (pixel.r as f32 * (1.0 - rectangle.color.a) as f32) as u8;
                    pixel.r += (rectangle.color.r as f32 * rectangle.color.a) as u8;
                    pixel.g = (pixel.g as f32 * (1.0 - rectangle.color.a) as f32) as u8;
                    pixel.g += (rectangle.color.g as f32 * rectangle.color.a) as u8;
                    pixel.b = (pixel.b as f32 * (1.0 - rectangle.color.a) as f32) as u8;
                    pixel.b += (rectangle.color.b as f32 * rectangle.color.a) as u8;
                    pixel.a = 1.0;
                }
            }
        }
    }
}
