use std::cmp;
use std::iter;


macro_rules! wrapln {
    ($($arg:tt)*) => {{
        let wrap_width = 79;
        let formatted = format!($($arg)*);
        let mut splitten = ("", &formatted[..]);
        while splitten.1.len() > wrap_width {
            let split_index = match splitten.1[..wrap_width].rfind(' ') {
                Some(index) => index + 1,  // split after the space
                None => wrap_width  // oh well
            };
            splitten = splitten.1.split_at(split_index);
            println!("{}", splitten.0);
        }
        println!("{}", splitten.1);
    }};
}


/// length of string in chars rather than bytes and excluding ANSI escape
/// sequences
fn visualen(string: &str) -> usize {
    let mut in_escape_sequence = false;
    let mut length = 0;
    for codepoint in string.chars() {
        if in_escape_sequence {
            if codepoint == 'm' {
                in_escape_sequence = false;
            }
        } else {
            if codepoint == '\u{1b}' {
                in_escape_sequence = true;
            } else {
                length += 1;
            }
        }
    }
    length
}

pub fn block_dimensions(block: &str) ->  (usize, usize) {  // (height, width)
    let lines = block.trim_right().split('\n').collect::<Vec<_>>();
    let height = lines.len();
    let mut width = 0;
    for line in &lines {
        let line_length = visualen(line);
        if line_length > width {
            width = line_length;
        }
    }
    (height, width)
}


fn pad_line(line: &str, target_width: usize) -> String {
    let natural_width = visualen(line);
    let pad = target_width - natural_width;
    let semipad = pad/2;
    let remipad = if 2 * semipad < pad { 1 } else { 0 };
    format!("{}{}{}",
            iter::repeat(' ').take(semipad).collect::<String>(),
            line,
            iter::repeat(' ').take(semipad+remipad).collect::<String>())
}


pub fn pack_blocks_vertically(upper_block: &str,
                              lower_block: &str) -> String {
    let (_upper_height, upper_width) = block_dimensions(upper_block);
    let (_lower_height, lower_width) = block_dimensions(lower_block);
    let grand_width = cmp::max(upper_width, lower_width);
    let mut packed = String::new();
    for line in upper_block.trim_right().split('\n')
        .chain(lower_block.trim_right().split('\n')) {
            packed.push_str(&pad_line(line, grand_width));
            packed.push('\n');
    }
    packed
}


fn vertically_pad_block(block: &str,
                        target_height: usize, target_width: usize) -> String {
    let natural_height = block.trim_right().split('\n')
        .collect::<Vec<_>>().len();
    let pad = target_height - natural_height;
    let semipad = pad/2;
    let remipad = if 2 * semipad < pad { 1 } else { 0 };
    let mut padded = String::new();

    fn push_padding(landing_pad: &mut String, pad: usize, line_count: usize) {
        for padline in iter::repeat(
            iter::repeat(' ')
                .take(pad).collect::<String>())
            .take(line_count) {
                landing_pad.push_str(&padline);
                landing_pad.push('\n');
            }
    }

    push_padding(&mut padded, target_width, semipad);
    padded.push_str(block);
    padded.push('\n');
    push_padding(&mut padded, target_width, semipad+remipad);
    padded
}


pub fn pack_blocks_horizontally(left_block: &str,
                                right_block: &str) -> String {
    let (left_height, left_width) = block_dimensions(left_block);
    let (right_height, right_width) = block_dimensions(right_block);
    let grand_height = cmp::max(left_height, right_height);

    let mut packed = String::new();
    for (semiline, cosemiline)
        in vertically_pad_block(left_block.trim_right(),
                                grand_height, left_width)
        .split('\n')
        .zip(vertically_pad_block(right_block.trim_right(),
                                  grand_height, right_width)
             .split('\n')) {
            packed.push_str(semiline);
            packed.push_str(cosemiline);
            packed.push('\n');
        }
    packed.pop();
    packed
}


#[cfg(test)]
mod tests {
    use ansi_term;
    use super::{block_dimensions, visualen, pack_blocks_horizontally,
                pack_blocks_vertically};

    #[test]
    fn concerning_text_block_dimensions() {
        let (height, width) = block_dimensions("foo\nbarr\nquux\n");
        assert_eq!(3, height);
        assert_eq!(4, width);
    }

    #[test]
    fn concerning_verical_packing() {
        let upper = "XX\nXX\n";
        let lower = "XXXXX\nXXXXX\n";
        let expected_packing = " XX  \n XX  \nXXXXX\nXXXXX\n";
        println!("expected_packing:\n{}", expected_packing);
        println!("actual packing:\n{}", pack_blocks_vertically(upper, lower));
        assert_eq!(expected_packing, pack_blocks_vertically(upper, lower));
    }

    #[test]
    fn concerning_horizontal_packing() {
        let left = "XXX\nXXX\n";
        let right = "XX\nXX\nXX\nXX\n";
        let expected_packing = "   XX\nXXXXX\nXXXXX\n   XX\n";
        println!("expected_packing:\n{}", expected_packing);
        println!("actual packing:\n{}", pack_blocks_horizontally(left, right));
        assert_eq!(expected_packing, pack_blocks_horizontally(left, right));
    }

    #[test]
    fn concerning_the_length_of_strings_containing_control_codes() {
        let codetext = format!("{}", ansi_term::Colour::Red
                               .paint("control code"));
        println!("colored string __repr__esentation: {:?}", &codetext);
        println!("colored string char-acterization: {:?}",
                 &codetext.chars().collect::<Vec<_>>());
        assert_eq!(12, visualen(&codetext));
    }

}
