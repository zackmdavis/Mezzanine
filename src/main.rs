#![feature(iter_arith, non_ascii_idents, test)]

extern crate argparse;
extern crate ansi_term;
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
