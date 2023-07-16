
#![feature(log_syntax)]

macro_rules! macro_2 {
    { } => { log_syntax!("asdf"); }
}

macro_rules! macro_1 {
    { } => { macro_2!() }
}

fn main() {
    macro_1!();
}
