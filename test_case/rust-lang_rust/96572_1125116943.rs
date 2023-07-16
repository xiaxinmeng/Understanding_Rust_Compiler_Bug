rust
#![feature(type_alias_impl_trait)]

fn main() {
    type T = impl Copy;
    let foo: T = (1u32, 2u32);
    match foo {
        (a, b) => (),
    }
}
