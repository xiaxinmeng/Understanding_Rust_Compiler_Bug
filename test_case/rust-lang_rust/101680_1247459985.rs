rust
pub trait By<'a, C> {
    type Type;
}

pub struct Ref;

impl<'a, T: 'a> By<'a, Ref> for T {
    type Type = &'a T;
}

pub trait Transmit<T, C> {
    fn send<'a: 'a>(message: <T as By<'a, C>>::Type)
    where
        T: By<'a, C>;
}

impl<T> Transmit<T, Ref> for () {
    fn send<'a: 'a>(_message: <T as By<'a, Ref>>::Type) {}
}

fn main() {}
