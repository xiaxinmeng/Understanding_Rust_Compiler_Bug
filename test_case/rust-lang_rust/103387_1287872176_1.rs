rust
#[cfg(trait_alias)]
pub trait WithBoundAssoc = WithAssoc where <Self as WithAssoc>::Assoc: Trait;

#[doc(hidden)]
#[cfg(not(trait_alias)]
pub trait WithBoundAssoc: WithAssoc<Assoc = Self::__Assoc> {
    type __Assoc: Trait;
}

#[doc(hidden)]
#[cfg(not(trait_alias)]
impl<T: WithAssoc> WithBoundAssoc for T where Self::Assoc: Trait {
    type __Assoc = Self::Assoc;
}
