rust
trait Foo {
    fn foo(&self);
}
struct A;
struct B;
impl const Foo for A {
    fn foo(&self) {}
}
impl Foo for B {
    fn foo(&self) {
        // Open file handles and network handles and compute some random numbers
    }
}

const fn bar<T: Foo>(t: &T) {
    t.foo();
}

const X: () = bar(&B); // not legal
const Y: () = bar(&A); // legal

fn main() {
    bar(&B); // legal
    bar(&A); // legal
}
