// DO NOT USE THIS AS AN EXAMPLE YET
// this is not yet complete
use console_renderer::*;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
struct Piece<'a> {
    name: &'a str,
    filename: &'a str,
    arrangement: [[bool; 4]; 4],
    rotation_origin: (u8, u8),
}

const PIECES: [Piece; 7] = [
    Piece {
        name: "l-block",
        filename: "l-block.png",
        arrangement: [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, true, false],
            [true, true, true, false],
        ],
        rotation_origin: (1, 1),
    },
    Piece {
        name: "j-block",
        filename: "j-block.png",
        arrangement: [
            [false, true, false, false],
            [false, true, false, false],
            [true, true, false, false],
            [false, false, false, false],
        ],
        rotation_origin: (1, 1),
    },
    Piece {
        name: "i-block",
        filename: "i-block.png",
        arrangement: [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
        rotation_origin: (1, 1),
    },
    Piece {
        name: "o-block",
        filename: "o-block.png",
        arrangement: [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        rotation_origin: (1, 1),
    },
    Piece {
        name: "s-block",
        filename: "s-block.png",
        arrangement: [
            [false, false, false, false],
            [false, true, true, false],
            [true, true, false, false],
            [false, false, false, false],
        ],
        rotation_origin: (1, 1),
    },
    Piece {
        name: "z-block",
        filename: "z-block.png",
        arrangement: [
            [false, false, false, false],
            [true, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        rotation_origin: (1, 1),
    },
    Piece {
        name: "t-block",
        filename: "t-block.png",
        arrangement: [
            [false, false, false, false],
            [true, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        rotation_origin: (1, 1),
    },
];

// every block will be made of 4x4 squares
struct PieceSquare {
    transform: Transform,
    sprite: Sprite,
    name: String,
}

struct PieceObject {
    transform: Transform,
    sprite: Sprite,
    name: String,
    scene: Rc<RefCell<Scene>>,
    children: Vec<Box<dyn Object>>,
}

impl Object for PieceObject {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_children(&self) -> &[Box<dyn Object>] {
        &self.children[..]
    }

    fn update(&mut self) {
        if self.transform.y < 77.0 {
            self.transform.y += 0.1;
        }
    }
}

fn is_valid_move(piece: Piece, piece_grid: &Vec<Vec<bool>>) -> bool {
    for (y, row) in piece_grid.iter().enumerate() {
        for (x, square) in row.iter().enumerate() {
            if *square {
                if x < 10 && y < 20 {
                    if piece.arrangement[y][x] {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn new_random_piece(scene: Rc<RefCell<Scene>>) -> impl Object {
    let piece = PIECES[rand::random::<usize>() % PIECES.len()];
    new_piece(piece, scene)
}

fn new_piece(piece: Piece, scene: Rc<RefCell<Scene>>) -> impl Object {
    PieceObject {
        transform: Transform {
            x: 2.0,
            y: 20.0,
            rotation: 0.0,
            scale: 1.0,
        },
        sprite: Image {
            texture: load_texture(format!("sprites/{}", piece.filename).as_str()),
        }
        .into(),
        name: piece.name.to_string(),
        scene: scene.clone(),
        children: Vec::new(),
    }
}

fn main() {
    let mut renderer = Renderer::new(40, 80);
    // console settings need to be adjusted for this to work
    renderer.set_stretch(1.0);
    let mut scene = Rc::new(RefCell::new(Scene::new()));
    scene.borrow_mut().background_color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 1.0,
    };

    let mut piece_grid = vec![vec![false; 20]; 10];

    loop {
        renderer.render(&mut *scene.borrow_mut());
    }
}
