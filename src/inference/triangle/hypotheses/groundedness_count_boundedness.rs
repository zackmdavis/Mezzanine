use inference::triangle::Hypothesis;
use triangles::Study;


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct GroundednessCountBoundednessHypothesis {
    pub grounded: bool,
    pub lower: Option<usize>,
    pub upper: Option<usize>
}


impl GroundednessCountBoundednessHypothesis {
    pub fn new(grounded: bool, lower: usize, upper: usize) -> Self {
        GroundednessCountBoundednessHypothesis {
            grounded: grounded,
            lower: Some(lower),
            upper: Some(upper)
        }
    }

    pub fn new_lower(grounded: bool, lower: usize) -> Self {
        GroundednessCountBoundednessHypothesis {
            grounded: grounded,
            lower: Some(lower),
            upper: None
        }
    }

    pub fn new_upper(grounded: bool, upper: usize) -> Self {
        GroundednessCountBoundednessHypothesis {
            grounded: grounded,
            lower: None,
            upper: Some(upper)
        }
    }
}


impl Hypothesis for GroundednessCountBoundednessHypothesis {
    fn predicts_the_property(&self, study: &Study) -> bool {
        let groundedness_count = study.groundedness_count(self.grounded);
        if let Some(min) = self.lower {
            if groundedness_count < min {
                return false;
            }
        }
        if let Some(max) = self.upper {
            if groundedness_count > max {
                return false;
            }
        }
        true
    }

    fn description(&self) -> String {
        let mut described: Vec<String> = vec![
            format!("the number of {}grounded triangles",
                    if !self.grounded { "un" } else { "" })];
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
