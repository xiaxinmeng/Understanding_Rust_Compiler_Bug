rust
pub trait Gat {
    type Item<T>;
    fn f<T>(&self) -> Self::Item<T> where Self: Copy;
}
