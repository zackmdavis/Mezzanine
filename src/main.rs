#![feature(iter_arith)]

extern crate argparse;
extern crate ansi_term;

#[macro_use] mod display;
#[macro_use] mod triangles;
mod play;
mod number_inference;
mod triangle_inference;

use play::play;

fn main() {
    play();
}
