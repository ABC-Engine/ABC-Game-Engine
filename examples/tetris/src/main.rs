// DO NOT USE THIS AS AN EXAMPLE YET
// this is not yet complete
use dioxus_debug_cell::RefCell; // better debugging, acts normal in release mode
use lazy_static::lazy_static;
use rand::Rng;
use std::io::{stdin, Write};
use std::rc::Rc;
use std::sync::Mutex;
use std::time::Instant;
use std::{any, vec};
use ABC_Game_Engine::*;

lazy_static! {
    static ref PIECE_GRID: Mutex<Vec<Vec<bool>>> = Mutex::new(vec![vec![false; 20]; 10]);
}

#[derive(Clone, Copy, Debug)]
struct Piece {
    name: &'static str,
    filename: &'static str,
    rotations: [[[bool; 4]; 4]; 4],
    current_rotation: usize,
}

const PIECES: [Piece; 7] = [
    Piece {
        name: "l-block",
        filename: "l-block.png",
        rotations: [
            [
                [false, false, false, false],
                [false, false, true, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, true, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, false],
                [true, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
        ],

        current_rotation: 0,
    },
    Piece {
        name: "j-block",
        filename: "j-block.png",
        rotations: [
            [
                [false, false, false, false],
                [true, false, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, false],
                [false, false, true, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [true, true, false, false],
            ],
        ],
        current_rotation: 0,
    },
    Piece {
        name: "i-block",
        filename: "i-block.png",
        rotations: [
            [
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
        ],
        current_rotation: 0,
    },
    Piece {
        name: "o-block",
        filename: "o-block.png",
        rotations: [
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
        ],
        current_rotation: 0,
    },
    Piece {
        name: "s-block",
        filename: "s-block.png",
        rotations: [
            [
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [false, true, true, false],
                [false, false, true, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
            ],
            [
                [false, false, false, false],
                [true, false, false, false],
                [true, true, false, false],
                [false, true, false, false],
            ],
        ],
        current_rotation: 0,
    },
    Piece {
        name: "z-block",
        filename: "z-block.png",
        rotations: [
            [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, false, true, false],
                [false, true, true, false],
                [false, true, false, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [true, true, false, false],
                [true, false, false, false],
            ],
        ],
        current_rotation: 0,
    },
    Piece {
        name: "t-block",
        filename: "t-block.png",
        rotations: [
            [
                [false, false, false, false],
                [false, true, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [false, true, true, false],
                [false, true, false, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, false],
                [false, true, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, false, false],
                [true, true, false, false],
                [false, true, false, false],
            ],
        ],
        current_rotation: 0,
    },
];

// every block will be made of 4x4 squares
struct PieceSquare {
    transform: Transform,
    sprite: Sprite,
    name: String,
}

#[derive(Clone)]
struct PieceObject {
    transform: Transform,
    sprite: Sprite,
    name: String,
    scene: Rc<RefCell<Scene>>,
    children: Vec<Rc<RefCell<Box<dyn Object>>>>,
    is_placed: bool,
    type_of_piece: Piece,
    spawn_instant: Instant,
    last_drop_time_ms: u128,
    time_between_drops_ms: u128,
    last_horizontal_move_time_ms: u128,
    time_between_horizontal_moves_ms: u128,
    last_rotation_time_ms: u128,
    time_between_rotations_ms: u128,
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

    fn get_children(&self) -> &[Rc<RefCell<Box<dyn Object>>>] {
        &self.children[..]
    }

    fn update(&mut self) {
        if self.is_placed {
            return;
        }

        if self.spawn_instant.elapsed().as_millis()
            > self.last_drop_time_ms + self.time_between_drops_ms
        {
            if is_valid_move(
                self.type_of_piece,
                &PIECE_GRID.lock().expect("failed to lock PIECE_GRID"),
                &self.transform,
                [0, 1],
            ) {
                //self.transform.y += 4.0;
            } else {
                // apply piece to grid
                update_piece_grid(self.type_of_piece, &self.transform);
                self.is_placed = true;
            }
        }

        //handle horizontal movement using crossterm events
        let key = get_input();
        if key == Option::Some(KeyCode::Char('a')) {
            if is_valid_move(
                self.type_of_piece,
                &PIECE_GRID.lock().expect("failed to lock PIECE_GRID"),
                &self.transform,
                [-1, 0],
            ) && self.spawn_instant.elapsed().as_millis()
                > self.last_horizontal_move_time_ms + self.time_between_horizontal_moves_ms
            {
                self.transform.x -= 4.0;
                self.last_horizontal_move_time_ms = self.spawn_instant.elapsed().as_millis();
            }
        } else if key == Option::Some(KeyCode::Char('d')) {
            if is_valid_move(
                self.type_of_piece,
                &PIECE_GRID.lock().expect("failed to lock PIECE_GRID"),
                &self.transform,
                [1, 0],
            ) && self.spawn_instant.elapsed().as_millis()
                > self.last_horizontal_move_time_ms + self.time_between_horizontal_moves_ms
            {
                self.transform.x += 4.0;
                self.last_horizontal_move_time_ms = self.spawn_instant.elapsed().as_millis();
            }
        } else if key == Option::Some(KeyCode::Char('s')) {
            if is_valid_move(
                self.type_of_piece,
                &PIECE_GRID.lock().expect("failed to lock PIECE_GRID"),
                &self.transform,
                [0, 1],
            ) {
                self.transform.y += 4.0;
                self.last_drop_time_ms = self.spawn_instant.elapsed().as_millis();
            }
        }
        // rotations
        if key == Option::Some(KeyCode::Char('q'))
            && self.spawn_instant.elapsed().as_millis()
                > self.last_rotation_time_ms + self.time_between_rotations_ms
        {
            rotate_object(self, true);
            self.transform.rotation += 90.0;
        } else if key == Option::Some(KeyCode::Char('e'))
            && self.spawn_instant.elapsed().as_millis()
                > self.last_rotation_time_ms + self.time_between_rotations_ms
        {
            rotate_object(self, false);
            self.transform.rotation -= 90.0;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn rotate_object(object: &mut PieceObject, is_left: bool) {
    let cloned_scene = object.scene.clone();
    let cloned_object = object.clone();
    let cloned_scene_for_closure = cloned_scene.clone();
    cloned_scene.borrow_mut().queue(Box::new(move || {
        let object_name = cloned_object.get_name().clone();
        let mut piece_clone = cloned_object.type_of_piece.clone();

        piece_clone.current_rotation = if is_left {
            (piece_clone.current_rotation + 1) % 4
        } else {
            let rotation_as_i8 = piece_clone.current_rotation as i8;
            (rotation_as_i8 - 1) as usize % 4
        };
        cloned_scene_for_closure
            .borrow_mut()
            .remove_object(&object_name);

        let mut new_object = new_piece(piece_clone, cloned_object.scene.clone());
        new_object.set_name(object_name.to_string());
        cloned_scene_for_closure.borrow_mut().add_object(new_object);
    }));
}

fn update_piece_grid(piece: Piece, transform: &Transform) {
    // change PIECE_GRID
    let offset = [
        ((transform.x / 4.0) as i8 + 1) as usize, // off by 1
        ((transform.y / 4.0) as i8) as usize,
    ];
    let mut old_piece_grid = PIECE_GRID.lock().expect("failed to lock PIECE_GRID");
    let mut new_piece_grid = old_piece_grid.clone();
    for (y, row) in piece.rotations[piece.current_rotation].iter().enumerate() {
        for (x, square) in row.iter().enumerate() {
            if *square {
                // should never be out of bounds
                new_piece_grid[(x + offset[0]) as usize][(y + offset[1]) as usize] = true;
            }
        }
    }
    let _ = std::mem::replace(&mut *old_piece_grid, new_piece_grid);
}

fn is_valid_move(
    piece: Piece,
    piece_grid: &Vec<Vec<bool>>,
    transform: &Transform,
    direction: [i8; 2],
) -> bool {
    let offset = (
        ((transform.x / 4.0) as i8 + direction[0]) as usize + 1, // off by 1
        ((transform.y / 4.0) as i8 + direction[1]) as usize,
    );
    for (y, row) in piece.rotations[piece.current_rotation].iter().enumerate() {
        for (x, square) in row.iter().enumerate() {
            if *square {
                // if not out of bounds
                if x + offset.0 > 0 && x + offset.0 < 10 && y + offset.1 > 0 && y + offset.1 < 20 {
                    if piece_grid[((x + offset.0) as usize).min(9)]
                        [((y + offset.1) as usize).min(19)]
                    {
                        // intersecting piece
                        return false;
                    }
                } else {
                    return false;
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
            x: 17.0,
            y: 2.0,
            rotation: 0.0,
            scale: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
        },
        sprite: Sprite::NoSprite(NoSprite),
        name: piece.name.to_string(),
        scene: scene.clone(),
        children: Vec::new(),
        is_placed: false,
        spawn_instant: Instant::now(),
        type_of_piece: piece,
        last_drop_time_ms: 0,
        time_between_drops_ms: 1000,
        last_horizontal_move_time_ms: 0,
        time_between_horizontal_moves_ms: 200,
        time_between_rotations_ms: 200,
        last_rotation_time_ms: 0,
    };

    for x in 0..4 {
        for y in 0..4 {
            if piece.rotations[piece.current_rotation][y][x] {
                let square = PieceSquare {
                    transform: Transform {
                        x: x as f64 * 4.0,
                        y: y as f64 * 4.0,
                        rotation: 0.0,
                        scale: 1.0,
                        origin_x: 0.0,
                        origin_y: 0.0,
                    },
                    sprite: Image {
                        texture: load_texture(format!("sprites/{}", piece.filename).as_str()),
                    }
                    .into(),
                    name: piece.name.to_string(),
                };
                new_piece
                    .children
                    .push(Rc::new(RefCell::new(Box::new(square))));
            }
        }
    }
    return new_piece;
}

fn main() {
    let mut renderer = Renderer::new(40, 80);
    // console settings need to be adjusted for this to work
    renderer.set_stretch(1.0);
    let scene = Scene::new();
    scene.set_background_color(Color {
        r: 100,
        g: 100,
        b: 100,
        a: 1.0,
    });

    let piece = new_random_piece(scene.clone());
    let mut piece_name = scene.borrow_mut().add_object(piece);

    loop {
        {
            renderer.render(scene.clone());
        }

        if let Some(tetris_block) = scene.borrow_mut().find_object(&piece_name) {
            if tetris_block
                .borrow()
                .as_any()
                .downcast_ref::<PieceObject>()
                .unwrap()
                .is_placed
            {
                let mut borrowed_scene = scene.borrow_mut();
                let piece = new_random_piece(scene.clone());
                piece_name = borrowed_scene.add_object(piece);
            }
        }
    }
}
