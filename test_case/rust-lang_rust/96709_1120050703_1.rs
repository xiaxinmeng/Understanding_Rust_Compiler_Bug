rust
pub trait Gat<'a> {
    type Item<T>;
    fn f<T>(&self) -> &'a <Self as Gat::<'a>>::Item<T>;
}
