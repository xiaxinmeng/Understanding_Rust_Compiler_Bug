rust
use std::fmt::Debug;

fn f(_trait_object: &Debug) {}

fn main() {
    f("str"); // does not compile today. would coerce to &&str
    f(&[1][..]); // does not compile today. would coerce to &&[{integer}]
}
