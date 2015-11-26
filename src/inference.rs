#![allow(dead_code)]

use std::collections::HashMap;
use std::process::Command;
use std::hash::Hash;
use std::cmp::Eq;
use std::iter::FromIterator;
use std::f64::NEG_INFINITY;


pub type Study = u16;

pub trait Hypothesis {
    fn predicts_the_property(&self, study: Study) -> bool;
    fn description(&self) -> String;
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct DivisibilityHypothesis {
    pub n: u16
}

impl DivisibilityHypothesis {
    pub fn new(n: u16) -> Self {
        DivisibilityHypothesis { n: n }
    }
}

impl Hypothesis for DivisibilityHypothesis {
    fn predicts_the_property(&self, study: Study) -> bool {
        study % self.n == 0
    }

    fn description(&self) -> String {
        format!("it is divisible by {}", self.n)
    }
}


pub struct BoundednessHypothesis {
    pub lower: Option<u16>,
    pub upper: Option<u16>
}

impl BoundednessHypothesis {
    pub fn new(lower: Study, upper: Study) -> Self {
        BoundednessHypothesis { lower: Some(lower), upper: Some(upper) }
    }
    pub fn new_lower(lower: Study) -> Self {
        BoundednessHypothesis { lower: Some(lower), upper: None }
    }
    pub fn new_upper(upper: Study) -> Self {
        BoundednessHypothesis { lower: None, upper: Some(upper) }
    }
}

impl Hypothesis for BoundednessHypothesis {
    fn predicts_the_property(&self, study: Study) -> bool {
        if let Some(min) = self.lower {
            if study < min {
                return false;
            }
        }
        if let Some(max) = self.upper {
            if study > max {
                return false;
            }
        }
        true
    }

    fn description(&self) -> String {
        let mut described: Vec<String> = Vec::new();
        if let Some(min) = self.lower {
            described.push(format!("it is not less than {}", min));
        }
        if self.lower.is_some() && self.upper.is_some() {
            described.push("and".to_owned());
        }
        if let Some(max) = self.upper {
            described.push(format!("it is not greater than {}", max));
        }
        described.join(" ")
    }
}


pub struct ConjunctiveHypothesis<H: Hypothesis, I: Hypothesis> {
    pub this: Box<H>,
    pub that: Box<I>
}

impl<H: Hypothesis, I: Hypothesis> Hypothesis for ConjunctiveHypothesis<H, I> {
    fn predicts_the_property(&self, study: Study) -> bool {
        self.this.predicts_the_property(study) &&
            self.that.predicts_the_property(study)
    }

    fn description(&self) -> String {
        format!("{} and {}", self.this.description(), self.that.description())
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

    pub fn completely_certain(&self) -> Option<H> {
        if self.backing().len() != 1 {
            None
        } else {
            Some(*self.backing().keys().nth(0).expect("should have one entry"))
        }
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

    pub fn value_of_information(&self, study: Study) -> f64 {
        let given_the_property = self.updated(study, true);
        let given_the_negation = self.updated(study, false);
        let expected_entropy =
            self.predict(study, true) * given_the_property.entropy() +
            self.predict(study, false) * given_the_negation.entropy();
        self.entropy() - expected_entropy
    }

    pub fn burning_question(&self, studies: Vec<Study>) -> Option<Study> {
        // CONSIDER: maybe this should just return a Study, and
        // panic if the distribution is empty?
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


fn factorize_on_system(k: u16) -> Vec<u16> {
    let stdout = Command::new("/usr/bin/factor").arg(format!("{}", k))
        .output().ok().expect("couldn't factor").stdout;
    let output = String::from_utf8(stdout).ok().expect("couldn't decode");
    let trimmed_output = output.trim();
    let output_parts = trimmed_output.split(": ");
    let output_result = output_parts.skip(1).next().unwrap();
    output_result.split(' ').map(
        |c| { c.parse::<u16>().ok().unwrap() }).collect::<Vec<_>>()
}


#[cfg(test)]
mod tests {
    use super::{BoundednessHypothesis, DivisibilityHypothesis,
                Distribution, Hypothesis,
                factorize_on_system};

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
        let hypotheses = (1..100u16).map(
            |n| DivisibilityHypothesis::new(n)).collect::<Vec<_>>();
        let prior = Distribution::ignorance_prior(hypotheses);

        assert_eq!(prior.burning_question(vec![57, 60]).unwrap(), 60);
    }

    #[test]
    fn concerning_making_a_heterogenous_hypothesis_vector() {
        let mut hypotheses:  Vec<Box<Hypothesis>> = Vec::new();
        hypotheses.push(Box::new(DivisibilityHypothesis::new(2)));
        hypotheses.push(Box::new(BoundednessHypothesis::new_lower(2)));
    }

}
