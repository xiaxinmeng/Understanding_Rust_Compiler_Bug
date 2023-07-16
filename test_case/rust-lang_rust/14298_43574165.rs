 .rs
#![feature(macro_rules)]

macro_rules! foo ( () => (()) )

fn main() {
    match () {
        x @ foo!() => (),
    }
}
