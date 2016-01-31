#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::iter::FromIterator;
use std::f64::NEG_INFINITY;


use triangles::{Color, Study};


pub trait Hypothesis {
    fn predicts_the_property(&self, study: &Study) -> bool;
    fn description(&self) -> String;
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct ColorBoundednessHypothesis {
    pub color: Color,
    pub lower: Option<usize>,
    pub upper: Option<usize>
}

impl ColorBoundednessHypothesis {
    pub fn new(color: Color, lower: usize, upper: usize) -> Self {
        ColorBoundednessHypothesis {
            color: color,
            lower: Some(lower),
            upper: Some(upper)
        }
    }

    pub fn new_lower(color: Color, lower: usize) -> Self {
        ColorBoundednessHypothesis {
            color: color,
            lower: Some(lower),
            upper: None
        }
    }

    pub fn new_upper(color: Color, upper: usize) -> Self {
        ColorBoundednessHypothesis {
            color: color,
            lower: None,
            upper: Some(upper)
        }
    }
}

impl Hypothesis for ColorBoundednessHypothesis {
    fn predicts_the_property(&self, study: &Study) -> bool {
        let color_count = study.color_count(self.color);
        if let Some(min) = self.lower {
            if color_count < min {
                return false;
            }
        }
        if let Some(max) = self.upper {
            if color_count > max {
                return false;
            }
        }
        true
    }

    fn description(&self) -> String {
        let mut described: Vec<String> = vec![
            format!("the number of {:?} triangles", self.color)];
        if let Some(min) = self.lower {
            described.push(format!("is not less than {}", min));
        }
        if self.lower.is_some() && self.upper.is_some() {
            described.push("and".to_owned());
        }
        if let Some(max) = self.upper {
            described.push(format!("is not greater than {}", max));
        }
        described.join(" ")
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
        let given_the_property = self.updated(study, true);
        let given_the_negation = self.updated(study, false);
        let expected_entropy =
            self.predict(study, true) * given_the_property.entropy() +
            self.predict(study, false) * given_the_negation.entropy();
        self.entropy() - expected_entropy
    }

    pub fn burning_question<'a>(&'a self, studies: &'a [Study])
                                -> Option<Study> {
        let mut top_value = NEG_INFINITY;
        let mut best_subject = None;
        for study in studies {
            let value = self.value_of_information(&study);
            if value > top_value {
                top_value = value;
                best_subject = Some(study.clone());
            }
        }
        best_subject
    }

}


#[cfg(test)]
mod tests {
    use triangles::{Color, Size, Stack, Study, Triangle};
    use super::*;

    #[test]
    fn concerning_updating_your_bayesian_distribution() {
        // Suppose we think the hypotheses "A study has the property if it has
        // at least 1 triangle of color C" for C in {Red, Green, Blue, Yellow}
        // are all equally likely, and that we aren't considering any other
        // alternatives.
        let hypotheses = vec![Color::Red, Color::Green,
                              Color::Blue, Color::Yellow].iter()
            .map(|&c| ColorBoundednessHypothesis::new_lower(c, 1))
            .collect::<Vec<_>>();
        let prior = Distribution::ignorance_prior(hypotheses);

        // If we learn that a study consisting of Red and Yellow triangles does
        // not have the property, then we think that C = Green or Blue are
        // equally likely.
        let beliefs = prior.updated(
            &study!(stack!(Triangle::new(Color::Red, Size::One),
                           Triangle::new(Color::Yellow, Size::One))), false);

        let probability_c_is_blue = beliefs.belief(
            ColorBoundednessHypothesis::new_lower(Color::Blue, 1));
        let probability_c_is_green = beliefs.belief(
            ColorBoundednessHypothesis::new_lower(Color::Green, 1));

        assert_eq!(probability_c_is_blue, 0.5);
        assert_eq!(probability_c_is_green, 0.5);
    }

}
