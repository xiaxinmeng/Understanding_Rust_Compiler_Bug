 rust
fn main() {
    match &[1][..] { &ref xs => xs[0], _ => 0 };
}
