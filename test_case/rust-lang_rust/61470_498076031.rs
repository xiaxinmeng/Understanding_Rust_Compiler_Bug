bash
#!/bin/bash

cargo new repro

echo >>repro/Cargo.toml '
[lib]
proc-macro = true
'

echo >repro/src/lib.rs '
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(D)]
pub fn m(input: TokenStream) -> TokenStream {
    assert_eq!(input.to_string(), "pub enum E { V = 0, }");
    TokenStream::new()
}
'

echo >repro/src/main.rs '
#[derive(repro::D)]
pub enum E { V = 0x0 }
fn main() {}
'

set -x
cargo +stable check --manifest-path repro/Cargo.toml
cargo +beta check --manifest-path repro/Cargo.toml
