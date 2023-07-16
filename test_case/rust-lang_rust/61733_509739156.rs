rust
fn main() {
    // `0` inside `stmt!()` is no longer a trailing expression
    // Fixes https://github.com/rust-lang/rust/issues/33953
    stmt!()
}
