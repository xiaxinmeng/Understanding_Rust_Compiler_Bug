rust
/// Note: the `Item` type parameter is not used in this trait,
/// but it allows impls to be more generic.
pub trait Concat<Item: ?Sized> {
    type Output;
    fn concat(slice: &Self) -> Self::Output;
}

pub trait Join<Separator> {
    type Output;
    fn join(slice: &Self, sep: Separator) -> Self::Output;
}

impl<T: Clone, V: Borrow<[T]>> Concat<T> for [V] {
    type Output = Vec<T>;
    fn concat(slice: &Self) -> Vec<T> { … }
}

impl<T: Clone, V: Borrow<[T]>> Join<&T> for [V] {
    type Output = Vec<T>;
    fn join(slice: &Self, sep: &T) -> Vec<T> { … }
}

impl<T: Clone, V: Borrow<[T]>> Join<&[T]> for [V] {
    type Output = Vec<T>;
    fn join(slice: &Self, sep: &[T]) -> Vec<T> { … }
}

impl<S: Borrow<str>> Concat<str> for [S] {
    type Output = String;
    fn concat(slice: &Self) -> String { … }
}

impl<S: Borrow<str>> Join<&str> for [S] {
    type Output = String;
    fn join(slice: &Self, sep: &str) -> String { … }
}
