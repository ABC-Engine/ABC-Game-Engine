use crate::{renderer::Animation, renderer::Image, renderer::Texture, Color};
use image::GenericImageView;
use std::{
    path::Path,
    string,
    time::{Duration, Instant},
};

/// Load a texture from a file, stretch
pub fn load_texture(path: &str) -> Texture {
    let image = image::open(&Path::new(path)).expect("Error: failed to open image");
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

/// Loads a spritesheet from a file
/// The spritesheet must be a grid of cells
/// Returns a vector of animations, one for each row of cells
pub fn load_spritesheet(
    number_of_cells_horizontal: u32,
    number_of_cells_vertical: u32,
    frame_duration_ms: u64,
    path: &str,
) -> Vec<Animation> {
    let mut spritesheets = vec![];
    let texture = load_texture(path);
    // split the texture into frames
    let (texture_width, texture_height) = (texture.pixels[0].len(), texture.pixels.len());
    let cell_width = texture_width as u32 / number_of_cells_horizontal;
    let cell_height = texture_height as u32 / number_of_cells_vertical;

    for vertical_cell_index in 0..number_of_cells_vertical {
        let mut spritesheet = Animation {
            frames: vec![],
            current_frame: 0,
            frame_time: Duration::from_millis(frame_duration_ms),
            current_frame_start_time: Instant::now(),
            loop_animation: true,
            finished: false,
        };

        for horizonal_cell_index in 0..number_of_cells_horizontal {
            let mut new_texture = Texture {
                pixels: vec![
                    vec![
                        Color {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: 0.0,
                        };
                        cell_width as usize
                    ];
                    cell_height as usize
                ],
            };
            for x in 0..cell_width {
                for y in 0..cell_height {
                    let pixel = texture.pixels[(y + (vertical_cell_index * cell_height)) as usize]
                        [(x + (horizonal_cell_index * cell_width)) as usize];
                    new_texture.pixels[y as usize][x as usize] = pixel;
                }
            }
            spritesheet.frames.push(Image {
                texture: new_texture,
            });
        }
        spritesheets.push(spritesheet);
    }

    return spritesheets;
}
