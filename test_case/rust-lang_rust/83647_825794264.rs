rust
trait A { type Assoc: ?Sized; }

impl A for () {
    type Assoc = Foo<()>;
}
struct Foo<T: A>(T::Assoc);

fn main() {
    let x: Foo<()>;
}
