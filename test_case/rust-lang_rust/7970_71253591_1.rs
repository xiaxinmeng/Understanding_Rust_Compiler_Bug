 rust
// b.rs
macro_rules! bar { }
macro_rules! foo {
    () => (bar!());
}
