rust
#![cfg_attr(nightly, feature(decl_macro))]

#[cfg(nightly)]
pub macro Default($item:item) { }
