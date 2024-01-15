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

// TODO: benchmark this, it should be faster than just rendering everything,
// but that might not be the case now
/// returns true if the given rectangle is in view of the camera
/// used for culling by the renderer
pub fn object_is_in_view_of_camera(
    camera: &Camera,
    object_transform: &Transform,
    object_sprite: &Sprite,
) -> bool {
    // the object is already offset by the camera's position
    match object_sprite {
        Sprite::Rectangle(Rectangle { width, height, .. }) => {
            let max_dist = (height.powi(2) + width.powi(2)).sqrt(); // this is the maximum width that a square would need to be to cover the rectangle at any rotation
            square_is_in_view_of_camera(camera, object_transform, max_dist as f64, max_dist as f64)
        }
        Sprite::Circle(Circle { radius, .. }) => {
            let diameter = *radius as f64 * 2.0;
            square_is_in_view_of_camera(camera, object_transform, diameter, diameter)
        }
        Sprite::Image(Image { texture }) => {
            let (texture_width, texture_height) =
                (texture.pixels[0].len() as f64, texture.pixels.len() as f64);
            let max_dist = (texture_width.powi(2) + texture_height.powi(2)).sqrt();
            square_is_in_view_of_camera(camera, object_transform, max_dist as f64, max_dist as f64)
        }
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
            square_is_in_view_of_camera(camera, object_transform, width, height)
        }
    }
}

pub fn square_is_in_view_of_camera(
    camera: &Camera,
    square_transform: &Transform,
    square_width: f64,
    square_height: f64,
) -> bool {
    // keep in mind that the square is already offset by the camera's position, so we don't need to add it here
    // example a square in the top left corner of the screen would have a position of (0, 0) by the time it gets here
    let camera_left = 0.0;
    let camera_right = camera.width as f64;
    let camera_top = 0.0;
    let camera_bottom = camera.height as f64;

    let square_left = square_transform.x - (square_width / 2.0);
    let square_right = square_transform.x + (square_width / 2.0);
    let square_top = square_transform.y - (square_height / 2.0);
    let square_bottom = square_transform.y + (square_height / 2.0);

    !(square_left > camera_right
        || square_right < camera_left
        || square_top > camera_bottom
        || square_bottom < camera_top)
}

#[cfg(test)]
mod camera_tests {
    #[test]
    fn square_is_in_view_of_camera() {
        use crate::{camera::Camera, *};
        use rand::Rng;

        let mut rng = rand::thread_rng();

        let mut camera = Camera::default();
        camera.set_size(100, 100);

        let mut camera_transform = Transform::default();
        camera_transform.x = 0.0;
        camera_transform.y = 0.0;

        let mut square_transform = Transform::default();
        square_transform.x = 50.0;
        square_transform.y = 50.0;

        for _ in 0..100000 {
            // fudge the square's position and size
            square_transform.x = rng.gen_range(0.0..=100.0);
            square_transform.y = rng.gen_range(0.0..=100.0);

            let square_width = rng.gen_range(0.0..=100.0);
            let square_height = rng.gen_range(0.0..=100.0);

            assert!(super::square_is_in_view_of_camera(
                &camera,
                &square_transform,
                square_width,
                square_height
            ));
        }
    }

    #[test]
    fn object_is_in_view_of_camera() {
        use crate::{camera::Camera, *};
        use rand::Rng;

        let mut rng = rand::thread_rng();

        let mut camera = Camera::default();
        camera.set_size(100, 100);

        let mut square_transform = Transform::default();
        square_transform.x = 50.0;
        square_transform.y = 50.0;

        for _ in 0..100000 {
            // fudge the square's position and size
            square_transform.x = rng.gen_range(0.0..=100.0);
            square_transform.y = rng.gen_range(0.0..=100.0);

            let square_width = rng.gen_range(0.0..=100.0);
            let square_height = rng.gen_range(0.0..=100.0);

            assert!(super::object_is_in_view_of_camera(
                &camera,
                &square_transform,
                &Sprite::Rectangle(Rectangle {
                    width: square_width,
                    height: square_height,
                    color: Color::default(),
                })
            ));
        }
    }
}
