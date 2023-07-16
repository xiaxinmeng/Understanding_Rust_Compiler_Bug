 rust
#![feature(rustc_macro)]

#[macro_use] extern crate demo_plugin;

#[derive(Foo, Bar)]
struct Baz {
    a: i32,
    b: i32,
}
