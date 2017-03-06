#![feature(core_intrinsics)]

extern crate time;
extern crate rand;
extern crate core;

mod game;

fn main() {
    for piece in game::board::pieces::iter() {
        print!("{} ", piece.index);
        println!("{}", piece.num_orientations);
    }
}

