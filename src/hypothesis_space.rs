#![allow(dead_code)]

use std::collections::HashMap;
use std::process::Command;
use std::hash::Hash;
use std::cmp::Eq;

static PRIMES: [u8; 25] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
                           43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

type Study = u8;

trait Hypothesis {
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

struct Distribution<H: Hypothesis + Hash + Eq>(HashMap<H, f64>);

impl<H: Hypothesis + Hash + Eq> Distribution<H> {
    pub fn map(&self) -> &HashMap<H, f64> {
        &self.0
    }

    pub fn entropy(&self) -> f64 {
        self.map().values().map(|p| -p * p.log2()).sum()
    }

    #[cfg(TODO_make_this_compile)]
    pub fn predict(&self, study: Study, verdict: bool) -> f64 {
        self.map().iter()
            .filter(|h, p| h.predicts_the_property(study) == verdict)
            .map(|_h, p| p).sum()
    }

    #[cfg(TODO)]
    pub fn update(&self, study: Study, verdict: bool) -> Self {}
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
    use super::factorize_on_system;

    #[test]
    fn concerning_factorizing_on_the_system() {
        assert_eq!(vec![2, 2, 3, 5], factorize_on_system(60));
        assert_eq!(vec![2, 5, 5], factorize_on_system(50));
    }

}
