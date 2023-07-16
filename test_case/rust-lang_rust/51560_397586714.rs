rust
    fn x() -> impl Iterator { [42].iter() }
    fn go_backwards<T: DoubleEndedIterator>(_: T) {}
    go_backwards(x());
    