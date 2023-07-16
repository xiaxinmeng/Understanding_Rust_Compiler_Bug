 rust
#![feature(step_trait)]
use std::iter::Step;

fn main () {
    let steps = Step::steps_between(&0, &5, &2);
    println!("{:?}", steps);
}
