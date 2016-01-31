#![allow(unused_imports, unused_mut)]

use triangles::{Color, COLORS, Triangle, Size, Stack, Study};
use inference::triangle::{ColorBoundednessHypothesis, Distribution};

pub fn play() {
    wrapln!("Welcome to Mezzanine v. {}! Privately think of a criterion. \
             This program will attempt to efficiently infer the nature of \
             the criterion by asking you whether specific studies do or do \
             not have the property of satisfying the criterion.",
             env!("CARGO_PKG_VERSION"));

    let mut hypotheses = Vec::new();
    for &color in Color::iter() {
        for lower in 1..5 {
            hypotheses.push(ColorBoundednessHypothesis::new_lower(color, lower));
        }
        for upper in 0..4 {
            hypotheses.push(ColorBoundednessHypothesis::new_upper(color, upper));
        }
    }

    let mut beliefs = Distribution::ignorance_prior(hypotheses);
    println!("Size of hypothesis space: {}", beliefs.len());

    loop {
        // TODO
        break;
    }
}
