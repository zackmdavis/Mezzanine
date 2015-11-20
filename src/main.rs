#![feature(iter_arith)]

mod inference;

#[cfg(not(test))]
fn main() {
    println!("Hello Mezzanine world!");
    inference::divisibility_priors();
}
