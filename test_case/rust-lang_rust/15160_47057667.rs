 rust
#![feature(quad_precision_float)]

fn main() {
    let x: f128 = 5.5;
    let y = x * 2.0 - 10.0;
    println!("{}", y as f64)
}
