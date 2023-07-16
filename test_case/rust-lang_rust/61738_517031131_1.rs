
error[E0223]: ambiguous associated type
 --> src/lib.rs:6:12
  |
6 |     inner: I::Item::IntoIter,
  |            ^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<<I as std::iter::Iterator>::Item as Trait>::IntoIter`
