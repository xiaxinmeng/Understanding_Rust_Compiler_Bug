 rust
#![feature(braced_empty_structs)]

#[derive(Copy, Clone)]
enum Foo {
    Bar { },
}
