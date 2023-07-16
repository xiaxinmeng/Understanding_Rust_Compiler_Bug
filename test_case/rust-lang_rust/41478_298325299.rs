rust
// rustc -Zsave-analysis 1.rs
struct B<S> {
}
fn f<S>() -> B<S> {
    B{}
}
fn main() {
}
