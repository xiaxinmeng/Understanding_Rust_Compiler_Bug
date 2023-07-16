rust
#![feature(rustc_attrs)]

struct Foo;
#[rustc_symbol_name]
impl Foo {}
