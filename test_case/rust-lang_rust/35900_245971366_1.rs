 rust
#![feature(rustc_macro)]

#[macro_use] extern crate demo_plugin;

#[derive(PartialEq, Eq, Foo)]
struct Bar {
    a: i32,
    b: i32,
}
