 rust
#![feature(specialization)]

trait Marker {
    type Mark;
}

trait Foo { fn foo(&self); }

struct Fizz;

impl Marker for Fizz {
    type Mark = ();
}

impl Foo for Fizz {
    fn foo(&self) { println!("Fizz!"); }
}

impl<T> Foo for T
    where T: Marker, T::Mark: Foo
{
    default fn foo(&self) { println!("Has Foo marker!"); }
}

struct Buzz;

impl Marker for Buzz {
    type Mark = Fizz;
}

fn main() {
    Fizz.foo();
    Buzz.foo();
}
