rust
#![feature(conservative_impl_trait)]

struct D;

impl D {
    fn i(&self) -> impl Iterator<Item=&str> {
        &[].iter()
    }
}

fn main() {
    let d = D;
    for _ in d.i() {}
}
