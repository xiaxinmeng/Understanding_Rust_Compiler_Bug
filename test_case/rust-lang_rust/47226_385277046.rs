bash
#!/bin/sh

cargo new --lib repro_macro
cargo new --lib repro

echo >repro_macro/src/lib.rs '
#![feature(proc_macro)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn m(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let tt = iter.next().unwrap();
    tt.span().error("first angle bracket").emit();
    assert!(iter.next().is_some());
    assert!(iter.next().is_none());
    TokenStream::empty()
}
'

echo >>repro_macro/Cargo.toml '
[lib]
proc-macro = true
'

echo >repro/src/lib.rs '
#![feature(proc_macro)]
extern crate repro_macro;
repro_macro::m!(>>);
fn main() {}
'

echo >>repro/Cargo.toml '
repro_macro = { path = "../repro_macro" }
'

cargo build --manifest-path repro/Cargo.toml
