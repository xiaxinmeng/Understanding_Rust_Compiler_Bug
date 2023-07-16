rust
fn main() {
    let x: f32 = 0.25 - 5000000.0;
    let y: f32 = -5000000.0;
    assert_eq!(x, y); // will pass
}
