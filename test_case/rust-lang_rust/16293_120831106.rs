 rust
struct Foo(i32, i64);

impl Foo {
    fn classic(&self) {
        println!("{}, {}", self.0, self.1);
    }

    fn destructuring(self) {
        let Foo(u, v) = self;
        println!("{}, {}", u, v);
    }
}

fn main() {
    let x = Foo(3, 14);
    x.classic(); // Works, but doesn't destructure
    Foo::destructuring(x);
}
