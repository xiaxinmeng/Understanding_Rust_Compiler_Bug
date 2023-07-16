sh
#!/bin/bash

rustup update nightly-2018-02-08

cargo new repro --bin

echo >>repro/Cargo.toml '
[lib]
proc-macro = true
'

echo >repro/src/lib.rs '
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Panic)]
pub fn f(_input: TokenStream) -> TokenStream {
    panic!("no can do");
}
'

echo >repro/src/main.rs '
#[macro_use]
extern crate repro;

#[derive(Panic)]
struct S;

fn main() {}
'

cargo +nightly-2018-02-08 build --manifest-path repro/Cargo.toml
