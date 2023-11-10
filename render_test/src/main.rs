use std::time::Instant;
use Console_Renderer::{new_renderer, Circle, Color, Object, Rectangle};

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = new_renderer(80, 40);
    renderer.add_object(Object::Rectangle(Rectangle {
        x: 10.0,
        y: 15.0,
        width: 7.0,
        height: 7.0,
        color: Color {
            r: 0,
            g: 255,
            b: 0,
            a: 1.0,
        },
    }));
    renderer.add_object(Object::Circle(Circle {
        x: 10.0,
        y: 20.0,
        radius: 10.0,
        color: Color {
            r: 255,
            g: 0,
            b: 0,
            a: 0.5,
        },
    }));

    loop {
        let start_of_frame_time = Instant::now();
        renderer.render();
        match &mut renderer.objects[1] {
            Object::Circle(circle) => circle.x += 0.05,
            Object::Rectangle(rectangle) => rectangle.x += 0.05,
        }
        loop {
            if start_of_frame_time.elapsed().as_millis() >= 16 {
                break;
            }
        }
    }
}
