rust
#[bikeshed::this_crate_shall_not_impl]
impl<#[downstream] T : ?Sized, A: Allocator> * for Box<T, A>;
