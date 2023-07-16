
#!/bin/sh

# set the linked toolchain here
TOOLCHAIN=bisector-nightly-2016-12-23-x86_64-unknown-linux-gnu

rm -rf repro
rm -rf repro_dispatch
rm -rf repro_derive
cargo new --lib repro
cargo new --lib repro_dispatch
cargo new --lib repro_derive

echo >repro/src/lib.rs '
#[macro_use]
extern crate repro_dispatch;

#[macro_use]
extern crate repro_derive;

pub struct MyDispatch;

impl repro_dispatch::Dispatch for MyDispatch {
    type Call = ();
}

#[macro_export]
macro_rules! problem_local {
    () => {
        #[derive(Print)]
        #[local]
        pub struct S(<MyDispatch as $crate::Dispatch::Second<Test>>::Call);
    }
}

problem!();
problem_local!();
'

echo >>repro/Cargo.toml '
repro_dispatch = { path = "../repro_dispatch" }
repro_derive = { path = "../repro_derive" }
'

echo >repro_dispatch/src/lib.rs '
pub trait Dispatch {
    type Call;
}

#[macro_export]
macro_rules! problem {
    () => {
        #[derive(Print)]
        #[nonlocal]
        pub struct S(<MyDispatch as $crate::Dispatch::Second<Test>>::Call);
    }
}
'

echo >repro_derive/src/lib.rs '
#![feature(proc_macro, proc_macro_lib)]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Print)]
pub fn derive(input: TokenStream) -> TokenStream {
    println!("\n\n{}\n\n", input);
    "".parse().unwrap()
}
'

echo >>repro_derive/Cargo.toml '
[lib]
proc-macro = true
'

CARGO_INCREMENTAL=0 cargo +$TOOLCHAIN build --manifest-path repro/Cargo.toml
CARGO_INCREMENTAL=0 cargo +$TOOLCHAIN rustc --manifest-path repro/Cargo.toml -- --version
