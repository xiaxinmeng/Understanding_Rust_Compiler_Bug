 rust
fn main() {
    // assertion failed: `(left == right)` (left: `-2`, right: `2`)
    assert_eq!(-2i32.abs(), (-2i32).abs());
}
