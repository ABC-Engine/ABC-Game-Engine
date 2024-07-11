use core::panic;

use physics::rapier2d::na as nalgebra;
use physics::rapier2d::prelude::RigidBody;

use ui::Slider;
use ABC_Game_Engine::*;

fn main() {
    let mut scene = Scene::new();

    loop {
        scene.world.run();

        panic!("This is a panic");
    }
}
