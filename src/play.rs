use std::io;
use std::io::Write;

use argparse::{ArgumentParser, Store};

use inference::{Distribution, DivisibilityHypothesis, Hypothesis,
                JoinedHypothesis};


pub fn play() {
    let mut bound: u16 = 100;
    {
        let mut arg_parser = ArgumentParser::new();
        arg_parser.set_description("Mezzanine: a guessing game");
        arg_parser.refer(&mut bound).add_option(
            &["--bound"], Store,
            "the largest admissible number in the game"
        );
        arg_parser.parse_args_or_exit();
    }

    println!("Welcome to Mezzanine v. {}! Privately think of a criterion. \
              This program will attempt to efficiently infer the nature of \
              the criterion by asking you whether specific numbers do or do \
              not have the property.", env!("CARGO_PKG_VERSION"));

    let studies = (1..bound).collect::<Vec<_>>();
    let hypotheses = studies.iter()
        .map(|n| JoinedHypothesis::full_stop(DivisibilityHypothesis::new(*n).to_basic())).collect::<Vec<_>>();

    let mut beliefs = Distribution::ignorance_prior(hypotheses);

    loop {
        match beliefs.completely_certain() {
            None => {
                let study = beliefs.burning_question(studies.clone()).unwrap();
                println!("This program's belief distribution has an entropy \
                          of {:.3} bits. Learning whether {} has the property \
                          is expected to reduce the entropy by {:.3} bits.",
                         beliefs.entropy(), study,
                         beliefs.value_of_information(study));
                let mut verdict_maybe = None;
                while let None = verdict_maybe {
                    print!("Does {} have the property? [Y/n] >> ", study);
                    io::stdout().flush().expect("couldn't flush stdout?!");
                    let mut input_buffer = String::new();
                    io::stdin()
                        .read_line(&mut input_buffer)
                        .ok().expect("couldn't read stdin!?");
                    verdict_maybe = match input_buffer.chars().nth(0) {
                        Some('Y') | Some('y') => Some(true),
                        Some('N') | Some('n') => Some(false),
                        _ => {
                            println!("\nAnswer Y or n. You must comply.");
                            continue;
                        },
                    };
                }
                let verdict = verdict_maybe.unwrap();
                println!("On the question of whether {} has the property, \
                          you said {}.\n", study, verdict);
                beliefs = beliefs.updated(study, verdict);
            }
            Some(known_truth) => {
                println!("This program infers that a natural number has the \
                          property iff {}.", known_truth.description());
                return;
            }
        }
    }
}
