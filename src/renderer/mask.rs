use crate::renderer::*;

/// the shape of a mask, not to be directly used on an entity, used to create a mask
pub enum MaskShape {
    Circle(Circle),
    Rectangle(Rectangle),
    Image(Image),
}

impl From<Circle> for MaskShape {
    fn from(circle: Circle) -> Self {
        MaskShape::Circle(circle)
    }
}

impl From<Rectangle> for MaskShape {
    fn from(rectangle: Rectangle) -> Self {
        MaskShape::Rectangle(rectangle)
    }
}

impl From<Image> for MaskShape {
    fn from(image: Image) -> Self {
        MaskShape::Image(image)
    }
}

/// A Mask can be put on a sprite to "cut out" a shape from the sprite (the cut out part will be transparent)
pub struct Mask {
    pub(crate) shape: MaskShape,
    pub transform: Transform,
}

impl Mask {
    pub fn new(shape: MaskShape, transform: Transform) -> Self {
        Mask { shape, transform }
    }
}
