rust
pub trait Change<T> {
    type Changed;
}

impl<T> Change<T> for () {
    type Changed = T;
}

pub trait Split {
    type First;
    type Rest;
}

pub trait Combine<T> {}

impl<U> Combine<()> for U {}

impl<T, U> Combine<U> for T
where
    U: Split,
    Self: Change<U::First>,
    <Self as Change<U::First>>::Changed: Combine<U::Rest>,
{
}

pub fn combine<T>(_: T)
where
    (): Combine<T>,
{
}
