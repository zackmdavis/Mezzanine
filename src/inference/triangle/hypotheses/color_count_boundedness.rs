use inference::triangle::Hypothesis;
use triangles::{Color, Study};


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct ColorCountBoundednessHypothesis {
    pub color: Color,
    pub lower: Option<usize>,
    pub upper: Option<usize>
}

impl ColorCountBoundednessHypothesis {
    pub fn new(color: Color, lower: usize, upper: usize) -> Self {
        ColorCountBoundednessHypothesis {
            color: color,
            lower: Some(lower),
            upper: Some(upper)
        }
    }

    pub fn new_lower(color: Color, lower: usize) -> Self {
        ColorCountBoundednessHypothesis {
            color: color,
            lower: Some(lower),
            upper: None
        }
    }

    pub fn new_upper(color: Color, upper: usize) -> Self {
        ColorCountBoundednessHypothesis {
            color: color,
            lower: None,
            upper: Some(upper)
        }
    }
}


impl Hypothesis for ColorCountBoundednessHypothesis {
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
