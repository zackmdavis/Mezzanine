mod number;
mod triangle;


use argparse::{ArgumentParser, Store};

use triangles::{Triangle, Stack, Study, Color, Size};


pub fn decorative_display_header() {
    let mascot_study = study!(stack!(Triangle::new(Color::Green, Size::Three),
                                     Triangle::new(Color::Yellow, Size::Two),
                                     Triangle::new(Color::Red, Size::One)),
                              stack!(Triangle::new(Color::Yellow, Size::Three),
                                     Triangle::new(Color::Blue, Size::Two)));
    println!("{}", mascot_study);
}


pub fn play() {
    let mut bound: u16 = 30;
    {
        let mut arg_parser = ArgumentParser::new();
        arg_parser.set_description("Mezzanine: a guessing game");
        arg_parser.refer(&mut bound).add_option(
            &["--bound"], Store,
            "the largest admissible number in the game"
        );
        arg_parser.parse_args_or_exit();
    }

    decorative_display_header();
    number::play(bound);
}
