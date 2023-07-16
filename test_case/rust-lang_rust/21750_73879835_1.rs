 rust
pub trait Arg<A> {
    fn arg(&self);
}

pub trait Traversal {
    type Item;
    fn foreach<F: Arg<Self::Item>>(F);
}

impl<'a> Traversal for i32 {
    type Item = &'a i32;
    fn foreach<F: Arg<<Self as Traversal>::Item>>(f: F) {
        f.arg();
    }
}
