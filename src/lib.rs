use colored::Colorize;

pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
    color: [u8; 4],
}

pub struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: [u8; 4],
}

pub enum Object {
    Circle(Circle),
    Rectangle(Rectangle),
}

pub struct Renderer {
    width: u32,
    height: u32,
    objects: Vec<Object>,
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
    fn render(&self) {
        let mut pixel_grid = vec![vec![[0, 0, 0, 0]; self.width as usize]; self.height as usize];
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

    fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    fn set_stretch(&mut self, stretch: f32) {
        self.stretch = stretch;
    }

    fn render_pixel_grid(&self, pixel_grid: Vec<Vec<[u8; 4]>>) {
        for row in pixel_grid {
            for pixel in row {
                print!("{}", "=".truecolor(pixel[0], pixel[1], pixel[2]));
            }
            println!();
        }
    }
}

pub fn render_circle(circle: &Circle, pixel_grid: &mut Vec<Vec<[u8; 4]>>, stretch: &f32) {
    let squared_radius = circle.radius.powi(2);
    for x in 0..pixel_grid[0].len() {
        for y in 0..pixel_grid.len() {
            let pixel = &mut pixel_grid[y][x];
            let adjusted_x = x as f32 / stretch;
            let dx = adjusted_x as f64 - circle.x;
            let dy = y as f64 - circle.y;
            let distance_squared = dx.powi(2) + dy.powi(2);
            if distance_squared <= squared_radius {
                *pixel = circle.color;
            }
        }
    }
}

pub fn render_rectangle(rectangle: &Rectangle, pixel_grid: &mut Vec<Vec<[u8; 4]>>, stretch: &f32) {
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

//test
#[test]
fn test_render() {
    let mut renderer = new_renderer(80, 40);
    renderer.add_object(Object::Circle(Circle {
        x: 10.0,
        y: 20.0,
        radius: 5.0,
        color: [255, 0, 0, 255],
    }));
    renderer.add_object(Object::Rectangle(Rectangle {
        x: 10.0,
        y: 15.0,
        width: 7.0,
        height: 7.0,
        color: [0, 255, 0, 255],
    }));
    renderer.render();
}
