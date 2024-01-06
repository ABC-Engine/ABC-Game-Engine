use crate::*;

/// A simple camera that can be used to move the view around
pub struct Camera {
    width: u32,
    height: u32,
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
    object_transform: Transform,
    object_width: f64,
    object_height: f64,
) -> bool {
    object_transform.x + object_width > camera_transform.x
        && object_transform.x < camera_transform.x + camera.width as f64
        && object_transform.y + object_height > camera_transform.y
        && object_transform.y < camera_transform.y + camera.height as f64
}
