rust
#![feature(specialization)]

trait Foo: Sized {
    type Assoc: Copy + From<Self> + Into<Self>;
}

impl<T> Foo for T {
    default type Assoc = Self;
}

fn xyz<T: Foo>(t: T) -> T::Bar {
    t.into()
}

fn main() {
    xyz(5i32);
}
