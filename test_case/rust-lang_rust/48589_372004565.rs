rust
macro_rules! foo {
    ($a:ident # $b:ident) => (())
}
fn main() {
    foo!(r#foo);
}
