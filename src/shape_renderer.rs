use crate::*;
//use function::{Circle, Color, Object, Rectangle, Sprite, Transform, Update};

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
