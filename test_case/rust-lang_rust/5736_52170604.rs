
extern crate num;
use num::complex::{Complex32, Complex64, Complex};

fn main() {
    let _0_0i: Complex64 = Complex { re: 0.0, im: 0.0 };
    let _1_0i: Complex64 = Complex { re: 1.0, im: 0.0 };

    println!("{}", _1_0i / _0_0i);
}
