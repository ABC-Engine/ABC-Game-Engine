use colored::Colorize;
use std::time::Instant;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
}

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

pub enum Object {
    Circle(Circle),
    Rectangle(Rectangle),
}

pub struct Renderer {
    width: u32,
    height: u32,
    pub objects: Vec<Object>,
    stretch: f32,
}

pub fn new_renderer(width: u32, height: u32) -> Renderer {
    Renderer {
        width,
        height,
        objects: Vec::new(),
        stretch: 2.3,
    }
}

impl Renderer {
    pub fn render(&self) {
        let mut pixel_grid = vec![
            vec![
                Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0.0
                };
                self.width as usize
            ];
            self.height as usize
        ];
        for object in &self.objects {
            // check if object is circle or rectangle
            match object {
                Object::Circle(circle) => render_circle(circle, &mut pixel_grid, &self.stretch),
                Object::Rectangle(rectangle) => {
                    render_rectangle(rectangle, &mut pixel_grid, &self.stretch)
                }
            }
        }
        self.render_pixel_grid(pixel_grid);
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn set_stretch(&mut self, stretch: f32) {
        self.stretch = stretch;
    }

    fn render_pixel_grid(&self, pixel_grid: Vec<Vec<Color>>) {
        print!("\x1B[2J\x1B[1;1H"); // clear terminal and set cursor to 1,1 (supposedly, does not seem to be working on windows)
        let mut screen_string = String::new();
        for row in pixel_grid {
            for pixel in row {
                screen_string.push_str(&format!("{}", "=".truecolor(pixel.r, pixel.g, pixel.b)));
            }
            screen_string.push_str("\n");
        }
        print!("{}", screen_string);
    }
}

pub fn render_circle(circle: &Circle, pixel_grid: &mut Vec<Vec<Color>>, stretch: &f32) {
    if circle.color.a == 0.0 {
        return;
    }

    let squared_radius = circle.radius.powi(2);
    for x in 0..pixel_grid[0].len() {
        for y in 0..pixel_grid.len() {
            let pixel = &mut pixel_grid[y][x];
            let adjusted_x = x as f32 / stretch;
            let dx = adjusted_x as f64 - circle.x;
            let dy = y as f64 - circle.y;
            let distance_squared = dx.powi(2) + dy.powi(2);
            if distance_squared <= squared_radius {
                //*pixel = circle.color;
                // alpha
                if circle.color.a == 1.0 {
                    *pixel = circle.color; // easier and should capture division by 0
                } else {
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

pub fn render_rectangle(rectangle: &Rectangle, pixel_grid: &mut Vec<Vec<Color>>, stretch: &f32) {
    for x in 0..pixel_grid[0].len() {
        for y in 0..pixel_grid.len() {
            let pixel = &mut pixel_grid[y][x];
            let adjusted_x = x as f32 / stretch;
            if adjusted_x as f64 >= rectangle.x
                && adjusted_x as f64 <= rectangle.x + rectangle.width
                && y as f64 >= rectangle.y
                && y as f64 <= rectangle.y + rectangle.height
            {
                *pixel = rectangle.color;
            }
        }
    }
}
