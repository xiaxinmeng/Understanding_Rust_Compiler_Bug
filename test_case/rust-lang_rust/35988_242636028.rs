 rust
enum E {
    V([Box<E>])
}

fn foo(_: &E) {}

fn main() {}
