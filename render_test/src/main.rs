// this highlights some major issues with the current renderer
use console_renderer::*;
use std::{thread, time, time::Instant};

// for an actual game, it might be advantageous to declare each of these objects in their own files
struct BouncingBall {
    transform: Transform,
    sprite: Sprite,
    x_velocity: f64,
    y_velocity: f64,
}

impl Object for BouncingBall {
    fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn update(&mut self) {
        self.y_velocity += 0.1;

        self.transform.x += self.x_velocity;
        self.transform.y += self.y_velocity;

        if self.transform.y >= WINDOW_DIMS.1 as f64 {
            self.y_velocity *= -0.9;
            self.transform.y = WINDOW_DIMS.1 as f64;
        } else if self.transform.y <= 0.0 as f64 {
            self.y_velocity *= -0.9;
            self.transform.y = 0.0;
        }
        if self.transform.x >= WINDOW_DIMS.0 as f64 {
            self.x_velocity *= -0.9;
            self.transform.x = WINDOW_DIMS.1 as f64
        } else if self.transform.x <= 0.0 as f64 {
            self.x_velocity *= -0.9;
            self.transform.x = 0.0;
        }
    }
}

struct PlagueMask {
    transform: Transform,
    sprite: Sprite,
}

impl Object for PlagueMask {
    fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn update(&mut self) {
        self.transform.rotation += 1.0;
    }
}

struct RotatingRectangle {
    transform: Transform,
    sprite: Sprite,
}

impl Object for RotatingRectangle {
    fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn update(&mut self) {
        self.transform.rotation += 1.0;
    }
}

const WINDOW_DIMS: (u32, u32) = (80, 40);
const CIRCLE_RADIUS: f64 = 5.0;

// Note: this does not work in vscode terminal, but it does work in the windows terminal
fn main() {
    let renderer = Renderer::new(WINDOW_DIMS.0, WINDOW_DIMS.1);
    let mut scene = Scene::new();
    scene.set_background_color(Color {
        r: 100,
        g: 0,
        b: 0,
        a: 1.0,
    });

    let circle = Circle {
        radius: CIRCLE_RADIUS,
        color: Color {
            r: 255,
            g: 0,
            b: 0,
            a: 1.0,
        },
    };

    let plague_mask = Image {
        texture: load_texture("Sample_Images/Icon10_01.png", 2.3),
    };

    let rectangle = Rectangle {
        width: 10.0,
        height: 10.0,
        color: Color {
            r: 0,
            g: 255,
            b: 0,
            a: 1.0,
        },
    };

    let bouncy_ball = BouncingBall {
        transform: Transform {
            x: 10.0,
            y: 20.0,
            rotation: 0.0,
            scale: 1.0,
        },
        sprite: circle.into(),
        x_velocity: 1.00,
        y_velocity: 0.00,
    };

    let plague_mask_object = PlagueMask {
        transform: Transform {
            x: 20.0,
            y: 20.0,
            rotation: 0.0,
            scale: 1.0,
        },
        sprite: plague_mask.into(),
    };

    let rotating_rectangle = RotatingRectangle {
        transform: Transform {
            x: 20.0,
            y: 20.0,
            rotation: 0.0,
            scale: 1.0,
        },
        sprite: rectangle.into(),
    };

    //scene.add_object(bouncy_ball);
    scene.add_object(plague_mask_object);
    //scene.add_object(rotating_rectangle);

    loop {
        let start_of_frame_time = Instant::now();

        renderer.render(&mut scene);

        thread::sleep(time::Duration::from_millis(
            (32 as f32 - start_of_frame_time.elapsed().as_millis() as f32).max(0.0) as u64,
        )); // 30 fps
    }
}
