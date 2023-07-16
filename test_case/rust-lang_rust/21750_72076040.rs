 rust
pub trait Arg<A> { }

pub trait Traversal {
    type Item;
    fn foreach<F: Arg<Self::Item>>(F);
}

impl<'a> Traversal for i32 {
    type Item = &'a i32;
    fn foreach<F: Arg<&'a i32>>(f: F) { }
}

fn main() {}
