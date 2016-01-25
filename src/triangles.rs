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
