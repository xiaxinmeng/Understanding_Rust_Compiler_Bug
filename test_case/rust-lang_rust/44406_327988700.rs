rust
macro_rules! foo {
    ($rest:tt) => {
        bar(baz: $rest)
    }
}
fn main() {
    foo!(true);
}
