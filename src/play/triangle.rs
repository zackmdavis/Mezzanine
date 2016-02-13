use std::io;
use std::io::Write;

use inference::triangle::{complexity_prior, Hypothesis, our_basic_hypotheses};


pub fn play() {
    wrapln!("Welcome to Mezzanine v. {}! Privately think of a criterion. \
             This program will attempt to efficiently infer the nature of \
             the criterion by asking you whether specific studies do or do \
             not have the property of satisfying the criterion.",
             env!("CARGO_PKG_VERSION"));

    let basic_hypotheses = our_basic_hypotheses();
    let mut beliefs = complexity_prior(basic_hypotheses);
    println!("Size of hypothesis space: {}", beliefs.len());

    let initial_entropy = beliefs.entropy();
    let mut question_count = 0;

    loop {
        let study = beliefs.burning_question(0.95, 10000);
        let value_of_continuing = beliefs.value_of_information(&study);
        if value_of_continuing == 0.0 {
            wrapln!("After asking {} questions (from an initial state \
                     of {:.3}-bit uncertainty),\n",
                    question_count, initial_entropy);

            if beliefs.len() == 1 {
                wrapln!("this program infers that a study has the \
                         property iff {}.",
                        beliefs.hypotheses()[0].description());
            } else {
                wrapln!("this program has inferred all that it can, and \
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
            question_count += 1;
            print!("Does the study below have the property? [Y/n/confess]\n\
                    {}\n(total pips: {})\n>> ",
                   study, study.pip_count());
            io::stdout().flush().expect("couldn't flush stdout?!");
            let mut input_buffer = String::new();
            io::stdin()
                .read_line(&mut input_buffer)
                .ok().expect("couldn't read stdin!?");
            verdict_maybe = match input_buffer.chars().nth(0) {
                Some('Y') | Some('y') => Some(true),
                Some('N') | Some('n') => Some(false),
                Some('C') | Some('c') => {
                    beliefs.confess(20);
                    continue;
                }
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
