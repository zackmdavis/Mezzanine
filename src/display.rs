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


pub fn block_dimensions(block: &str) ->  (usize, usize) {  // (height, width)
    let lines = block.trim_right().split('\n').collect::<Vec<_>>();
    let height = lines.len();
    let mut width = 0;
    for line in &lines {
        let line_length = line.len();
        if line_length > width {
            width = line_length;
        }
    }
    (height, width)
}


fn pad_line(line: &str, target_width: usize) -> String {
    let natural_width = line.len();
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


#[allow(unused_variables, dead_code)]
pub fn pack_blocks_horizontally(left_block: &str,
                                right_block: &str) -> String {
    let (left_height, _left_width) = block_dimensions(left_block);
    let (right_height, _right_width) = block_dimensions(right_block);
    let grand_height = cmp::max(left_height, right_height);
    String::from("#TODO")
}


#[cfg(test)]
mod tests {
    use super::{block_dimensions, pack_blocks_vertically};

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

}
