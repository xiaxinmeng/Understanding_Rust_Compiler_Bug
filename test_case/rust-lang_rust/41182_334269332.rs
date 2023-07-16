rust
#![feature(conservative_impl_trait)]

struct S {
    v: Vec<u32>,
}

impl S {
    //fn all<'a>(&'a self) -> impl Iterator<Item=&'a u32> {
    fn all(&self) -> impl Iterator<Item=&u32> {
        self.v.iter()
    }
}

fn main() {
    let s = S { v: Vec::new() };
    s.all().count();
}
