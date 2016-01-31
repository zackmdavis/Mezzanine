use std::fmt;

use ansi_term;
use display;

/// We will classify our gloss'ry of shapes into compliance
/// We are magical methodical apes doing triangle science


const ONE_FORM: &'static str = "/\\\n‾‾";

const TWO_FORM: &'static str = " /\\\n/  \\\n‾‾‾‾";

const THREE_FORM: &'static str = "  /\\\n /  \\\n/    \\\n‾‾‾‾‾‾";


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Size {
    One,
    Two,
    Three
}

impl Size {
    fn display(&self) -> String {
        match *self {
            Size::One => ONE_FORM.to_owned(),
            Size::Two => TWO_FORM.to_owned(),
            Size::Three => THREE_FORM.to_owned()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Color {
    fn to_colorizer(&self) -> ansi_term::Colour {
        match *self {
            Color::Red => ansi_term::Colour::Red,
            Color::Blue => ansi_term::Colour::Blue,
            Color::Green => ansi_term::Colour::Green,
            Color::Yellow => ansi_term::Colour::Yellow
        }
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


pub struct TriangleStack {
    triangles: Vec<Triangle>
}


macro_rules! stack {
    ($($triangle:expr),*) => {
        {
            let mut our_stack = TriangleStack::new();
            $(our_stack.push($triangle);)*
            our_stack
        }
    }
}


impl TriangleStack {
    pub fn new() -> Self {
        TriangleStack { triangles: Vec::new() }
    }

    pub fn push(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
}


impl fmt::Display for TriangleStack {
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


pub struct TriangleStudy {
    stacks: Vec<TriangleStack>
}

macro_rules! study {
    ($($stack:expr),*) => {
        {
            let mut our_study = TriangleStudy::new();
            $(our_study.append($stack);)*
            our_study
        }
    }
}

impl fmt::Display for TriangleStudy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rendered = String::new();
        for stack in &self.stacks {
            // XXX TODO: leave a column of air between the stacks
            rendered = display::pack_blocks_horizontally(
                &rendered,
                &format!("{}", stack),
            );
        }
        write!(f, "{}", rendered)
    }
}

impl TriangleStudy {
    pub fn new() -> Self {
         TriangleStudy { stacks: Vec::new() }
    }

    pub fn append(&mut self, stack: TriangleStack) {
        self.stacks.push(stack);
    }
}

pub struct TriangleStudyIter<'a> {
    study: &'a TriangleStudy,
    stack_index: usize,
    triangle_index: usize
}

impl<'a> Iterator for TriangleStudyIter<'a> {
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

impl<'a> IntoIterator for &'a TriangleStudy {
    type Item = &'a Triangle;
    type IntoIter = TriangleStudyIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TriangleStudyIter {
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

}
