rust
pub struct A {}

pub trait X {
    fn do_thing() -> A;
}

impl<I> X for I {
    fn do_thing() -> A {
        unimplemented!()
    }
}
