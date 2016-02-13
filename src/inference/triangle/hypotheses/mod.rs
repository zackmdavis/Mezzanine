pub mod color_count_boundedness;
pub mod size_count_boundedness;

use inference::triangle::Hypothesis;
use inference::triangle::hypotheses::color_count_boundedness::ColorCountBoundednessHypothesis;
use inference::triangle::hypotheses::size_count_boundedness::SizeCountBoundednessHypothesis;
use triangles::Study;


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum BasicHypothesis {
    ColorCountBoundedness(ColorCountBoundednessHypothesis),
    SizeCountBoundedness(SizeCountBoundednessHypothesis)
}

impl From<ColorCountBoundednessHypothesis> for BasicHypothesis {
    fn from(h: ColorCountBoundednessHypothesis) -> Self {
        BasicHypothesis::ColorCountBoundedness(h)
    }
}

impl From<SizeCountBoundednessHypothesis> for BasicHypothesis {
    fn from(h: SizeCountBoundednessHypothesis) -> Self {
        BasicHypothesis::SizeCountBoundedness(h)
    }
}

impl BasicHypothesis {
    pub fn is_same_type(&self, other: &BasicHypothesis) -> bool {
        match *self {
            BasicHypothesis::ColorCountBoundedness(_h) => match *other {
                BasicHypothesis::ColorCountBoundedness(_h) => true,
                _ => false
            },
            BasicHypothesis::SizeCountBoundedness(_h) => match *other {
                BasicHypothesis::SizeCountBoundedness(_h) => true,
                _ => false
            }
        }
    }
}


impl Hypothesis for BasicHypothesis {
    // XXX: the amount of boilerplate quasi-duplicated code in this project is
    // out of control; it remains to be seen how much of it can be gotten under
    // control with macros and a better understanding of how to use Rust
    fn predicts_the_property(&self, study: &Study) -> bool {
        match *self {
            BasicHypothesis::ColorCountBoundedness(h) =>
                h.predicts_the_property(study),
            BasicHypothesis::SizeCountBoundedness(h) =>
                h.predicts_the_property(study)
        }
    }
    fn description(&self) -> String {
        match *self {
            BasicHypothesis::ColorCountBoundedness(h) => h.description(),
            BasicHypothesis::SizeCountBoundedness(h) => h.description()
        }
    }
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Remainder {
    And(BasicHypothesis),
    Or(BasicHypothesis),
    FullStop
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct JoinedHypothesis {
    pub proposition: BasicHypothesis,
    pub remainder: Remainder
}

impl JoinedHypothesis {
    pub fn full_stop(hypothesis: BasicHypothesis) -> Self {
        JoinedHypothesis { proposition: hypothesis,
                           remainder: Remainder::FullStop }
    }

    pub fn and(first_conjunct: BasicHypothesis,
               second_conjunct: BasicHypothesis) -> Self {
        JoinedHypothesis { proposition: first_conjunct,
                           remainder: Remainder::And(second_conjunct) }
    }

    pub fn or(first_disjunct: BasicHypothesis,
              second_disjunct: BasicHypothesis) -> Self {
        JoinedHypothesis { proposition: first_disjunct,
                           remainder: Remainder::Or(second_disjunct) }
    }

    pub fn check_substantiality(&self, sample_cap: usize) -> bool {
        let mut falsifiable = false;
        let mut confirmable = false;
        for _ in 0..sample_cap {
            let study = Study::sample();
            if self.predicts_the_property(&study) {
                confirmable = true;
            } else {
                falsifiable = true;
            }
            if falsifiable && confirmable {
                return true;
            }
        }
        false
    }
}


impl Hypothesis for JoinedHypothesis {
    fn predicts_the_property(&self, study: &Study) -> bool {
        match self.remainder {
            Remainder::And(ref conjunct) => {
                self.proposition.predicts_the_property(study) &&
                    conjunct.predicts_the_property(study)
            },
            Remainder::Or(ref disjunct) => {
                self.proposition.predicts_the_property(study) ||
                    disjunct.predicts_the_property(study)
            },
            Remainder::FullStop => {
                self.proposition.predicts_the_property(study)
            }
        }
    }

    fn description(&self) -> String {
        match self.remainder {
            Remainder::And(ref conjunct) => {
                format!("{} and {}", self.proposition.description(),
                        conjunct.description())
            },
            Remainder::Or(ref disjunct) => {
                format!("{} or {}", self.proposition.description(),
                        disjunct.description())
            },
            Remainder::FullStop => {
                self.proposition.description()
            }
        }
    }
}
