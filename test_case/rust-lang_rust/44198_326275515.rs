rust
#![feature(conservative_impl_trait)]

fn foo(_: &u8) -> impl Iterator<Item = &u8> {
    unimplemented!()
}

fn main() {}
