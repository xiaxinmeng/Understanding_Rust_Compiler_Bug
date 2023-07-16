rust
fn main() {}

extern "C" fn foo<F: Fn(i32) -> usize>(f: F) -> usize {
    f(10)
}

extern "C" fn bar<T: Clone>(f: fn(i32) -> usize) -> usize {
    f(10)
}
