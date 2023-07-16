
#![feature(associated_consts)]

trait Matrix {
    const EYE: f64;
}

fn get_EYE<M: Matrix>(t: M) {
    M::EYE
}

fn main() { }
