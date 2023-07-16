 rust
use std::default::Default;

trait Foo: Default { fn foo(); }
impl Foo for () { fn foo() { println!("()"); } }
impl Foo for i32 { fn foo() { println!("i32"); } }

struct Error;
impl Error {
    fn foo(&self) -> ! { loop {} }
}

fn bar<T: Foo>() -> Result<T, Error> { loop {} }

fn baz<T: Foo>(_: &T) { <T as Foo>::foo(); }

fn main() {
    let mut a = Default::default();
    |&mut:| a = bar().unwrap_or_else(|e| e.foo());
    baz(&a); // ()
}
