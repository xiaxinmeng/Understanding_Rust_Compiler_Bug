rust
fn main() {
    fn f<T>(_: T) {}
    let f = f;
    let v = 0;
    f(&v);
}
