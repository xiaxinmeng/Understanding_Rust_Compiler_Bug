rust
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(nonstandard_style)]

pub trait Trait {
    type MAX;
}

/// [`u32::MAX`]
//~^ ERROR
impl Trait for u32 {
    type MAX = u32;
}
