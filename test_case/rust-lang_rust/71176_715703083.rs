rust
trait A {
    type B<'a>;
}

struct C(&'static dyn A<B = ()>);

fn main() {}
