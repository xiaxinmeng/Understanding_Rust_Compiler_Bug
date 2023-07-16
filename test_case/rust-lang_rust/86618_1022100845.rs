rust
#![feature(type_changing_struct_update)]

#[derive(Default)]
struct A {
    x: i32
}

fn main() {
    let a = A{
        ..Default::default()
    };
}
