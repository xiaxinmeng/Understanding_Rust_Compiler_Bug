rust
struct Foo;
impl Foo {
    fn bar(&mut self, _: ()) {}
}

fn main() {
    let mut foo = Foo;
    foo.bar(drop(&foo)); // OK
    foo.bar(drop(&mut foo)); // ERROR
}
