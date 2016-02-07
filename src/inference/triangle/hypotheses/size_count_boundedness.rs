use inference::triangle::Hypothesis;
use triangles::{Size, Study};


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct SizeCountBoundednessHypothesis {
    pub size: Size,
    pub lower: Option<usize>,
    pub upper: Option<usize>
}


impl SizeCountBoundednessHypothesis {
    pub fn new(size: Size, lower: usize, upper: usize) -> Self {
        SizeCountBoundednessHypothesis {
            size: size,
            lower: Some(lower),
            upper: Some(upper)
        }
    }

    pub fn new_lower(size: Size, lower: usize) -> Self {
        SizeCountBoundednessHypothesis {
            size: size,
            lower: Some(lower),
            upper: None
        }
    }

    pub fn new_upper(size: Size, upper: usize) -> Self {
        SizeCountBoundednessHypothesis {
            size: size,
            lower: None,
            upper: Some(upper)
        }
    }
}


impl Hypothesis for SizeCountBoundednessHypothesis {
    fn predicts_the_property(&self, study: &Study) -> bool {
        let size_count = study.size_count(self.size);
        if let Some(min) = self.lower {
            if size_count < min {
                return false;
            }
        }
        if let Some(max) = self.upper {
            if size_count > max {
                return false;
            }
        }
        true
    }

    fn description(&self) -> String {
        let mut described: Vec<String> = vec![
            format!("the number of size-{:?} triangles", self.size)];
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
