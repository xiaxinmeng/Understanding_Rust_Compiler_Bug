rust
#![feature(conservative_impl_trait)]

pub fn iter(x: &usize) -> impl Iterator<Item=(&i32, &Box<i32>)> {
    unimplemented!()
}

fn main() {}
