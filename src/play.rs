use std::io;
use std::io::Write;

use argparse::{ArgumentParser, Store};

use inference::{BoundednessHypothesis, Distribution, DivisibilityHypothesis,
                Hypothesis, JoinedHypothesis};


pub fn play() {
    let mut bound: u16 = 30;
    {
        let mut arg_parser = ArgumentParser::new();
        arg_parser.set_description("Mezzanine: a guessing game");
        arg_parser.refer(&mut bound).add_option(
            &["--bound"], Store,
            "the largest admissible number in the game"
        );
        arg_parser.parse_args_or_exit();
    }
    bound += 1; // convenience with exclusive ranges

    println!("Welcome to Mezzanine v. {}! Privately think of a criterion. \
              This program will attempt to efficiently infer the nature of \
              the criterion by asking you whether specific numbers do or do \
              not have the property.", env!("CARGO_PKG_VERSION"));

    let studies = (1..bound).collect::<Vec<_>>();

    let divisibility_hypotheses = studies.iter()
        .take_while(|&n| *n <= bound/2)
        .map(|n| JoinedHypothesis::full_stop(DivisibilityHypothesis::new(*n)
                                             .to_basic())).collect::<Vec<_>>();
    println!("Number of divisibility hypotheses: {}",
             divisibility_hypotheses.len());

    let mut boundedness_hypotheses = Vec::new();
    for min in 2..bound-1 {
        for max in min..bound {
            boundedness_hypotheses.push(
                JoinedHypothesis::full_stop(
                    BoundednessHypothesis::new(min, max).to_basic()))
        }
    }
    println!("Number of boundedness hypotheses: {}",
             boundedness_hypotheses.len());

    let mut conjunctive_hypotheses = Vec::new();
    for boundedness_hypothesis in &boundedness_hypotheses {
        for divisibility_hypothesis in &divisibility_hypotheses {
            conjunctive_hypotheses.push(
                JoinedHypothesis::and(divisibility_hypothesis.proposition,
                                      boundedness_hypothesis.proposition)
            )
        }
    }
    println!("Number of conjunctive hypotheses: {}",
             conjunctive_hypotheses.len());

    let mut disjunctive_hypotheses = Vec::new();
    for boundedness_hypothesis in &boundedness_hypotheses {
        for divisibility_hypothesis in &divisibility_hypotheses {
            disjunctive_hypotheses.push(
                JoinedHypothesis::or(divisibility_hypothesis.proposition,
                                     boundedness_hypothesis.proposition)
            )
        }
    }
    println!("Number of disjunctive hypotheses: {}",
             disjunctive_hypotheses.len());

    let mut hypotheses = Vec::new();
    hypotheses.extend(divisibility_hypotheses);
    hypotheses.extend(boundedness_hypotheses);
    hypotheses.extend(conjunctive_hypotheses);
    hypotheses.extend(disjunctive_hypotheses);

    let mut beliefs = Distribution::ignorance_prior(hypotheses);
    println!("Size of hypothesis space: {}", beliefs.len());

    loop {
        let complete_certainty = beliefs.completely_certain();
        match complete_certainty {
            None => {
                let study = beliefs.burning_question(studies.clone()).unwrap();
                let voi = beliefs.value_of_information(study);
                if voi == 0.0 {
                    println!("This program has inferred all that it can, and \
                              is indifferent between the following hypotheses \
                              concerning when a number has the property:");
                    for hypothesis in beliefs.hypotheses() {
                        println!("  * {}", hypothesis.description());
                    }
                    break;
                }
                println!("This program's belief distribution (over {} remaining \
                          hypotheses) has an entropy of {:.3} bits. Learning \
                          whether {} has the property is expected to reduce the \
                          entropy by {:.3} bits.",
                         beliefs.len(), beliefs.entropy(), study, voi);
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
