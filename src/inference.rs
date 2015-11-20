#![allow(dead_code)]

use std::collections::HashMap;
use std::process::Command;
use std::hash::Hash;
use std::cmp::Eq;
use std::iter::FromIterator;
use std::f64::NEG_INFINITY;

static PRIMES: [u8; 25] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
                           43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

pub type Study = u8;

pub trait Hypothesis {
    fn predicts_the_property(&self, study: Study) -> bool;
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct DivisibilityHypothesis {
    n: u8
}

impl DivisibilityHypothesis {
    pub fn new(n: u8) -> Self {
        DivisibilityHypothesis { n: n }
    }
}

impl Hypothesis for DivisibilityHypothesis {
    fn predicts_the_property(&self, study: Study) -> bool {
        study % self.n == 0
    }
}

pub struct Distribution<H: Hypothesis + Hash + Eq>(HashMap<H, f64>);

impl<H: Hypothesis + Hash + Eq + Copy> Distribution<H> {
    pub fn new() -> Self {
        let backing = HashMap::<H, f64>::new();
        Distribution(backing)
    }

    pub fn ignorance_prior(hypotheses: Vec<H>) -> Self {
        let mut backing = HashMap::<H, f64>::new();
        let probability_each: f64 = 1.0/(hypotheses.len() as f64);
        for hypothesis in hypotheses.into_iter() {
            backing.insert(hypothesis, probability_each);
        }
        Distribution(backing)
    }

    fn backing(&self) -> &HashMap<H, f64> {
        &self.0
    }

    fn mut_backing(&mut self) -> &mut HashMap<H, f64> {
        &mut self.0
    }

    pub fn belief(&self, hypothesis: H) -> f64 {
        *self.backing().get(&hypothesis).unwrap_or(&0.0f64)
    }

    pub fn entropy(&self) -> f64 {
        self.backing().values().map(|p| -p * p.log2()).sum()
    }

    pub fn predict(&self, study: Study, verdict: bool) -> f64 {
        self.backing().iter()
            .filter(|hp| {
                let h = hp.0;
                h.predicts_the_property(study) == verdict
            })
            .map(|hp| {
                let p = hp.1;
                p
            }).sum()
    }

    pub fn updated(&self, study: Study, verdict: bool) -> Self {
        let normalization_factor = 1.0/self.predict(study, verdict);
        let rebacking_pairs = self.backing()
            .into_iter().filter(|hp| {
                let h = hp.0;
                h.predicts_the_property(study) == verdict
            }).map(|hp| {
                let (h, p) = hp;
                (*h, normalization_factor * p)
            });
        let rebacking = HashMap::from_iter(rebacking_pairs);
        Distribution(rebacking)
    }

    #[allow(unused_parens)]
    pub fn value_of_information(&self, study: Study) -> f64 {
        let given_the_property = self.updated(study, true);
        let given_the_negation = self.updated(study, false);
        let expected_entropy = (
            self.predict(study, true) * given_the_property.entropy() +
                self.predict(study, false) * given_the_negation.entropy());
        self.entropy() - expected_entropy
    }

    pub fn burning_question(&self, studies: Vec<Study>) -> Option<Study> {
        let mut top_value = NEG_INFINITY;
        let mut best_subject: Option<Study> = None;
        for study in &studies {
            let value = self.value_of_information(*study);
            if value > top_value {
                top_value = value;
                best_subject = Some(*study);
            }
        }
        best_subject
    }

}


fn factorize_on_system(k: u8) -> Vec<u8> {
    let stdout = Command::new("/usr/bin/factor").arg(format!("{}", k))
        .output().ok().expect("couldn't factor").stdout;
    let output = String::from_utf8(stdout).ok().expect("couldn't decode");
    let trimmed_output = output.trim();
    let output_parts = trimmed_output.split(": ");
    let output_result = output_parts.skip(1).next().unwrap();
    output_result.split(' ').map(
        |c| { c.parse::<u8>().ok().unwrap() }).collect::<Vec<_>>()
}


pub fn divisibility_priors() -> HashMap<DivisibilityHypothesis, f64> {
    // TODO ???

    HashMap::new()
}


#[cfg(test)]
mod tests {
    use super::{DivisibilityHypothesis, Distribution, factorize_on_system};

    #[test]
    fn concerning_factorizing_on_the_system() {
        assert_eq!(vec![2, 2, 3, 5], factorize_on_system(60));
        assert_eq!(vec![2, 5, 5], factorize_on_system(50));
    }

    #[test]
    fn concerning_updating_your_bayesian_distribution() {
        // Suppose we think the hypotheses "A number has the property
        // iff it is divisible by n" for n in {2, 3, 5, 7, 11} are all
        // equally likely.
        let hypotheses = vec![2, 3, 5, 7, 11].iter().map(
            |n| DivisibilityHypothesis::new(*n)).collect::<Vec<_>>();
        let prior = Distribution::ignorance_prior(hypotheses);

        // If we learn that 15 does not have the property, then the 3
        // and 5 hypotheses are eliminated, and instead we think that
        // n = 2, 7, or 11 are equally likely.
        let beliefs = prior.updated(15, false);

        let probability_n_is_two = beliefs.belief(
            DivisibilityHypothesis::new(2));
        let probability_n_is_seven = beliefs.belief(
            DivisibilityHypothesis::new(7));
        let probability_n_is_eleven = beliefs.belief(
            DivisibilityHypothesis::new(11));

        let one_third: f64 = 1./3.;
        assert_eq!(probability_n_is_two, one_third);
        assert_eq!(probability_n_is_seven, one_third);
        assert_eq!(probability_n_is_eleven, one_third);

        // And we think that 14 has a 2/3 chance of having the
        // property.
        assert_eq!(beliefs.predict(14, true), 2./3.);
    }

    #[test]
    fn concerning_what_to_ask_about() {
        let hypotheses = (1..100u8).map(
            |n| DivisibilityHypothesis::new(n)).collect::<Vec<_>>();
        let prior = Distribution::ignorance_prior(hypotheses);

        assert_eq!(prior.burning_question(vec![57, 60]).unwrap(), 60);
    }

}
