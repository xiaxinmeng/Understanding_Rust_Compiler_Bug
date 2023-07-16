 Rust
struct Foo(uint);

fn uint_to_foo(x: uint) -> Foo {
    Foo(x)
}

fn main() {
    range(0u, 10).map(uint_to_foo);
}
