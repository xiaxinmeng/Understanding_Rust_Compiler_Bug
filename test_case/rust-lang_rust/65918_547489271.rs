rust
#![feature(type_alias_impl_trait)]

mod attribute {
    use std::convert::TryFrom;

    pub struct Attribute<T, OnSet, OnResize> {
        _m: std::marker::PhantomData<(T, OnSet, OnResize)>,
    }

    pub enum AnyAttribute<OnSet, OnResize> {
        Varianti32(Attribute<i32, OnSet, OnResize>),
    }
    impl<'t, T, S> TryFrom<&'t AnyAttribute<T, S>> for &'t Attribute<i32, T, S> {
        type Error = ();
        fn try_from(_: &'t AnyAttribute<T, S>) -> Result<&'t Attribute<i32, T, S>, Self::Error> {
            unimplemented!()
        }
    }

    pub trait IsAttribute<OnSet, OnResize> {
        fn add_element(&mut self);
        fn len(&self) -> usize;
    }
}

mod scope {
    use std::convert::{TryFrom, TryInto};
    use std::marker::PhantomData;
    use std::ops::Index;

    use crate::attribute as attr;

    pub struct TypedIndex<T> {
        pub ix: usize,
        phantom: PhantomData<T>,
    }

    impl<T> TypedIndex<T> {
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    pub struct Scope<OnDirty> {
        pub attributes: Vec<AnyAttribute<OnDirty>>,
    }

    pub type Attribute<T, OnDirty> = attr::Attribute<
        T,
        DummyT<OnDirty>,
        DummyT<OnDirty>,
    >;

    pub type AnyAttribute<OnDirty> = attr::AnyAttribute<
        DummyT<OnDirty>,
        DummyT<OnDirty>,
    >;

    pub type DummyT<T> = impl Fn();
    pub fn _dummy<T>() -> DummyT<T> { move || {} }

    impl<OnDirty> Scope<OnDirty> {
        pub fn ne() -> Self {
            unimplemented!()
        }
    }

    impl<T, OnDirty> Index<TypedIndex<T>> for Scope<OnDirty>
    where
        for<'t> &'t T: TryFrom<&'t AnyAttribute<OnDirty>>,
    {
        type Output = T;
        fn index(&self, t: TypedIndex<T>) -> &Self::Output {
            self.attributes.index(t.ix).try_into().ok().unwrap()
        }
    }
}

pub type Closure_workspace_on_change_handler = impl Fn() + Clone;
pub fn workspace_on_change_handler() -> Closure_workspace_on_change_handler {
    move || {}
}

pub type Attribute<T> = scope::Attribute<T, Closure_workspace_on_change_handler>;
pub type AttributeScope = scope::Scope<Closure_workspace_on_change_handler>;

pub fn start() {
    let pos_id = scope::TypedIndex::new();
    let pt_scope = AttributeScope::ne();
    let _pos: &Attribute<i32> = &pt_scope[pos_id];
}
