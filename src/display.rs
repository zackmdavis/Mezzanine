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
