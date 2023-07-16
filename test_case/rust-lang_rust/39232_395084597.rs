rust
use std::fmt::Debug;

trait Foo: Debug + 'static {
    fn debug(&self) -> String where Self: Sized { format!("{:?}", self) }
}

impl Foo {
    fn debug(&self) -> String { format!("{:?}", self) }
}

trait AsFoo {
    fn as_foo(&self) -> &Foo;
}
impl<T: Bar> AsFoo for T {
    fn as_foo(&self) -> &Foo { self }
}

trait Bar: Foo + AsFoo {}
impl Bar {
    fn debug_via_bar(&self) { println!("debug: {}", self.as_foo().debug()) }
}

impl Foo for i32 {}
impl Bar for i32 {}

fn main() {
    let x: &Bar = &42i32;
    x.debug_via_bar();
}
