
conflicting implementations of trait `std::iter::FromIterator<std::result::Result<_, MyError>>` for type
`std::result::Result<MyArray<std::result::Result<_, MyError>, {_: usize}>, MyError>`
conflicting implementation in crate `core`:
- impl<A, E, V> FromIterator<Result<A, E>> for Result<V, E>
  where V: FromIterator<A>;
  