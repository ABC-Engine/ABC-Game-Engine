use crate::*;
// should the shape structs be moved to this file?

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
                    *pixel = overlay_colors(&pixel, &circle.color);
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
                    *pixel = overlay_colors(&pixel, &rectangle.color);
                }
            }
        }
    }
}

pub struct Texture {
    pub pixels: Vec<Vec<Color>>, // not sure how inefficient this is but it will do for now
}

pub fn render_texture(
    texture: &Texture,
    transform: &Transform,
    pixel_grid: &mut Vec<Vec<Color>>,
    stretch: &f32,
) {
    let (texture_width, texture_height) = (texture.pixels[0].len(), texture.pixels.len());
    for x in 0..pixel_grid[0].len() {
        for y in 0..pixel_grid.len() {
            let out_pixel = &mut pixel_grid[y][x];
            //let adjusted_x = x as f32 / stretch;
            if x as f64 >= transform.x
                && x as f64 <= transform.x + texture_width as f64
                && y as f64 >= transform.y
                && y as f64 <= transform.y + texture_height as f64
            {
                let texture_pixel = &texture.pixels
                    [((y as f32 - transform.y as f32) as usize).min(texture_height - 1)]
                    [((x as f32 - transform.x as f32) as usize).min(texture_width - 1)];
                if texture_pixel.a == 1.0 {
                    *out_pixel = *texture_pixel;
                } else {
                    *out_pixel = overlay_colors(out_pixel, texture_pixel);
                }
            }
        }
    }
}

fn overlay_colors(color1: &Color, color2: &Color) -> Color {
    // there has to be a better way to do this but I'm not sure how...
    let mut return_color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    };
    return_color.r = (color1.r as f32 * (1.0 - color2.a) as f32) as u8;
    return_color.r += (color2.r as f32 * color2.a) as u8;
    return_color.g = (color1.g as f32 * (1.0 - color2.a) as f32) as u8;
    return_color.g += (color2.g as f32 * color2.a) as u8;
    return_color.b = (color1.b as f32 * (1.0 - color2.a) as f32) as u8;
    return_color.b += (color2.b as f32 * color2.a) as u8;
    return_color.a = 1.0;
    return_color
}
