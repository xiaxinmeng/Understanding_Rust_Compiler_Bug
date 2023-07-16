rust
struct Foo;

fn main() {
    || { if let Foo::NotEvenReal() = Foo {} };
}
