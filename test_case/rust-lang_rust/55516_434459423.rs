rust
fn main() {
    use std::fmt::Write;
    let mut example = String::new();
    (&mut example).write_fmt(format_args!("{}", 42));
}
