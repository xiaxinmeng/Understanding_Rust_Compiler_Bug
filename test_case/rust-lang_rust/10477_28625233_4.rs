 rust
struct Foo;
impl Foo { fn foo(&self) { bar() } }
fn bar() {
   fn baz() {}

   Foo.foo();
   baz();
}

fn main() {}


struct Foo2;
impl Foo2 { fn foo2(&self) { bar2() } }
fn bar2() {
   fn baz2() {}

   Foo2.foo2();
   baz2();
}

pub fn pub_fn() {
    let foo2_struct = Foo2;
    foo2_struct.foo2();
}
