rust
#![feature(type_alias_impl_trait)]

use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;
use std::ops::Index;

pub struct TypedIndex<T> {
    pub ix: usize,
    phantom: PhantomData<T>,
}

impl<T> TypedIndex<T> {
    pub fn new() -> Self {
        unimplemented!()
    }
}

pub type DummyT<T> = impl Fn();
pub type Dummy     = impl Fn();
pub fn _dummy()      -> Dummy     { move || {} }
pub fn _dummy_t<T>() -> DummyT<T> { move || {} }

pub enum Variants<T> {
    V(A1<T>),
}
impl<'t, S> TryFrom<&'t Variants<S>> for &'t A1<S> {
    type Error = ();
    fn try_from(_: &'t Variants<S>) -> Result<&'t A1<S>, Self::Error> {
        unimplemented!()
    }
}

pub type Variants2<OnDirty> = Variants<DummyT<OnDirty>>;

pub struct Scope<OnDirty> {
    pub attributes: Vec<Variants2<OnDirty>>,
}

impl<OnDirty> Scope<OnDirty> {
    pub fn new() -> Self {
        unimplemented!()
    }
}

impl<T, OnDirty> Index<TypedIndex<T>> for Scope<OnDirty>
where
    for<'t> &'t T: TryFrom<&'t Variants2<OnDirty>>,
{
    type Output = T;
    fn index(&self, t: TypedIndex<T>) -> &Self::Output {
        self.attributes.index(t.ix).try_into().ok().unwrap()
    }
}

pub struct A1 <T>  (std::marker::PhantomData<(T)>);
pub type   A2<T>  = A1  <DummyT<T>>;
pub type   A3     = A2 <Dummy>;
pub type   Scope2 = Scope<Dummy>;

pub fn start() {
    let pos_id = TypedIndex::new();
    let pt_scope = Scope2::new();
    let _pos: &A3 = &pt_scope[pos_id];
}
