 rust
#![no_std]

#![feature(lang_items)]
#[lang="sized"]

struct Foo; // can be omitted

struct Bar<T> {field: T}

fn main() {}
