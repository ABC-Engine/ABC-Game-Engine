use crate::{Color, Texture};
use image::GenericImageView;
use std::path::Path;

/// Load a texture from a file, stretch
pub fn load_texture(path: &str) -> Texture {
    let image = image::open(&Path::new(path)).unwrap();
    let (width, height) = image.dimensions();

    let mut new_texture = Texture {
        pixels: vec![
            vec![
                Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0.0,
                };
                width as usize
            ];
            height as usize
        ],
    };
    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y);
            let color = Color {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3] as f32 / 255.0, // assuming a 0-255 alpha channel for now -- not sure if this could be a problem
            };
            new_texture.pixels[y as usize][x as usize] = color;
        }
    }
    return new_texture;
}
