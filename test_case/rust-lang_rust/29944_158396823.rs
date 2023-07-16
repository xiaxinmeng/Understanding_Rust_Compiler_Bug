 rust
#![feature(float_extras)]
extern crate ieee754; // [dependencies] ieee754 = "0.2"
use ieee754::Ieee754;

use std::f32;

// C as f32
const C_H: f32 = 57.295780181884765625;
// (C - C_H) as f32
const C_L: f32 = -6.6880244276035227812826633453369140625e-7;

fn exact_mul(x: f32) -> f32 {
    let a = C_L * x;
    let b = C_H.mul_add(x, a);
    b
}

fn main() {
    let mut errs = vec![];
    let mut count = 0_usize;
    for x in f32::MIN.upto(f32::MAX) {
        count += 1;
        let exact = exact_mul(x);
        let approx = x.to_degrees();
        if exact != approx {
            if let Some(ulp) = exact.ulp() {
                let ulp_error = (exact - approx).abs() / ulp;
                let bits_wrong = ulp_error.round() as u32;
                errs.push(bits_wrong);
            }
        }
    }
    println!("{}/{} wrong, max ulp err {}",
             errs.len(), count,
             errs.iter().cloned().max().unwrap_or(0))
}
