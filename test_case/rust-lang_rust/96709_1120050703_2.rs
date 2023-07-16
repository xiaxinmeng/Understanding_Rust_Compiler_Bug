rust
pub trait Gat {
    type Item<T>;
    fn f<T>(&self) -> <Self as Gat>::Item<T> where Self::Item<T>: Default {
        <_>::default()
    }
}
