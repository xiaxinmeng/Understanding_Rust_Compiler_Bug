 Rust
#![allow(dead_code)]

#[derive(Clone)]
enum Expression {
    Dummy,
    Add(Box<Expression>),
}

use Expression::*;

fn simplify(exp: Expression) -> Expression {
    match exp {
        Add(n) => *n.clone(),
        _ => Dummy
    }
}

fn main() {}
