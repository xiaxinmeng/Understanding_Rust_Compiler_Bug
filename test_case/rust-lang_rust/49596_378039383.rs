bash
#!/bin/sh

cargo new --lib structopt_derive
cargo new --lib repro

echo >structopt_derive/src/lib.rs '
#![feature(proc_macro)]

extern crate proc_macro;
use proc_macro::{TokenStream, TokenTreeIter, TokenTree, TokenNode, Spacing};

#[proc_macro_derive(StructOpt)]
pub fn derive_structopt(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    assert_eq!(iter.next().unwrap().to_string(), "struct");
    assert_eq!(iter.next().unwrap().to_string(), "S");
    let mut inner = unwrap_group(iter.next().unwrap());

    assert_eq!(inner.next().unwrap().to_string(), "#");
    let mut x = unwrap_group(inner.next().unwrap());
    assert_eq!(x.next().unwrap().to_string(), "doc");
    println!("{:?} {}", unwrap_spacing(x.next().unwrap()), x.next().unwrap());

    assert_eq!(inner.next().unwrap().to_string(), "#");
    let mut y = unwrap_group(inner.next().unwrap());
    assert_eq!(y.next().unwrap().to_string(), "doc");
    println!("{:?} {}", unwrap_spacing(y.next().unwrap()), y.next().unwrap());

    TokenStream::empty()
}

fn unwrap_group(tt: TokenTree) -> TokenTreeIter {
    match tt.kind {
        TokenNode::Group(_, s) => s.into_iter(),
        _ => unimplemented!(),
    }
}

fn unwrap_spacing(tt: TokenTree) -> Spacing {
    match tt.kind {
        TokenNode::Op(_, s) => s,
        _ => unimplemented!(),
    }
}
'

echo >>structopt_derive/Cargo.toml '
[lib]
proc-macro = true
'

echo >repro/src/lib.rs '
#![allow(dead_code)]

#[macro_use]
extern crate structopt_derive;

fn f() {
    #[derive(StructOpt)]
    struct S {
        /// X
        /// Y
        #[doc(hidden)]
        foo: bool,
    }
}
'

echo >>repro/Cargo.toml '
structopt_derive = { path = "../structopt_derive" }
'

cargo build --manifest-path repro/Cargo.toml
