rust
#![allow(unused)]
fn main() {
    let a: f64 = (-25252734735360000_f64 * 365_f64)  + ( -25252734735360000_f64 * 0.2425_f64) ;
    let b: f64 = -25252734735360000_f64 * 365.2425_f64;
    println!("a = {}", a);
    println!("b = {}", b);
    assert_eq!(a, b);
}
