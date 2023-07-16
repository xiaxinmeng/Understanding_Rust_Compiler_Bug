rust
#[cfg(::proc_macro::TokenStream)]
struct S;

macro_rules! m { () => { extern crate proc_macro; } }

m!();
