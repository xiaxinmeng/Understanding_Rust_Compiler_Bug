rust
#![feature(marker_trait_attr)]

struct Wrapper<T>(T);

#[marker]
pub trait MyTrait {}

impl MyTrait for Wrapper<u32> {}
impl MyTrait for Wrapper<bool> {}

fn weird_call<T>(val: T) -> Wrapper<T> where Wrapper<T>: MyTrait {
    Wrapper(val)
}

fn main() {
    let a: Wrapper<u32> = weird_call(25);
}
