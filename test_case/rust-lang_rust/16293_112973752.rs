 rust
struct Foo(i32, i64);

impl Foo {
    fn classic(&self) {
        println!("{}, {}", self.0, self.1);
    }

    fn destructuring(&Foo(u, v): &Self) {
        println!("{}, {}", u, v);
    }
}

fn main() {
    let x = Foo(3, 14);
    x.classic(); // Works, but doesn't destructure
    Foo::destructuring(&x); // Works, but x is not an invocant
    x.destructuring(); // Explodes
}
