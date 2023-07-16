 rust
struct Foo<A=(int, char)> {
    a: A
}

fn default_foo(x: Foo) {
     let (_i, _c): (int, char) = x.a;
     x.bar();
}

impl Foo {
    fn bar(&self) {
        let (_i, _c): (int, char) = self.a;
    }
}

fn main() {
    default_foo(Foo { a: (1, 'a') })
}
