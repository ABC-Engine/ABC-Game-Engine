use crate::renderer::*;
use pixel_canvas::Canvas;

/// This is just a starting point for the canvas renderer. It doesn't work yet.
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

    let (width, height) = (pixel_grid.len(), pixel_grid[0].len());

    let pixel_grid = pixel_grid.clone();

    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
    // provided.
    let canvas = Canvas::new(width, height).title("Pixel Canvas");
    // The canvas will render for you at up to 60fps.
    canvas.render(move |_mouse, image| {
        // Modify the `image` based on your state.
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                // flip the y axis
                let y = height - y - 1;
                let pixel_grid_color = pixel_grid[y][x].clone();
                *pixel = pixel_canvas::Color {
                    r: pixel_grid_color.r,
                    g: pixel_grid_color.g,
                    b: pixel_grid_color.b,
                }
            }
        }
    });
}
