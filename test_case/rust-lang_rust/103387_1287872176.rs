rust
#![feature(trait_alias)]

pub trait Trait {}

pub trait WithAssoc {
    type Assoc;
}

pub trait WithBoundAssoc = WithAssoc where <Self as WithAssoc>::Assoc: Trait;
// pub trait WithBoundAssoc: WithAssoc where Self::Assoc: Trait {}

pub struct S<T: WithBoundAssoc>(T::Assoc);
