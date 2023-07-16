 rust
fn outer<T>(x: T) -> T {
    fn inner(x: uint) -> uint {
        x
    }

    x
}

fn main() {
    outer(5i);
    outer(5u);
}
