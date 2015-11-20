#![feature(iter_arith)]

mod inference;

use inference::{Distribution, DivisibilityHypothesis};

#[cfg(not(test))]
fn main() {
    let hypotheses = (1..100u8).map(
        |n| DivisibilityHypothesis::new(n)).collect::<Vec<_>>();
    let prior = Distribution::ignorance_prior(hypotheses);

    let mut studies = (1..100u8)
        .map(|i| (i, prior.value_of_information(i)))
        .collect::<Vec<_>>();
    studies.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());

    for &study_assessment in &studies {
        let (study, value) = study_assessment;
        println!("value of learning about {} is {} bits", study, value);
    }
}
