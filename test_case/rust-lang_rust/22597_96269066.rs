 rust
#![feature(trace_macros)]

macro_rules! double {
    ($x:expr) => (2 * $x)
}

fn main() {
    trace_macros!(true);
    println!("{}", double!(21));
}
