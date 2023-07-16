rust
struct Foo {
    a: i32
}

impl Foo {
    fn bar(&mut self) {
        self.a = 42;
    }
}

fn main() {
    let mut foo = Foo { a: 1 };
    foo.bar();
}
