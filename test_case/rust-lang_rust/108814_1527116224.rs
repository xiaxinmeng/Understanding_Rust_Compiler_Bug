`rust
#![crate_type="lib"]

pub trait Foo<T> {
    type A;
    fn get::A
}


struct YetAnotherStruct<'a, I: for<isize> Foo<&'x &isize>> {
    field: I::A,
}

