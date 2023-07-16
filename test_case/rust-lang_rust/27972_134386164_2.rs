 rust
trait ZZ<'a> {}
fn _f<'a>() where std::fmt::Display : ZZ<'a> {}
fn main() {}
