#![feature(iter_arith, test)]

extern crate argparse;
extern crate ansi_term;
extern crate itertools;
extern crate rand;
extern crate test;

#[macro_use] mod display;
#[macro_use] mod triangles;
mod play;
mod inference;


use play::play;

fn main() {
    play();
}
