rust
// OK
pub trait NonGat<'a, 'b> {
    type Item;
    fn f<'d>(_: &'b &'d Self) -> &'a &'b <Self as NonGat<'a, 'b>>::Item;
}

// Must add `where Self: 'b` currently
pub trait Gat<'a, 'b> {
    type Item<'c>;
    fn f<'d>(_: &'b &'d Self) -> &'a &'b <Self as Gat<'a, 'b>>::Item<'static>;
}
