 rust
extern mod a;

fn main() {
    let foo = a::Foo { a: 52 };
    foo.foo();
}
