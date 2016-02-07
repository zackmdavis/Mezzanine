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
