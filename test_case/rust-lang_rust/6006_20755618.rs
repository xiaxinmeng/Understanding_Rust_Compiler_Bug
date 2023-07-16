 rust
struct Foo {
    x: ~fn()
}

fn main() {
    let y = || {};
    let x = Foo { x: y };
}
