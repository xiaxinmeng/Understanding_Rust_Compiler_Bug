 rust
#![feature(phase)]
#![no_std]
#![feature(globs)]
#[phase(plugin, link)]
extern crate "std" as std;
extern crate "native" as rt;
#[prelude_import]
use std::prelude::*;
struct Doc;


#[cfg(phantom)]
struct Node<'a>;

impl Doc {
    #[cfg(phantom)]
    fn node<'a>(&'a self) -> Node<'a> { (Node as Node<'<empty>>) }

}

fn main() {
    let node;
    ({
         let doc = (Doc as Doc);
         ((node as Node<'<empty>>) = ((doc as Doc).node() as Node<'_>) as ());
     } as ())
}
