rust
#!/bin/sh

rm -rf repro
rm -rf repro_macros

cargo new --lib repro
cargo new --lib repro_macros

echo >repro/src/lib.rs '
#![feature(proc_macro, proc_macro_gen, proc_macro_mod)]

extern crate repro_macros;

use repro_macros::nothing;

trait Trait1 {
    fn method(&self) {}
}

trait Trait2 {
    fn method(&self) {}
}

impl Trait1 for u8 {}

impl Trait2 for u8 {}

#[nothing]              // < < < COMMENT OUT TO FIX COMPILATION > > >
pub mod module {
    use super::Trait1;
    
    pub fn foo(arg: u8) {
        arg.method()
    }
}
'

echo >>repro/Cargo.toml '
repro_macros = { path = "../repro_macros" }
'

echo >repro_macros/src/lib.rs '
#![feature(proc_macro)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn nothing(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}
'

echo >>repro_macros/Cargo.toml '
[lib]
proc-macro = true
'

cargo build --manifest-path repro/Cargo.toml 
