rust
fn main() {
    let black_box_one = (std::env::args().len()) as f32;
    let result = - (black_box_one / 0.0);
    let expected = f32::NEG_INFINITY;
    assert_eq!(result, expected);
}
