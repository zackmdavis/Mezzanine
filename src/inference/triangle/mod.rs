#![allow(dead_code)]

pub mod hypotheses;

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::{Eq, Ordering};
use std::iter::FromIterator;

use ansi_term::Style;

use triangles::{Color, Size, Study};
use inference::triangle::hypotheses::BasicHypothesis;
use inference::triangle::hypotheses::JoinedHypothesis;

// TODO: bypass one layer of namespacing by reëxporting these from just
// inference::triangle::hypotheses?
use inference::triangle::hypotheses::color_count_boundedness::ColorCountBoundednessHypothesis;
use inference::triangle::hypotheses::size_count_boundedness::SizeCountBoundednessHypothesis;
use inference::triangle::hypotheses::pip_parity::PipParityHypothesis;


pub trait Hypothesis {
    fn predicts_the_property(&self, study: &Study) -> bool;
    fn description(&self) -> String;
}


pub fn our_basic_hypotheses() -> Vec<BasicHypothesis> {
    let mut hypotheses = Vec::new();
    for &color in Color::iter() {
        for exact in 1..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    ColorCountBoundednessHypothesis::new(
                        color, exact, exact)));
        }
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
        for exact in 1..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    SizeCountBoundednessHypothesis::new(
                        size, exact, exact)));
        }
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

    for modulus in 2..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    PipParityHypothesis::new(modulus)));
    }

    hypotheses
}


#[derive(Debug)]
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

    pub fn len(&self) -> usize {
        self.backing().len()
    }

    pub fn hypotheses(&self) -> Vec<&H> {
        self.backing().keys().collect::<Vec<_>>()
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

    pub fn predict(&self, study: &Study, verdict: bool) -> f64 {
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

    pub fn updated(&self, study: &Study, verdict: bool) -> Self {
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

    pub fn value_of_information(&self, study: &Study) -> f64 {
        let mut entropy = 0.;
        let mut probability_of_the_property = 0.;
        let mut probability_of_the_negation = 0.;

        for (&hypothesis, &probability) in self.backing().iter() {
            if hypothesis.predicts_the_property(study) {
                probability_of_the_property += probability;
            } else {
                probability_of_the_negation += probability;
            }
            entropy += -probability * probability.log2();
        }

        let property_normalization_factor = 1./probability_of_the_property;
        let negation_normalization_factor = 1./probability_of_the_negation;

        let mut entropy_given_the_property = 0.;
        let mut entropy_given_the_negation = 0.;

        for (&hypothesis, &probability) in self.backing().iter() {
            if hypothesis.predicts_the_property(study) {
                let p = property_normalization_factor * probability;
                entropy_given_the_property += -p * p.log2();
            } else {
                let p = negation_normalization_factor * probability;
                entropy_given_the_negation += -p * p.log2();
            }
        }

        let expected_entropy =
            probability_of_the_property * entropy_given_the_property +
            probability_of_the_negation * entropy_given_the_negation;

        entropy - expected_entropy
    }

    pub fn burning_question(&self, desired_bits: f64, sample_cap: usize)
                            -> Study {
        let mut study = Study::sample();
        let mut value = self.value_of_information(&study);
        let mut top_study = study.clone();
        let mut top_value = value;
        let mut samples = 1;
        loop {
            if value > top_value {
                top_value = value;
                top_study = study;
            }
            if (top_value > desired_bits) || (samples >= sample_cap) {
                break;
            }
            study = Study::sample();
            value = self.value_of_information(&study);
            samples += 1;
        }
        top_study
    }

    pub fn inspect(&self, n: usize) {
        let mut backing = self.backing().iter().collect::<Vec<_>>();
        backing.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal));
        let total_probability_mass: f64 = backing.iter()
            .map(|hp| { hp.1 }).sum();
        println!("Total probability mass: {:.6}", total_probability_mass);
        println!("Top {} hypotheses:", n);
        for &(&hypothesis, &probability) in backing.iter().take(n) {
            wrapln!(" * {}: {}", hypothesis.description(),
                    Style::new().bold().paint(&format!("{:.4}", probability)));
        }
    }
}


pub fn complexity_prior(basic_hypotheses: Vec<BasicHypothesis>)
                                -> Distribution<JoinedHypothesis> {
    let mut backing = HashMap::<JoinedHypothesis, f64>::new();
    let probability_each_basic = (2./3.)/(basic_hypotheses.len() as f64);
    let probability_each_joined = (1./3.)/(basic_hypotheses.len().pow(2) as f64);
    for &basic in &basic_hypotheses {
        backing.insert(JoinedHypothesis::full_stop(basic),
                       probability_each_basic);
    }
    for (i, &one_basic) in basic_hypotheses.iter().enumerate() {
        for (j, &another_basic) in basic_hypotheses.iter().enumerate() {
            if j <= i {
                continue;
            }
            if one_basic.obviates(&another_basic) {
                continue;
            }
            let conjunction = JoinedHypothesis::and(one_basic, another_basic);
            let disjunction = JoinedHypothesis::or(one_basic, another_basic);
            for &junction in &vec![conjunction, disjunction] {
                if junction.check_substantiality(50) {
                    backing.insert(junction, probability_each_joined);
                }
            }
        }
    }
    Distribution(backing)
}


#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;
    use triangles::{Color, Size, Stack, Study, Triangle};
    use inference::triangle::hypotheses::{BasicHypothesis, JoinedHypothesis};
    use inference::triangle::hypotheses::color_count_boundedness::ColorCountBoundednessHypothesis;

    #[test]
    fn concerning_updating_your_bayesian_distribution() {
        // Suppose we think the hypotheses "A study has the property if it has
        // at least 1 triangle of color C" for C in {Red, Green, Blue, Yellow}
        // are all equally likely, and that we aren't considering any other
        // alternatives.
        let hypotheses = vec![Color::Red, Color::Green,
                              Color::Blue, Color::Yellow].iter()
            .map(|&c| ColorCountBoundednessHypothesis::new_lower(c, 1))
            .collect::<Vec<_>>();
        let prior = Distribution::ignorance_prior(hypotheses);

        // If we learn that a study consisting of Red and Yellow triangles does
        // not have the property, then we think that C = Green or Blue are
        // equally likely.
        let beliefs = prior.updated(
            &study!(stack!(Triangle::new(Color::Red, Size::One),
                           Triangle::new(Color::Yellow, Size::One))), false);

        let probability_c_is_blue = beliefs.belief(
            ColorCountBoundednessHypothesis::new_lower(Color::Blue, 1));
        let probability_c_is_green = beliefs.belief(
            ColorCountBoundednessHypothesis::new_lower(Color::Green, 1));

        assert_eq!(probability_c_is_blue, 0.5);
        assert_eq!(probability_c_is_green, 0.5);
    }

    #[test]
    fn concerning_soundness_of_our_complexity_penalty() {
        // ⎲ ∞
        // ⎳ i=1  1/2^i = 1
        //
        // So ... I want to give conjunctions and disjunctions a lower prior
        // probability, but I'm running into the same philosophical difficulty
        // that I ran into when I was first sketching out the number game, as
        // accounted in the README: if the true meaning of the complexity
        // penalty is that the hypothesis "A" gets to sum over the unspecified
        // details borne by the more complicated hypotheses "A ∧ B" and "A ∧
        // C", then it's not clear how this insight translates to this setting,
        // where we want to represent our knowledge as a collection of mutually
        // exclusive hypotheses: we don't care about being able to refine a
        // true-but-vague theory to a true-but-more-precise theory; we want to
        // say that the precise theory is true and that all others are false.
        //
        // Probably the real answer is that this game just isn't very
        // philosophically interesting: we should have a complexity penalty to
        // exactly the extent that we think the human property-specifiers the
        // engine will face are going to choose disjunctions or disjunctions
        // less often than a uniform sample over distinct hypotheses would.
        let basics = vec![
            BasicHypothesis::from(
                ColorCountBoundednessHypothesis::new_lower(Color::Blue, 1)),
            BasicHypothesis::from(
                ColorCountBoundednessHypothesis::new_lower(Color::Red, 1))
        ];
        let distribution = complexity_prior(basics);

        assert_eq!(1./3.,
                   distribution.belief(JoinedHypothesis::full_stop(
                       BasicHypothesis::from(
                           ColorCountBoundednessHypothesis::new_lower(
                               Color::Blue, 1)))));
        assert_eq!(1./12.,
                   distribution.belief(JoinedHypothesis::and(
                       BasicHypothesis::from(
                           ColorCountBoundednessHypothesis::new_lower(
                               Color::Blue, 1)),
                       BasicHypothesis::from(
                           ColorCountBoundednessHypothesis::new_lower(
                               Color::Red, 1)))));
    }

    #[bench]
    fn concerning_the_expense_of_updating(bencher: &mut Bencher) {
        let distribution = complexity_prior(our_basic_hypotheses());
        bencher.iter(|| {
            distribution.updated(&Study::sample(), true);
        });
    }

    #[bench]
    fn concerning_the_expense_of_computing_entropy(bencher: &mut Bencher) {
        let distribution = complexity_prior(our_basic_hypotheses());
        bencher.iter(|| {
            distribution.entropy();
        });
    }

    #[bench]
    fn concerning_the_expense_of_prediction(bencher: &mut Bencher) {
        let distribution = complexity_prior(our_basic_hypotheses());
        bencher.iter(|| {
            distribution.predict(&Study::sample(), true);
        });
    }

    #[bench]
    fn concerning_the_expense_of_the_value(bencher: &mut Bencher) {
        let distribution = complexity_prior(our_basic_hypotheses());
        bencher.iter(|| {
            distribution.value_of_information(&Study::sample());
        });
    }

}
