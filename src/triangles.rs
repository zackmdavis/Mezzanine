use std::fmt;
use std::slice;

use ansi_term;
use display;
use itertools::Itertools;
use rand::random;

/// We will classify our gloss'ry of shapes into compliance
/// We are magical methodical apes doing triangle science


const ONE_FORM: &'static str = "/\\ \n‾‾ ";

const TWO_FORM: &'static str = " /\\  \n/  \\ \n‾‾‾‾ ";

const THREE_FORM: &'static str = "  /\\   \n /  \\  \n/    \\ \n‾‾‾‾‾‾ ";


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Size {
    One,
    Two,
    Three
}

pub static SIZES: [Size; 3] = [Size::One, Size::Two, Size::Three];

impl Size {
    // Should this be fmt::Display::fmt?
    fn display(&self) -> String {
        match *self {
            Size::One => ONE_FORM.to_owned(),
            Size::Two => TWO_FORM.to_owned(),
            Size::Three => THREE_FORM.to_owned()
        }
    }

    pub fn iter() -> slice::Iter<'static, Self> {
        SIZES.iter()
    }

    pub fn sample() -> Self {
        let index = random::<usize>() % 3;
        SIZES[index]
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

pub static COLORS: [Color; 4] = [
    Color::Red, Color::Blue, Color::Green, Color::Yellow];

// TODO: `impl`ement fmt::Display for Color

impl Color {
    fn to_colorizer(&self) -> ansi_term::Colour {
        match *self {
            Color::Red => ansi_term::Colour::Red,
            Color::Blue => ansi_term::Colour::Blue,
            Color::Green => ansi_term::Colour::Green,
            Color::Yellow => ansi_term::Colour::Yellow
        }
    }

    pub fn iter() -> slice::Iter<'static, Self> {
        COLORS.iter()
    }

    pub fn sample() -> Self {
        let index = random::<usize>() % 4;
        COLORS[index]
    }
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Triangle {
    pub size: Size,
    pub color: Color
}

impl Triangle {
    pub fn new(color: Color, size: Size) -> Self {
        Triangle { color: color, size: size }
    }

    pub fn universe() -> Vec<Triangle> {
        Color::iter().cartesian_product(Size::iter())
            .map(|(&color, &size)| { Triangle::new(color, size) })
            .collect::<Vec<_>>()
    }

    pub fn sample() -> Self {
        Triangle::new(Color::sample(), Size::sample())
    }
}


impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let monochrome = self.size.display();
        let mut rendered = String::new();
        for line in monochrome.split('\n') {
            rendered.push_str(&format!("{}", self.color
                                       .to_colorizer().paint(line)));
            rendered.push('\n');
        }
        rendered.pop();
        write!(f, "{}", rendered)
    }
}


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Stack {
    triangles: Vec<Triangle>
}

macro_rules! stack {
    // XXX: `stack!()` triggers unused_mut lint. I don't think vec![] does
    // this—how does it manage?
    ($($triangle:expr),*) => {
        {
            let mut our_stack = Stack::new();
            $(our_stack.push($triangle);)*
            our_stack
        }
    }
}

impl Stack {
    pub fn new() -> Self {
        Stack { triangles: Vec::new() }
    }

    pub fn push(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    pub fn bounded_universe() -> Vec<Self> {
        // arbitrarily limit ourselves to at most two in a stack for now
        // pending sorely-needed clever data-structural advances
        let mut ghost_universe = Triangle::universe().iter()
            .map(|&t| Some(t)).collect::<Vec<_>>();
        ghost_universe.push(None);

        Triangle::universe().iter().cartesian_product(ghost_universe.iter())
            .map(|(&bottom, &top)| {
                let mut stack = stack!(bottom);
                if let Some(cap) = top {
                    stack.push(cap);
                }
                stack
            }).collect::<Vec<_>>()
    }

    pub fn sample() -> Self {
        // NOTE: A uniform distribution over heights is nonuniform over
        // possible stacks (because there are exponentially more taller
        // stacks), but that's OK, and we probably want a bias towards simpler
        // studies anyway
        let height = 1 + random::<usize>() % 4;
        let mut stack = Stack::new();
        for _ in 0..height {
            stack.push(Triangle::sample());
        }
        stack
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rendered = String::new();
        for triangle in &self.triangles {
            rendered = display::pack_blocks_vertically(
                &format!("{}", triangle),
                &rendered
            );
        }
        write!(f, "{}", rendered)
    }
}


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Study {
    stacks: Vec<Stack>
}

macro_rules! study {
    ($($stack:expr),*) => {
        {
            let mut our_study = Study::new();
            $(our_study.append($stack);)*
            our_study
        }
    }
}

impl fmt::Display for Study {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rendered = String::new();
        for stack in &self.stacks {
            rendered = display::pack_blocks_horizontally(
                &rendered,
                &format!("{}", stack),
            );
        }
        write!(f, "{}", rendered)
    }
}

impl Study {
    pub fn new() -> Self {
         Study { stacks: Vec::new() }
    }

    pub fn append(&mut self, stack: Stack) {
        self.stacks.push(stack);
    }

    pub fn color_count(&self, color: Color) -> usize {
        self.into_iter().filter(|t| { t.color == color }).count()
    }

    pub fn size_count(&self, size: Size) -> usize {
        self.into_iter().filter(|t| { t.size == size }).count()
    }

    pub fn bounded_universe() -> Vec<Self> {
        Stack::bounded_universe().iter()
            .cartesian_product(Stack::bounded_universe().iter())
            .map(|(left, right)| { study!(left.clone(), right.clone()) })
            .collect::<Vec<_>>()
    }

    pub fn sample() -> Self {
        // Again, a uniform distribution over stack count is nonuniform over
        // possible studies; we think it's fine!
        let breadth = 1 + random::<usize>() % 4;
        let mut study = Study::new();
        for _ in 0..breadth {
            study.append(Stack::sample());
        }
        study
    }
}

pub struct StudyIter<'a> {
    study: &'a Study,
    stack_index: usize,
    triangle_index: usize
}

impl<'a> Iterator for StudyIter<'a> {
    type Item = &'a Triangle;

    fn next(&mut self) -> Option<&'a Triangle> {
        match self.study.stacks.get(self.stack_index) {
            Some(stack) => {
                match stack.triangles.get(self.triangle_index) {
                    Some(triangle) => {
                        self.triangle_index += 1;
                        Some(triangle)
                    },
                    None => {
                        self.triangle_index = 0;
                        self.stack_index += 1;
                        self.next()
                    },
                }
            },
            None => None
        }
    }
}

impl<'a> IntoIterator for &'a Study {
    type Item = &'a Triangle;
    type IntoIter = StudyIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        StudyIter {
            study: self,
            triangle_index: 0,
            stack_index: 0
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concerning_study_iteration() {
        let study = study!(stack!(Triangle::new(Color::Blue, Size::Three),
                                  Triangle::new(Color::Red, Size::One)),
                           stack!(Triangle::new(Color::Green, Size::Two),
                                  Triangle::new(Color::Yellow, Size::One)));
        let mut triangle_count = 0;
        for triangle in &study {
            println!("{:?}", triangle);
            triangle_count += 1;
        }
        assert_eq!(4, triangle_count);  // they're all here
    }

    #[test]
    fn on_counting_colors() {
        let study = study!(stack!(Triangle::new(Color::Blue, Size::Three),
                                  Triangle::new(Color::Blue, Size::Two),
                                  Triangle::new(Color::Blue, Size::One)),
                           stack!(),
                           stack!(Triangle::new(Color::Yellow, Size::Three),
                                  Triangle::new(Color::Blue, Size::One)));
        assert_eq!(4, study.color_count(Color::Blue));
    }

    #[test]
    fn concerning_the_size_of_the_universe() {
        let heavenly_sphere = Triangle::universe().len();
        assert_eq!(12, heavenly_sphere);

        let hubble_bubble = Stack::bounded_universe().len();
        assert_eq!(156, hubble_bubble);
    }

}
