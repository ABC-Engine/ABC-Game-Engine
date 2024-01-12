use crate::renderer::*;

pub fn render_pixel_grid(
    renderer: &mut Renderer,
    pixel_grid: &Vec<Vec<Color>>,
    scene_params: &SceneParams,
) {
    // if the pixel scale is greater than 1, scale the pixel grid
    // make sure that the pixel grid is not already scaled
    if renderer.renderer_params.pixel_scale != 1
        && pixel_grid.len() <= renderer.renderer_params.width as usize
    {
        let mut scaled_pixel_grid = vec![
            vec![
                Color::default();
                (renderer.renderer_params.width * renderer.renderer_params.pixel_scale as u32)
                    as usize
            ];
            (renderer.renderer_params.height * renderer.renderer_params.pixel_scale as u32)
                as usize
        ];

        for (x, row) in pixel_grid.into_iter().enumerate() {
            for (y, pixel) in row.into_iter().enumerate() {
                for i in 0..renderer.renderer_params.pixel_scale {
                    for j in 0..renderer.renderer_params.pixel_scale {
                        scaled_pixel_grid
                            [x * renderer.renderer_params.pixel_scale as usize + i as usize]
                            [y * renderer.renderer_params.pixel_scale as usize + j as usize] =
                            *pixel;
                    }
                }
            }
        }
        render_pixel_grid(renderer, &scaled_pixel_grid, scene_params);
        return;
    }

    let mut pixel_character = "".to_string();
    for (x, row) in pixel_grid.into_iter().enumerate() {
        for (y, pixel) in row.into_iter().enumerate() {
            // if the pixel is the same as the last pixel, don't render it
            if renderer.last_pixel_grid.len() != 0 && *pixel == renderer.last_pixel_grid[x][y] {
                continue;
            }
            crossterm::queue!(renderer.handle, cursor::MoveTo(y as u16, x as u16))
                .expect("Failed to move cursor");

            // \x08 is backspace
            if pixel.a == 0.0 {
                write!(renderer.handle, "\x08{}", " ").expect("failed to write white space");
            } else {
                if scene_params.is_random_chars {
                    pixel_character +=
                        &char::from(rand::thread_rng().gen_range(33..126)).to_string();
                } else {
                    pixel_character += &scene_params.character.to_string();
                }

                write!(
                    renderer.handle,
                    "\x08{}",
                    pixel_character.truecolor(pixel.r, pixel.g, pixel.b)
                )
                .expect("failed to write pixel");
                pixel_character.clear();
            }
        }
    }

    renderer.handle.flush().expect("failed to flush stdout");
    renderer.last_pixel_grid = pixel_grid.clone();
}
