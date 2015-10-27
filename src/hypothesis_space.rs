#![allow(dead_code)]

struct DivisibilityHypothesis {
    n: u8
}

impl DivisibilityHypothesis {
    pub fn predicts_the_property(&self, subject: u8) -> bool {
        subject % self.n == 0
    }
}
