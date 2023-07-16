rust
//! `::upstream`
pub trait Trait {}

#[bikeshed::this_crate_shall_not_impl]
impl<#[downstream] T : ?Sized, #[downstream] A: Allocator> Trait for Box<T, A>;
