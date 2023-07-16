rust
fn main() {
    use std::fmt::Write;
    let mut example = String::new();
    write!(&mut example, "{}", 42);
}
