// this highlights some major issues with the current renderer
use std::time::Instant;
use Console_Renderer::{new_renderer, Circle, Color, Object, Rectangle};

struct BouncingBall {
    x_velocity: f64,
    y_velocity: f64,
    x: f64,
    y: f64,
}

const window_size: (u32, u32) = (80, 40);
const circle_radius: f64 = 5.0;

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = new_renderer(window_size.0, window_size.1);
    let rectangle = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
        color: Color {
            r: 0,
            g: 0,
            b: 0,
            a: 1.0,
        },
    };
    let circle = Circle {
        x: 10.0,
        y: 20.0,
        radius: circle_radius,
        color: Color {
            r: 255,
            g: 0,
            b: 0,
            a: 0.5,
        },
    };

    let mut bouncy_ball = BouncingBall {
        x_velocity: 0.00,
        y_velocity: 0.00,
        x: 10.0,
        y: 20.0,
    };

    renderer.add_object(Object::Rectangle(rectangle));
    renderer.add_object(Object::Circle(circle));

    loop {
        let start_of_frame_time = Instant::now();
        let circle_index = 1;

        // definitely shouldn't have to do this
        match &mut renderer.objects[1] {
            Object::Circle(circle) => circle.y = bouncy_ball.y,
            Object::Rectangle(rectangle) => rectangle.x += 0.05,
        }

        bouncy_ball.y_velocity += 0.1;

        bouncy_ball.x += bouncy_ball.x_velocity;
        bouncy_ball.y += bouncy_ball.y_velocity;

        if bouncy_ball.y + circle_radius >= window_size.1 as f64
            || bouncy_ball.y - circle_radius <= 0.0 as f64
        {
            bouncy_ball.y_velocity *= -1.0;
        }
        if bouncy_ball.x + circle_radius >= window_size.0 as f64
            || bouncy_ball.x - circle_radius <= 0.0 as f64
        {
            bouncy_ball.x_velocity *= -1.0;
        }

        renderer.render();

        loop {
            if start_of_frame_time.elapsed().as_millis() >= 16 {
                break;
            }
        }
    }
}
