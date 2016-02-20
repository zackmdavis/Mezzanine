use triangles::{Color, Size};
use inference::triangle::hypotheses::BasicHypothesis;

// TODO: bypass one layer of namespacing by reëxporting these from just
// inference::triangle::hypotheses?
use inference::triangle::hypotheses::color_count_boundedness::ColorCountBoundednessHypothesis;
use inference::triangle::hypotheses::size_count_boundedness::SizeCountBoundednessHypothesis;
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

    for modulus in 2..4 {
            hypotheses.push(
                BasicHypothesis::from(
                    PipParityHypothesis::new(modulus, 0)));
    }
    hypotheses.push(BasicHypothesis::from(PipParityHypothesis::new(2, 1)));

    hypotheses
}
