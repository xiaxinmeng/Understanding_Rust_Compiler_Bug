 rust
enum X {
    Foo(uint),
    Bar(bool)
}

fn main() {
    match Foo(42) {
        _ if true => (),
        Foo(..) => (),
        Bar(..) => fail!("Oh dear")
    }
}
