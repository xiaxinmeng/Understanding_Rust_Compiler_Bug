rust
#![feature(type_changing_struct_update)]

#[derive(Default)]
struct Struct<T>(T);

fn _f() { Struct { 0: (), ..Default::default() }; }
