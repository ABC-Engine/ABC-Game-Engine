use crate::*;
// should the shape structs be moved to this file?

pub fn render_circle(
    circle: &Circle,
    transform: &Transform,
    pixel_grid: &mut Vec<Vec<Color>>,
    stretch: f32,
) {
    if circle.color.a == 0.0 {
        return;
    }

    let squared_radius = circle.radius.powi(2);
    for x in 0..pixel_grid[0].len() {
        for y in 0..pixel_grid.len() {
            let adjusted_x = (x as f32 / stretch) / transform.scale;
            let adjusted_y = y as f32 / transform.scale;

            let dx = adjusted_x as f64 - transform.x;
            let dy = adjusted_y as f64 - transform.y;

            let distance_squared = dx.powi(2) + dy.powi(2);
            if distance_squared <= squared_radius {
                let pixel = &mut pixel_grid[y][x];

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
    stretch: f32,
) {
    if rectangle.color.a == 0.0 {
        return;
    }
    for x in 0..pixel_grid[0].len() {
        for y in 0..pixel_grid.len() {
            let mut adjusted_x = (x as f32 / stretch) / transform.scale;
            let mut adjusted_y = y as f32 / transform.scale;

            if transform.rotation != 0.0 {
                (adjusted_x, adjusted_y) = rotate_point_around(
                    adjusted_x,
                    adjusted_y,
                    transform.x as f32,
                    transform.y as f32,
                    transform.rotation,
                );
            }

            if adjusted_x as f64 >= transform.x - rectangle.width / 2.0
                && adjusted_x as f64 <= transform.x + rectangle.width / 2.0
                && adjusted_y as f64 >= transform.y - rectangle.height / 2.0
                && adjusted_y as f64 <= transform.y + rectangle.height / 2.0
            {
                let pixel = &mut pixel_grid[y][x];

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
    stretch: f32,
) {
    let (texture_width, texture_height) = (texture.pixels[0].len(), texture.pixels.len());
    for x in 0..pixel_grid[0].len() {
        for y in 0..pixel_grid.len() {
            let mut adjusted_x = x as f32 / transform.scale;
            let mut adjusted_y = (y as f32 * stretch) / transform.scale;

            if transform.rotation != 0.0 {
                (adjusted_x, adjusted_y) = rotate_point_around(
                    adjusted_x,
                    adjusted_y,
                    transform.x as f32,
                    transform.y as f32,
                    transform.rotation,
                );
            }

            //positions relative to the center of the texture where the origin is centered
            let relative_x = adjusted_x as f64 - transform.x;
            let relative_y = adjusted_y as f64 - transform.y;

            if relative_x >= -(texture_width as f64 / 2.0)
                && relative_x <= texture_width as f64 / 2.0
                && relative_y >= -(texture_height as f64 / 2.0)
                && relative_y <= texture_height as f64 / 2.0
            {
                let out_pixel = &mut pixel_grid[y][x];

                let texture_y_coord = ((relative_y + texture_height as f64 / 2.0) as usize);
                let texture_x_coord = ((relative_x + texture_width as f64 / 2.0) as usize);

                if (texture_x_coord >= texture_height || texture_y_coord >= texture_width) {
                    continue;
                }

                let texture_pixel = &texture.pixels[texture_y_coord][texture_x_coord];

                if texture_pixel.a == 1.0 {
                    *out_pixel = *texture_pixel;
                } else {
                    *out_pixel = overlay_colors(out_pixel, texture_pixel);
                }
            }
        }
    }
}

fn overlay_colors(back_color: &Color, front_color: &Color) -> Color {
    // there has to be a better way to do this but I'm not sure how...
    let mut return_color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    };
    return_color.r = (back_color.r as f32 * (1.0 - front_color.a) as f32) as u8;
    return_color.r += (front_color.r as f32 * front_color.a) as u8;
    return_color.g = (back_color.g as f32 * (1.0 - front_color.a) as f32) as u8;
    return_color.g += (front_color.g as f32 * front_color.a) as u8;
    return_color.b = (back_color.b as f32 * (1.0 - front_color.a) as f32) as u8;
    return_color.b += (front_color.b as f32 * front_color.a) as u8;
    return_color.a = back_color.a + front_color.a * (1.0 - back_color.a);
    return_color
}

fn rotate_point_around(x: f32, y: f32, cx: f32, cy: f32, angle_degrees: f64) -> (f32, f32) {
    // Convert angle from degrees to radians
    let angle_radians = angle_degrees.to_radians();

    // Translate the point to the origin
    let translated_x = x - cx;
    let translated_y = y - cy;

    // Perform the rotation using trigonometric functions
    let rotated_x =
        translated_x * angle_radians.cos() as f32 - translated_y * angle_radians.sin() as f32;
    let rotated_y =
        translated_x * angle_radians.sin() as f32 + translated_y * angle_radians.cos() as f32;

    // Translate the point back to its original position
    let final_x = rotated_x + cx;
    let final_y = rotated_y + cy;

    (final_x, final_y)
}
