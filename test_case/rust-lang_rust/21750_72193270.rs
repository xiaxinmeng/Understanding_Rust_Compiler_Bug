 rust
pub trait Traversal {
    type Item;

    fn foreach<F>(self, F) where F: FnMut(Self::Item);
}

impl<'a, T> Traversal for &'a [T] {
    type Item = &'a T;

    fn foreach<F>(self, mut f: F) where F: FnMut(&'a T) {
        //
    }
}
