#![feature(iter_arith)]

extern crate argparse;
extern crate ansi_term;
extern crate itertools;

#[macro_use] mod display;
#[macro_use] mod triangles;
mod play;
mod inference;


use play::play;

fn main() {
    play();
}
