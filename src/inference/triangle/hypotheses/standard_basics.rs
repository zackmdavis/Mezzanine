use triangles::{Color, Size};
use inference::triangle::hypotheses::BasicHypothesis;

use inference::triangle::hypotheses::color_count_boundedness::ColorCountBoundednessHypothesis;
use inference::triangle::hypotheses::size_count_boundedness::SizeCountBoundednessHypothesis;
use inference::triangle::hypotheses::groundedness_count_boundedness::GroundednessCountBoundednessHypothesis;
use inference::triangle::hypotheses::pip_boundedness::PipBoundednessHypothesis;
use inference::triangle::hypotheses::pip_parity::PipParityHypothesis;

pub fn standard_basic_hypotheses() -> Vec<BasicHypothesis> {
    let mut hypotheses = Vec::new();

    for &color in Color::iter() {
        for exact in 1..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    ColorCountBoundednessHypothesis::new(
                        color, exact, exact)));
        }
        for lower in 1..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    ColorCountBoundednessHypothesis::new_lower(
                        color, lower)));
        }
        for upper in 0..3 {
            hypotheses.push(
                BasicHypothesis::from(
                    ColorCountBoundednessHypothesis::new_upper(
                        color, upper)));
        }
    }

    for &size in Size::iter() {
        for exact in 1..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    SizeCountBoundednessHypothesis::new(
                        size, exact, exact)));
        }
        for lower in 1..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    SizeCountBoundednessHypothesis::new_lower(
                        size, lower)));
        }
        for upper in 0..3 {
            hypotheses.push(
                BasicHypothesis::from(
                    SizeCountBoundednessHypothesis::new_upper(
                        size, upper)));
        }
    }

    // all studies have at least one grounded triangle, so the "at least 1
    // grounded" and "none grounded" hypotheses are uninteresting (trivial and
    // vacuous, respectively), but their analogues concerning ungrounded
    // triangles are interesting
    hypotheses.push(
        BasicHypothesis::from(
            GroundednessCountBoundednessHypothesis::new_lower(false, 1)));
    hypotheses.push(
        BasicHypothesis::from(
            GroundednessCountBoundednessHypothesis::new_upper(false, 0)));

    for groundedness in vec![true, false] {
        for exact in 1..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    GroundednessCountBoundednessHypothesis::new(
                        groundedness, exact, exact)));
        }

        for lower in 2..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    GroundednessCountBoundednessHypothesis::new_lower(
                        groundedness, lower)));
        }

        for upper in 1..3 {
            hypotheses.push(
                BasicHypothesis::from(
                    GroundednessCountBoundednessHypothesis::new_upper(
                        groundedness, upper)));
        }
    }

    for pip_count in 4..16 {
        hypotheses.push(
            BasicHypothesis::from(
                PipBoundednessHypothesis::exactly(pip_count)));
        hypotheses.push(
            BasicHypothesis::from(
                PipBoundednessHypothesis::at_least(pip_count)));
        hypotheses.push(
            BasicHypothesis::from(
                PipBoundednessHypothesis::at_most(pip_count)));
    }

    for modulus in 2..6 {
            hypotheses.push(
                BasicHypothesis::from(
                    PipParityHypothesis::new(modulus, 0)));
    }
    hypotheses.push(BasicHypothesis::from(PipParityHypothesis::new(2, 1)));

    hypotheses
}
