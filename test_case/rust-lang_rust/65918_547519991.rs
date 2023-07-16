rust
#![feature(type_alias_impl_trait)]

use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;
use std::ops::Index;

type DummyT<T> = impl Fn();
fn _dummy_t<T>() -> DummyT<T> {
    || {}
}

struct Phantom<T>(PhantomData<T>);
struct Phantom2<T>(PhantomData<T>);
struct Scope<T>(Phantom2<DummyT<T>>);

impl<'t, S> TryFrom<&'t Phantom2<S>> for &'t Phantom<S> {
    type Error = ();
    fn try_from(_: &'t Phantom2<S>) -> Result<&'t Phantom<S>, Self::Error> {
        unimplemented!()
    }
}

impl<T> Scope<T> {
    fn new() -> Self {
        unimplemented!()
    }
}

impl<T, U> Index<Phantom<T>> for Scope<U>
where
    for<'t> &'t T: TryFrom<&'t Phantom2<DummyT<U>>>,
{
    type Output = T;
    fn index(&self, _: Phantom<T>) -> &Self::Output {
        (&self.0).try_into().ok().unwrap()
    }
}

fn main() {
    let pos_id = Phantom(PhantomData);
    let pt_scope = Scope::<i32>::new();
    let _pos: &Phantom<DummyT<i32>> = &pt_scope[pos_id];
}
