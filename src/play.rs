use std::collections::HashSet;
use std::io;
use std::io::Write;

use argparse::{ArgumentParser, Store};

use number_inference::{BoundednessHypothesis, Distribution,
                       DivisibilityHypothesis,
                       Hypothesis, JoinedHypothesis};
use triangles::{Triangle, Stack, Study, Color, Size};


pub fn decorative_display_header() {
    let mut stack = Stack::new();
    stack.push(Triangle::new(Color::Green, Size::Three));
    stack.push(Triangle::new(Color::Yellow, Size::Two));
    stack.push(Triangle::new(Color::Red, Size::One));

    let mut another_stack = Stack::new();
    another_stack.push(Triangle::new(Color::Yellow, Size::Three));
    another_stack.push(Triangle::new(Color::Blue, Size::Two));

    let mut mascot_study = Study::new();
    mascot_study.append(stack);
    mascot_study.append(another_stack);
    println!("{}", mascot_study);
}


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

    decorative_display_header();
    wrapln!("Welcome to Mezzanine v. {}! Privately think of a criterion \
             concerning natural numbers not greater than {}. \
             This program will attempt to efficiently infer the nature of \
             the criterion by asking you whether specific numbers do or do \
             not have the property of satisfying the criterion.",
             env!("CARGO_PKG_VERSION"), bound);

    bound += 1; // convenience with exclusive ranges
    let studies = (1..bound).collect::<Vec<_>>();

    let mut hypotheses = Vec::new();

    // divisibility
    for divisor in 1..bound/2 {
        hypotheses.push(JoinedHypothesis::full_stop(
            DivisibilityHypothesis::new(divisor).to_basic()));
    }

    // boundedness
    for min in 2..bound-1 {
        for max in min..bound {
            hypotheses.push(
                JoinedHypothesis::full_stop(
                    BoundednessHypothesis::new(min, max).to_basic()))
        }
    }

    // conjunctions and disjunctions of divisibility and boundedness
    //
    // This approach is a little uglier for users than actually reasoning about
    // ranges (it'll choose the first encountered of allowable bounds for a
    // prediction, rather than the tightest bound), but it's more generalizable
    let mut joined_predictions = HashSet::new();
    for divisor in 1..bound/2 {
        for min in 2..bound-1 {
            for max in min..bound {
                let multiples = (1..bound)
                    .map(|i| i*divisor)
                    .take_while(|&n| n <= bound)
                    .collect::<HashSet<_>>();
                let range = (min..max+1).collect::<HashSet<_>>();

                let mut conjunctive_prediction = multiples.intersection(&range)
                    .cloned().collect::<Vec<_>>();
                conjunctive_prediction.sort();
                let mut disjunctive_prediction = multiples.union(&range)
                    .cloned().collect::<Vec<_>>();
                disjunctive_prediction.sort();

                let mut factor_prediction = multiples.iter()
                    .cloned().collect::<Vec<_>>();
                factor_prediction.sort();
                let mut range_prediction = range.iter()
                    .cloned().collect::<Vec<_>>();
                range_prediction.sort();

                if conjunctive_prediction != factor_prediction &&
                        conjunctive_prediction != range_prediction &&
                        !joined_predictions.contains(&conjunctive_prediction) {
                    hypotheses.push(
                        JoinedHypothesis::and(
                            DivisibilityHypothesis::new(divisor).to_basic(),
                            BoundednessHypothesis::new(min, max).to_basic())
                    );
                    joined_predictions.insert(conjunctive_prediction);
                }
                if disjunctive_prediction != factor_prediction &&
                        disjunctive_prediction != range_prediction &&
                        !joined_predictions.contains(&disjunctive_prediction) {
                    hypotheses.push(
                        JoinedHypothesis::or(
                            DivisibilityHypothesis::new(divisor).to_basic(),
                            BoundednessHypothesis::new(min, max).to_basic())
                    );
                    joined_predictions.insert(disjunctive_prediction);
                }
            }
        }
    }

    let mut beliefs = Distribution::ignorance_prior(hypotheses);
    println!("Size of hypothesis space: {}", beliefs.len());

    loop {
        let complete_certainty = beliefs.completely_certain();
        match complete_certainty {
            None => {
                let study = beliefs.burning_question(studies.clone()).unwrap();
                let voi = beliefs.value_of_information(study);
                if voi == 0.0 {
                    wrapln!("This program has inferred all that it can, and \
                              is indifferent between the following hypotheses \
                              concerning when a number has the property:");
                    for hypothesis in beliefs.hypotheses() {
                        println!("  * {}", hypothesis.description());
                    }
                    break;
                }
                wrapln!("This program's belief distribution (over {} remaining \
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
                            wrapln!("\nAnswer Y or n. You must comply.");
                            continue;
                        },
                    };
                }
                let verdict = verdict_maybe.unwrap();
                wrapln!("On the question of whether {} has the property, \
                         you said {}.\n", study, verdict);
                beliefs = beliefs.updated(study, verdict);
            }
            Some(known_truth) => {
                wrapln!("This program infers that a natural number has the \
                         property iff {}.", known_truth.description());
                return;
            }
        }
    }
}
