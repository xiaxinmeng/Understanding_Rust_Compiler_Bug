 rust
enum X {
    Foo(uint),
    Bar(bool)
}

fn main() {
    match Foo(42) {
        Foo(..) => (),
        _ if true => (),
        Bar(..) => fail!("Oh dear")
    }
}
