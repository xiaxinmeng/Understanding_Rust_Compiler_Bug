 rust
#[derive(Eq)]
struct A { a: i32 }

impl A {
    fn eq(&self, other: &A) -> bool { true }
}

#[derive(Eq, PartialEq)]
struct B { a: A }

const CONST_B1: B = B { a: A { a: 1 } };
const CONST_B2: B = B { a: A { a: 2 } };

fn main() {
    match CONST_B1 {
        CONST_B2 => println!("b2"),
        CONST_B1 => println!("b1"),
        _ => {}
     }
}
