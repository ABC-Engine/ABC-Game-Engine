// this highlights some major issues with the current renderer
use std::time::Instant;
use Console_Renderer::*;

struct BouncingBall {
    transform: Transform,
    sprite: Sprite,
    x_velocity: f64,
    y_velocity: f64,
}

impl Update for BouncingBall {
    fn update(&mut self) {
        self.y_velocity += 0.1;

        self.transform.x += self.x_velocity;
        self.transform.y += self.y_velocity;

        if self.transform.y + CIRCLE_RADIUS >= WINDOW_DIMS.1 as f64
            || self.transform.y - CIRCLE_RADIUS <= 0.0 as f64
        {
            self.y_velocity *= -1.0;
        }
        if self.transform.x + CIRCLE_RADIUS >= WINDOW_DIMS.0 as f64
            || self.transform.x - CIRCLE_RADIUS <= 0.0 as f64
        {
            self.x_velocity *= -1.0;
        }
    }
}

impl Object for BouncingBall {
    fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

const WINDOW_DIMS: (u32, u32) = (80, 40);
const CIRCLE_RADIUS: f64 = 5.0;

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let mut renderer = new_renderer(WINDOW_DIMS.0, WINDOW_DIMS.1);
    let mut scene = Scene::new();
    let rectangle = Rectangle {
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
        radius: CIRCLE_RADIUS,
        color: Color {
            r: 255,
            g: 0,
            b: 0,
            a: 0.5,
        },
    };

    let mut bouncy_ball = BouncingBall {
        transform: Transform {
            x: 10.0,
            y: 20.0,
            rotation: 0.0,
            scale: 1.0,
        },
        sprite: Sprite::Circle(circle),
        x_velocity: 0.00,
        y_velocity: 0.00,
    };

    scene.add_object(bouncy_ball);

    loop {
        let start_of_frame_time = Instant::now();
        let circle_index = 1;

        renderer.render(&mut scene);

        loop {
            if start_of_frame_time.elapsed().as_millis() >= 16 {
                break;
            }
        }
    }
}
