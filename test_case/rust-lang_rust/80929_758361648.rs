rust
struct Foo {
    bar: fn() -> i32,
}

fn bar() {}

fn main() {
    Foo { bar };
}
