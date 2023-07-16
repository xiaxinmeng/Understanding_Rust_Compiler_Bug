Rust
#![feature(generic_associated_types)]

trait Iter {
    type Item<'a>;
    
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iter for () {
    type Item<'s> = &'s Self;
    
    fn next(&mut self) -> Option<Self::Item> {
        Some(self)
    }
}

fn main() {
}
