use crate::{Color, Texture};
use image::GenericImageView;
use std::path::Path;

/// Load a texture from a file, stretch
pub fn load_texture(path: &str, stretch: f32) -> Texture {
    let image = image::open(&Path::new(path)).unwrap();
    let (width, height) = image.dimensions();

    let resized_image = image::imageops::resize(
        &image,
        (width as f32 * stretch) as u32,
        height,
        image::imageops::FilterType::Nearest,
    );

    let (width, height) = resized_image.dimensions();

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
            let pixel = resized_image.get_pixel(x, y);
            let color = Color {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3] as f32 / 255.0,
            };
            new_texture.pixels[y as usize][x as usize] = color;
        }
    }
    return new_texture;
}
