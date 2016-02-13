use inference::triangle::Hypothesis;
use triangles::Study;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct PipParityHypothesis {
    pub modulus: usize
}

impl PipParityHypothesis {
    pub fn new(modulus: usize) -> Self {
        PipParityHypothesis { modulus: modulus }
    }
}

impl Hypothesis for PipParityHypothesis {
    fn predicts_the_property(&self, study: &Study) -> bool {
        study.pip_count() % self.modulus == 0
    }

    fn description(&self) -> String {
        format!("the total pip count is divisible by {}", self.modulus)
    }
}
