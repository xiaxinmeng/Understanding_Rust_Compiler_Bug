 rust
struct Foo;

fn uint_to_foo(x: uint) -> Foo {
    Foo
}

fn main() {
    range(0u, 10).map(uint_to_foo);
}
