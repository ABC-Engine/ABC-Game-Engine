// DO NOT USE THIS AS AN EXAMPLE YET
// this is not yet complete
use console_renderer::*;
use rand::Rng;
use std::cell::RefCell;
use std::io::stdin;
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

impl Object for PieceSquare {
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
        &[]
    }

    fn update(&mut self) {}
}

struct PieceObject {
    transform: Transform,
    sprite: Sprite,
    name: String,
    scene: Rc<RefCell<Scene>>,
    children: Vec<Box<dyn Object>>,
    is_placed: bool,
    spawn_instant: Instant,
    last_drop_time_ms: u128,
    time_between_drops_ms: u128,
    last_horizontal_move_time_ms: u128,
    time_between_horizontal_moves_ms: u128,
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
        if self.transform.y < 77.0
            && (self.spawn_instant.elapsed().as_millis()
                > self.last_drop_time_ms + self.time_between_drops_ms)
        {
            self.transform.y += 4.0;
            self.last_drop_time_ms = self.spawn_instant.elapsed().as_millis();
        } else {
            self.is_placed = true;
        }

        //handle horizontal movement using crossterm events
        let key = get_input();
        if key == Option::Some(KeyCode::Char('a')) {
            if self.transform.x > 2.0
                && self.spawn_instant.elapsed().as_millis()
                    > self.last_horizontal_move_time_ms + self.time_between_horizontal_moves_ms
            {
                self.transform.x -= 4.0;
                self.last_horizontal_move_time_ms = self.spawn_instant.elapsed().as_millis();
            }
        } else if key == Option::Some(KeyCode::Char('d')) {
            if self.transform.x < 36.0
                && self.spawn_instant.elapsed().as_millis()
                    > self.last_horizontal_move_time_ms + self.time_between_horizontal_moves_ms
            {
                self.transform.x += 4.0;
                self.last_horizontal_move_time_ms = self.spawn_instant.elapsed().as_millis();
            }
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
    let mut new_piece = PieceObject {
        transform: Transform {
            x: 2.0,
            y: 2.0,
            rotation: 0.0,
            scale: 1.0,
        },
        sprite: Sprite::NoSprite(NoSprite),
        name: piece.name.to_string(),
        scene: scene.clone(),
        children: Vec::new(),
        is_placed: false,
        spawn_instant: Instant::now(),
        last_drop_time_ms: 0,
        time_between_drops_ms: 500,
        last_horizontal_move_time_ms: 0,
        time_between_horizontal_moves_ms: 200,
    };

    for x in 0..4 {
        for y in 0..4 {
            if piece.arrangement[y][x] {
                let mut square = PieceSquare {
                    transform: Transform {
                        x: x as f64 * 4.0,
                        y: y as f64 * 4.0,
                        rotation: 0.0,
                        scale: 1.0,
                    },
                    sprite: Image {
                        texture: load_texture(format!("sprites/{}", piece.filename).as_str()),
                    }
                    .into(),
                    name: piece.name.to_string(),
                };
                new_piece.children.push(Box::new(square));
            }
        }
    }
    return new_piece;
}

fn main() {
    let mut renderer = Renderer::new(40, 80);
    // console settings need to be adjusted for this to work
    renderer.set_stretch(1.0);
    let mut scene = Rc::new(RefCell::new(Scene::new()));
    scene.borrow_mut().background_color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 1.0,
    };

    let mut piece = new_random_piece(scene.clone());
    scene.borrow_mut().add_object(piece);

    let mut piece_grid = vec![vec![false; 20]; 10];

    loop {
        //scene.borrow_mut().update_objects();
        renderer.render(&mut *scene.borrow_mut());
    }
}
