rust
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(nonstandard_style)]

pub trait Trait {
    type Trait;
}

/// [`Struct::Trait`]
//~^ ERROR
pub struct Struct;

impl Trait for Struct {
    type Trait = Struct;
}

impl Struct {
    pub const Trait: usize = 0;
}
