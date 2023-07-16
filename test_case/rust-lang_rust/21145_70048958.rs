 rust
pub trait Index<Index: ?Sized> {
    type Output: ?Sized;

    fn index<'a>(&'a self, index: &Index) -> &'a Output;
}
