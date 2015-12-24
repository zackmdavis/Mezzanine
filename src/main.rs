#![feature(iter_arith)]

extern crate argparse;
extern crate ansi_term;

#[macro_use] mod display;
mod inference;
mod play;
mod triangles;

use play::play;

fn main() {
    play();
}
