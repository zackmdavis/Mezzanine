pub mod color_count_boundedness;
pub mod size_count_boundedness;
pub mod groundedness_count_boundedness;
pub mod pip_boundedness;
pub mod pip_parity;
pub mod standard_basics;

use inference::triangle::Hypothesis;
use inference::triangle::hypotheses::color_count_boundedness::ColorCountBoundednessHypothesis;
use inference::triangle::hypotheses::size_count_boundedness::SizeCountBoundednessHypothesis;
use inference::triangle::hypotheses::groundedness_count_boundedness::GroundednessCountBoundednessHypothesis;
use inference::triangle::hypotheses::pip_boundedness::PipBoundednessHypothesis;
use inference::triangle::hypotheses::pip_parity::PipParityHypothesis;

use triangles::Study;


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum BasicHypothesis {
    ColorCountBoundedness(ColorCountBoundednessHypothesis),
    SizeCountBoundedness(SizeCountBoundednessHypothesis),
    GroundednessCountBoundedness(GroundednessCountBoundednessHypothesis),
    PipBoundedness(PipBoundednessHypothesis),
    PipParity(PipParityHypothesis),
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

impl From<GroundednessCountBoundednessHypothesis> for BasicHypothesis {
    fn from(h: GroundednessCountBoundednessHypothesis) -> Self {
        BasicHypothesis::GroundednessCountBoundedness(h)
    }
}

impl From<PipBoundednessHypothesis> for BasicHypothesis {
    fn from(h: PipBoundednessHypothesis) -> Self {
        BasicHypothesis::PipBoundedness(h)
    }
}

impl From<PipParityHypothesis> for BasicHypothesis {
    fn from(h: PipParityHypothesis) -> Self {
        BasicHypothesis::PipParity(h)
    }
}


impl BasicHypothesis {
    pub fn obviates(&self, other: &BasicHypothesis) -> bool {
        match *self {
            BasicHypothesis::ColorCountBoundedness(h1) => match *other {
                BasicHypothesis::ColorCountBoundedness(h2) => {
                    h1.color == h2.color
                },
                BasicHypothesis::PipBoundedness(_h2) => true,
                BasicHypothesis::PipParity(_h2) => true,
                _ => false
            },
            BasicHypothesis::SizeCountBoundedness(h1) => match *other {
                BasicHypothesis::SizeCountBoundedness(h2) => {
                    h1.size == h2.size
                },
                _ => false
            },
            BasicHypothesis::GroundednessCountBoundedness(h1) => match *other {
                BasicHypothesis::GroundednessCountBoundedness(h2) => {
                    h1.grounded == h2.grounded
                },
                _ => false
            },
            BasicHypothesis::PipBoundedness(_h1) => match *other {
                BasicHypothesis::SizeCountBoundedness(_h2) => true,
                BasicHypothesis::PipBoundedness(_h2) => true,
                BasicHypothesis::PipParity(_h2) => true,
                _ => false
            },
            BasicHypothesis::PipParity(_h1) => match *other {
                BasicHypothesis::SizeCountBoundedness(_h2) => true,
                BasicHypothesis::PipBoundedness(_h2) => true,
                BasicHypothesis::PipParity(_h2) => true,
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
                h.predicts_the_property(study),
            BasicHypothesis::GroundednessCountBoundedness(h) =>
                h.predicts_the_property(study),
            BasicHypothesis::PipBoundedness(h) =>
                h.predicts_the_property(study),
            BasicHypothesis::PipParity(h) =>
                h.predicts_the_property(study),
        }
    }
    fn description(&self) -> String {
        match *self {
            BasicHypothesis::ColorCountBoundedness(h) => h.description(),
            BasicHypothesis::SizeCountBoundedness(h) => h.description(),
            BasicHypothesis::GroundednessCountBoundedness(h) => h.description(),
            BasicHypothesis::PipBoundedness(h) => h.description(),
            BasicHypothesis::PipParity(h) => h.description(),
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
