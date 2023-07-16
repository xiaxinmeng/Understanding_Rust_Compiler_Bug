rust
#![feature(generic_associated_types)]
#![deny(unused_lifetimes)]

pub trait Foo {
    type Message<'a>; //~ ERROR
}
