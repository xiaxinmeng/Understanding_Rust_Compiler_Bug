rust
#![feature(use_extern_macros)]

extern crate two_macros;

::two_macros::macro_one!();
two_macros::macro_one!();

mod foo { pub use two_macros::macro_one as bar; }
