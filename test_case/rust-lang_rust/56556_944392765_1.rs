rust
use std::ops::Deref;

fn test<T, TDeref>()
where
    T: Deref<Target = TDeref>,
    TDeref: ?Sized,
    for<'a> &'a TDeref: IntoIterator,
    for<'a> <&'a TDeref as IntoIterator>::IntoIter: Clone,
{
}

fn main() {
    test::<Vec<u8>, _>();
}
