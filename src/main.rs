#![feature(iter_arith)]

extern crate argparse;

mod inference;
mod play;

use play::play;

fn main() {
    play();
}
