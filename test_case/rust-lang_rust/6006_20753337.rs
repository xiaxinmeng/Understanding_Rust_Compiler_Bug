 rust
struct Foo {
    x: ~fn()
}

fn main() {
    let x = Foo { x: ||{} };
}
