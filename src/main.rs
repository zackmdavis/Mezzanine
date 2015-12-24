#![feature(iter_arith)]

extern crate argparse;

#[macro_use] mod display;
mod inference;
mod play;

use play::play;

fn main() {
    play();
}
