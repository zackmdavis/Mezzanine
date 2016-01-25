use std::fmt;

use ansi_term;
use display;

/// We will classify our gloss'ry of shapes into compliance
/// We are magical methodical apes doing triangle science


const ONE_FORM: &'static str = "/\\\n‾‾";

const TWO_FORM: &'static str = " /\\\n/  \\\n‾‾‾‾";

const THREE_FORM: &'static str = "  /\\\n /  \\\n/    \\\n‾‾‾‾‾‾";


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


pub enum Color {
    Red,
    #[allow(dead_code)]
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
        write!(f, "{}", self.color.to_colorizer().paint(&self.size.display()))
    }
}


pub struct TriangleStack {
    triangles: Vec<Triangle>
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
    // XXX: there's a bug somewhere that leaves a disasterous extra space for
    // size-three triangles in the stack
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rendered = String::new();
        for i in (0..self.triangles.len()).rev() {
            rendered = display::pack_blocks_vertically(
                &rendered,
                &format!("{}", self.triangles[i])
            );
        }
        write!(f, "{}", rendered)
    }
}


pub struct TriangleStudy {
    triangles: Vec<TriangleStack>
}
