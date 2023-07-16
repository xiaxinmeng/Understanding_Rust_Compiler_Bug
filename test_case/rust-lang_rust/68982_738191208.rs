
error[E0412]: cannot find type `A` in this scope
 --> src/lib.rs:9:41
  |
6 |     fn from_iter<T>(_: T) -> Self
  |                  - similarly named type parameter `T` defined here
...
9 |         std::iter::IntoIterator::Item = A,
  |                                         ^ help: a type parameter with a similar name exists: `T`
