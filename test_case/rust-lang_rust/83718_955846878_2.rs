rust
pub struct A {}

pub trait X {
    fn do_thing(self) -> A;
}

impl<I> X for I {
    fn do_thing(self) -> A {
        unimplemented!()
    }
}
