use inference::triangle::Hypothesis;
use triangles::Study;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct PipParityHypothesis {
    pub modulus: usize,
    pub remainder: usize,
}

impl PipParityHypothesis {
    pub fn new(modulus: usize, remainder: usize) -> Self {
        PipParityHypothesis { modulus: modulus, remainder: remainder }
    }
}

impl Hypothesis for PipParityHypothesis {
    fn predicts_the_property(&self, study: &Study) -> bool {
        study.pip_count() % self.modulus == self.remainder
    }

    fn description(&self) -> String {
        if self.modulus == 2 {
            match self.remainder {
                0 => "the total pip count is even".to_owned(),
                1 => "the total pip count is odd".to_owned(),
                _ => unreachable!()
            }
        } else {
            if self.remainder == 0 {
                format!("the total pip count is divisible by {}", self.modulus)
            } else {
                format!("the total pip count is {} modulo {}",
                        self.remainder, self.modulus)
            }
        }
    }
}
