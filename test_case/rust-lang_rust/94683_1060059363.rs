rust
#![feature(generic_associated_types)]

pub trait Trait {
    type Gat<'a>;
}

pub type T = fn(&<() as Trait>::Gat<'_>);
