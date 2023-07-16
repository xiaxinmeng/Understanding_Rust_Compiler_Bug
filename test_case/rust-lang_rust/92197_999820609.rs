rust
fn main() {
    match (f32::NAN, f32::NAN) {
        (x, y) if x > y => {},
        (x, y) if x < y => {},
        (x, y) if x == y => {},
        _ => unreachable!("oh no"),
    }
}
