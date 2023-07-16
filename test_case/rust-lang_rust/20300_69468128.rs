 rust
trait Trait {
    type Item;
}

struct Foo;

impl Foo {
    fn foo1<T: Trait>(_x: T::Item) {}

    fn foo2<T>(_x: <T as Trait>::Item) where T: Trait {}
}

struct Bar;

impl Trait for Bar {
    type Item = i32;
}

fn main() {
    Foo::foo1::<Bar>(5i32);
    Foo::foo2::<Bar>(5i32);
}
