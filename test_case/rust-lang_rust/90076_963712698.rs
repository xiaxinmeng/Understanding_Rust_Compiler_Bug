rust
impl<T> Iterable for Vec<T> {
    // with new syntax lhs and rhs wouldn't fit on the same line
    type SomeRatherAbsurdlyLongIdentBecauseReasonsAndFormattingFun<'a>
    where
        Self: 'a,
    = <&'a [T] as IntoIterator>::IntoIter;
}

impl ... {
    // what will we want to do with a type that can't fit on one line?
    type TRef<'a>
    where
        <North as Foo>::ABCDEFGH: 'a,
        <South as Foo>::ABCDEFGH: 'a,
        <East as Foo>::ABCDEFGH: 'a,
        <West as Foo>::ABCDEFGH: 'a,
        <NorthEast as Foo>::ABCDEFGH: 'a,
    = Any<
        &'a North::ABCDEFGH,
        &'a South::ABCDEFGH,
        &'a East::ABCDEFGH,
        &'a West::ABCDEFGH,
        &'a NorthEast::ABCDEFGH,
    >;
}

