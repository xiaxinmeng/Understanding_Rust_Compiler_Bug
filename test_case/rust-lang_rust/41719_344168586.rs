rust
#![feature(use_extern_macros)]

enum Foo {}

#[derive(Foo::bar)]
struct A {}
