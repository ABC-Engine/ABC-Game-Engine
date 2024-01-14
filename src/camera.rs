use crate::*;

/// A simple camera that can be used to move the view around
#[derive(Clone)]
pub struct Camera {
    pub(crate) width: u32,
    pub(crate) height: u32,
    // if false, the camera will not be used by the renderer
    // if multiple cameras are active, the program will panic
    pub(crate) is_active: bool,
}

impl Camera {
    /// creates a new camera
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            is_active: true,
        }
    }

    /// creates a new camera with the size (160, 160)
    pub fn default() -> Self {
        Self {
            width: 160,
            height: 160,
            is_active: true,
        }
    }

    /// sets the size of the camera
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }
}

/// returns true if the given rectangle is in view of the camera
/// used for culling by the renderer
pub fn object_is_in_view_of_camera(
    camera: &Camera,
    camera_transform: Transform,
    object_transform: &Transform,
    object_sprite: &Sprite,
) -> bool {
    match object_sprite {
        Sprite::Rectangle(Rectangle { width, height, .. }) => {
            let max_dist = (height.powi(2) + width.powi(2)).sqrt(); // this is the maximum width that a square would need to be to cover the rectangle at any rotation
            square_is_in_view_of_camera(
                camera,
                camera_transform,
                object_transform,
                max_dist as f64,
                max_dist as f64,
            )
        }
        Sprite::Circle(Circle { radius, .. }) => {
            let diameter = *radius as f64 * 2.0;
            square_is_in_view_of_camera(
                camera,
                camera_transform,
                object_transform,
                diameter,
                diameter,
            )
        }
        Sprite::Image(Image { texture }) => square_is_in_view_of_camera(
            camera,
            camera_transform,
            object_transform,
            texture.pixels[0].len() as f64,
            texture.pixels.len() as f64,
        ),
        Sprite::Animation(Animation {
            frames,
            current_frame,
            ..
        }) => {
            let current_texture = &frames[*current_frame].texture.pixels;
            let (width, height) = (
                current_texture[0].len() as f64,
                current_texture.len() as f64,
            );
            square_is_in_view_of_camera(camera, camera_transform, object_transform, width, height)
        }
    }
}

pub fn square_is_in_view_of_camera(
    camera: &Camera,
    camera_transform: Transform,
    square_transform: &Transform,
    square_width: f64,
    square_height: f64,
) -> bool {
    let camera_left = camera_transform.x - (camera.width as f64 / 2.0);
    let camera_right = camera_transform.x + (camera.width as f64 / 2.0);
    let camera_top = camera_transform.y - (camera.height as f64 / 2.0);
    let camera_bottom = camera_transform.y + (camera.height as f64 / 2.0);

    let square_left = square_transform.x - (square_width / 2.0);
    let square_right = square_transform.x + (square_width / 2.0);
    let square_top = square_transform.y - (square_height / 2.0);
    let square_bottom = square_transform.y + (square_height / 2.0);

    !(square_left > camera_right
        || square_right < camera_left
        || square_top > camera_bottom
        || square_bottom < camera_top)
}
