rust
pub trait Func<T> {
    fn func(self);
}

pub trait A {
    type AssocT;
}

impl A for () {
    type AssocT = ();
}

pub fn crash() {
    Func::<std::marker::PhantomData<<() as A>::AssocT>>::func(());
}
