 rust
mod mul1 {
    pub trait Mul {}
}

mod mul2 {
    pub trait Mul {}
}

#[derive(Debug)]
struct Foo;

// When we comment the next line:
use mul1::Mul;

// We get the following error for the `impl` below:
//   error: use of undeclared trait name `Mul` [E0405]
// Ideally, when this bug is fixed, we would like to see:
//   error: use of undeclared trait name `Mul` [E0405]
//   note: Possible alternatives are:
//         use mul1::Mul;        // from module mul1 in this crate <or>
//         use mul2::Mul;        // from module mul1 in this crate <or>
//         use std::ops::Mul;    // from the standard library <or>
//         use other_crate::Mul; // from external crate other_crate ...
impl Mul for Foo { }

fn main() {
    let foo = Foo;
    println!("Hello, {:?}!", foo);
}
