#![feature(iter_arith)]

mod hypothesis_space;

#[cfg(not(test))]
fn main() {
    println!("Hello Mezzanine world!");
    hypothesis_space::divisibility_priors();
}
