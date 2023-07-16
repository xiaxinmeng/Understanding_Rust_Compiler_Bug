rust
fn main() {
    let x: f64 = 0.25 - 5000000.0;
    let y: f64 = -5000000.0;
    assert_eq!(x, y); // this will panic
}
