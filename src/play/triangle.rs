use std::io;
use std::io::Write;

use triangles::{Color, Size};
use inference::triangle::{complexity_prior, Hypothesis};
use inference::triangle::hypotheses::BasicHypothesis;
use inference::triangle::hypotheses::color_count_boundedness::ColorCountBoundednessHypothesis;
use inference::triangle::hypotheses::size_count_boundedness::SizeCountBoundednessHypothesis;


pub fn play() {
    wrapln!("Welcome to Mezzanine v. {}! Privately think of a criterion. \
             This program will attempt to efficiently infer the nature of \
             the criterion by asking you whether specific studies do or do \
             not have the property of satisfying the criterion.",
             env!("CARGO_PKG_VERSION"));

    let mut hypotheses = Vec::new();
    for &color in Color::iter() {
        for lower in 1..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    ColorCountBoundednessHypothesis::new_lower(
                        color, lower)));
        }
        for upper in 0..3 {
            hypotheses.push(
                BasicHypothesis::from(
                    ColorCountBoundednessHypothesis::new_upper(
                        color, upper)));
        }
    }

    for &size in Size::iter() {
        for lower in 1..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    SizeCountBoundednessHypothesis::new_lower(
                        size, lower)));
        }
        for upper in 0..3 {
            hypotheses.push(
                BasicHypothesis::from(
                    SizeCountBoundednessHypothesis::new_upper(
                        size, upper)));
        }
    }

    let mut beliefs = complexity_prior(hypotheses);
    println!("Size of hypothesis space: {}", beliefs.len());

    loop {
        let study = beliefs.burning_question(0.95, 10000);
        let value_of_continuing = beliefs.value_of_information(&study);
        if value_of_continuing == 0.0 {
            if beliefs.len() == 1 {
                wrapln!("This program infers that a study has the \
                         property iff {}.",
                        beliefs.hypotheses()[0].description());
            } else {
                wrapln!("This program has inferred all that it can, and \
                         is indifferent between the following hypotheses \
                         concerning when a study has the property:");
                for hypothesis in beliefs.hypotheses() {
                    println!("  * {}", hypothesis.description());
                }
            }
            break;
        }
        wrapln!("This program's belief distribution (over {} remaining \
                 hypotheses) has an entropy of {:.3} bits. Learning \
                 whether the following study has the property is expected \
                 to reduce the entropy by {:.3} bits.",
                beliefs.len(), beliefs.entropy(), value_of_continuing);
        let mut verdict_maybe = None;
        while let None = verdict_maybe {
            print!("Does the study below have the property? [Y/n]\n{}>> ",
                   study);
            io::stdout().flush().expect("couldn't flush stdout?!");
            let mut input_buffer = String::new();
            io::stdin()
                .read_line(&mut input_buffer)
                .ok().expect("couldn't read stdin!?");
            verdict_maybe = match input_buffer.chars().nth(0) {
                Some('Y') | Some('y') => Some(true),
                Some('N') | Some('n') => Some(false),
                _ => {
                    wrapln!("\nAnswer Y or n. You must comply.");
                    continue;
                },
            };
        }

        let verdict = verdict_maybe.unwrap();
        wrapln!("On the question of whether the previous study had the \
                 property, you said {}.\n", verdict);
        beliefs = beliefs.updated(&study, verdict);
    }
}
