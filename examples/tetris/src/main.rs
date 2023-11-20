// DO NOT USE THIS AS AN EXAMPLE YET
// this is not yet complete
use console_renderer::*;
use std::cell::RefCell;
use std::rc::Rc;

// every block will be made of 4x4 squares

struct LPiece {
    transform: Transform,
    sprite: Sprite,
    name: String,
    scene: Rc<RefCell<Scene>>,
}

impl Object for LPiece {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn update(&mut self) {
        if self.transform.y < 77.0 {
            self.transform.y += 0.1;
        }
    }
}

fn main() {
    let mut renderer = Renderer::new(40, 80);
    renderer.set_stretch(1.0);
    let mut scene = Rc::new(RefCell::new(Scene::new()));
    scene.borrow_mut().background_color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 1.0,
    };

    let mut piece_grid = vec![vec![0; 20]; 10];

    let mut l_piece = LPiece {
        transform: Transform {
            x: 2.0,
            y: 20.0,
            rotation: 0.0,
            scale: 1.0,
        },
        sprite: Image {
            texture: load_texture("sprites/l-block.png"),
        }
        .into(),
        name: String::from("LPiece"),
        scene: scene.clone(),
    };

    scene.borrow_mut().add_object(l_piece);

    loop {
        renderer.render(&mut *scene.borrow_mut());
    }
}
