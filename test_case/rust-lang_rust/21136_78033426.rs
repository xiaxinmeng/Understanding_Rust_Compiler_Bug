 rust
use std::marker::PhantomData;

struct GenericStruct<T> {
    marker: PhantomData<T>
}

impl <T> GenericStruct<T> {
    // or fn bar(&self, _: &mut T) {}
    fn bar(&self, _: T) {}                   // <--- Only change
}

struct FooBar;

fn main() {
    let mut foobar = FooBar;
    let foo = GenericStruct { marker: PhantomData };
    foo.bar(&mut foobar);
    foo.bar(&mut foobar);
}
