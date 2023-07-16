rust
pub struct S;

pub trait P {
    type I: Into<u64> + Into<S>;
}

trait A<B> {
    fn m(self, _: B) -> S;
}

impl A<S> for S {
    fn m(self, _: S) -> S {
        todo!()
    }
}

impl A<&'static S> for S {
    fn m(self, _: &'static S) -> S {
        todo!()
    }
}

pub fn f<T: P>(i: T::I) {
    S.m(i.into());
}
