rust
// crate foo
#[macro_export]
macro_rules! macro_from_foo { () => {} }

// crate bar
#![feature(use_extern_macros)]
// ^ implied by `#![feature(proc_macro)]`, `#![feature(decl_macro)]`

extern crate foo; // no #[macro_use]
use foo::macro_from_foo; // `pub use` also works, subsumes `#[macro_reexport]`
macro_from_foo!();
