
trait Foo {
    fn foo(&self);
}

impl Foo for i32 {
    fn foo(&self) { println!("success {}", self); }
}

struct Baz { x: i32 }

impl Foo for Baz where i32: Foo {
    fn foo(&self) {
        self.x.foo()
    }
}

fn main() {
    let b = Baz { x: 5 };
    b.foo();
}

