rust
#![feature(conservative_impl_trait)]

fn g() -> impl Iterator<Item=u8> { //~ ERROR E0277
    loop {}
}

fn main() {}
