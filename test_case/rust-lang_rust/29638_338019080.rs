rust
#![crate_type = "dylib"]
#![feature(macro_reexport)]

#[macro_reexport(reexported)]
#[macro_use] #[no_link]
extern crate macro_reexport_1;
