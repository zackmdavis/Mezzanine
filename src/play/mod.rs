mod number;
mod triangle;


use argparse::{ArgumentParser, Store, StoreTrue};

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
    let mut number = false;
    {
        let mut arg_parser = ArgumentParser::new();
        arg_parser.set_description("Mezzanine: a guessing game");
        arg_parser.refer(&mut bound).add_option(
            &["--bound"], Store,
            "the largest admissible number in the game"
        );
        // XXX: should really be an enum with a default rather than a boolean
        arg_parser.refer(&mut number).add_option(
            &["--number"], StoreTrue,
            "play the classic number game rather than triangle science"
        );
        arg_parser.parse_args_or_exit();
    }

    decorative_display_header();
    if number {
        number::play(bound);
    } else {
        triangle::play();
    }
}
