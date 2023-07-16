 rust
struct X(int);

impl Drop for X {
    fn drop(&mut self) { }
}

fn main() {
    let foo = X(1i);
    drop(foo);
    match foo {
        X(1i) => (),
        _ => unreachable!()
    }
}
