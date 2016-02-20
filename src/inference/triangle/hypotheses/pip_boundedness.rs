use inference::triangle::Hypothesis;
use triangles::Study;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct PipBoundednessHypothesis {
    pub lower: Option<usize>,
    pub upper: Option<usize>
}


impl PipBoundednessHypothesis {
    pub fn new(lower: usize, upper: usize) -> Self {
        PipBoundednessHypothesis {
            lower: Some(lower),
            upper: Some(upper)
        }
    }

    pub fn exactly(count: usize) -> Self {
        PipBoundednessHypothesis {
            lower: Some(count),
            upper: Some(count)
        }
    }

    pub fn at_least(lower: usize) -> Self {
        PipBoundednessHypothesis {
            lower: Some(lower),
            upper: None
        }
    }

    pub fn at_most(upper: usize) -> Self {
        PipBoundednessHypothesis {
            lower: None,
            upper: Some(upper)
        }
    }
}


impl Hypothesis for PipBoundednessHypothesis {
    fn predicts_the_property(&self, study: &Study) -> bool {
        let pip_count = study.pip_count();
        if let Some(min) = self.lower {
            if pip_count < min {
                return false;
            }
        }
        if let Some(max) = self.upper {
            if pip_count > max {
                return false;
            }
        }
        true
    }

    fn description(&self) -> String {
        let mut described: Vec<String> = vec!["the number of pips".to_owned()];

        // exceptional case for exactness
        if self.lower.is_some() && self.upper.is_some() &&
            self.lower.unwrap() == self.upper.unwrap() {
                described.push(format!("is exactly {}", self.lower.unwrap()));
                return described.join(" ");
        }

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
