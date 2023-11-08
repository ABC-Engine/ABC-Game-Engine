use colored::Colorize;

pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
    color: [u8; 3],
}

pub struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: [u8; 3],
}

pub struct Renderer {
    width: u32,
    height: u32,
    circles: Vec<Circle>,
    rectangles: Vec<Rectangle>,
    stretch: f32,
}

pub fn new_renderer(width: u32, height: u32) -> Renderer {
    Renderer {
        width,
        height,
        circles: Vec::new(),
        rectangles: Vec::new(),
        stretch: 2.3,
    }
}

impl Renderer {
    fn render(&self) {
        let mut pixel_grid = vec![vec![[0, 0, 0]; self.width as usize]; self.height as usize];
        for i in 0..self.circles.len() {
            render_circle(&self.circles[i], &mut pixel_grid, &self.stretch);
        }
        for i in 0..self.rectangles.len() {
            render_rectangle(&self.rectangles[i], &mut pixel_grid, &self.stretch);
        }
        render_pixel_grid(pixel_grid);
    }

    fn add_circle(&mut self, circle: Circle) {
        self.circles.push(circle);
    }

    fn add_rectangle(&mut self, rectangle: Rectangle) {
        self.rectangles.push(rectangle);
    }
}

pub fn render_circle(circle: &Circle, pixel_grid: &mut Vec<Vec<[u8; 3]>>, stretch: &f32) {
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

pub fn render_rectangle(rectangle: &Rectangle, pixel_grid: &mut Vec<Vec<[u8; 3]>>, stretch: &f32) {
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

pub fn render_pixel_grid(pixel_grid: Vec<Vec<[u8; 3]>>) {
    for row in pixel_grid {
        for pixel in row {
            print!("{}", "=".truecolor(pixel[0], pixel[1], pixel[2]));
        }
        println!();
    }
}

//test
#[test]
fn test_render() {
    let mut renderer = new_renderer(240, 160);
    renderer.add_circle(Circle {
        x: 40.0,
        y: 70.0,
        radius: 20.0,
        color: [255, 0, 0],
    });
    renderer.add_rectangle(Rectangle {
        x: 20.0,
        y: 20.0,
        width: 20.0,
        height: 20.0,
        color: [0, 255, 0],
    });
    renderer.render();
}
