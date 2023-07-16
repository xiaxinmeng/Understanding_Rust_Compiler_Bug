rust
// library crate in Rust 2021:
#[macro_export]
macro_rules! x { ($t:tt) => { $t::panic!("{{"); }; }

#[macro_export]
macro_rules! y { ($t:tt) => { std::$t!("{{"); }; }


// binary crate in Rust 2018:
use dep2021::*;

macro_rules! z {
    (x) => { x!(std); };
    (y) => { y!(panic); };
}

fn main() {
    x!(std);   // 2021
    y!(panic); // 2021
    z!(x);     // 2018 !!
    z!(y);     // 2021
}
