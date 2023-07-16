
struct X { x: uint }

trait Foo {
    fn foo(&self) -> &self/uint;
}

impl X: Foo {
    fn foo(&self) -> &self/uint { &self.x }
}

fn main() {
    let x0 = ~X { x: 3 };
    let x1 = ~X { x: 3 };
    let y = (move x0) as ~Foo;
    let z = y.foo();
    y = (move x1) as ~Foo; // Should result in an error because we are modifying `y`, which is still borrowed
    io::println(*z);
}
