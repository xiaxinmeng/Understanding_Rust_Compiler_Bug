rust
macro_rules! pathexpr {
    ($p:path) => ($p);
}
fn main() {
    println!("{}", pathexpr!(std::str::from_utf8())(b"Hello").unwrap());
}
