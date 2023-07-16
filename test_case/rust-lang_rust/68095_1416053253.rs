
error[E0271]: type mismatch resolving `<Iter<'_, {integer}> as IntoIterator>::Item == {integer}`
 --> src/main.rs:2:26
  |
2 |     let a = (0..3).chain([3, 4, 5].iter());
  |                    ----- ^^^^^^^^^^^^^^^^ expected integer, found `&{integer}`
  |                    |
  |                    required by a bound introduced by this call
  |
note: the method call chain might not have had the expected associated types
 --> src/main.rs:2:36
  |
2 |     let a = (0..3).chain([3, 4, 5].iter());
  |                          --------- ^^^^^^ `IntoIterator::Item` is `&{integer}` here
  |                          |
  |                          this expression has type `[{integer}; 3]`
note: required by a bound in `std::iter::Iterator::chain`
 --> /rustc/f3126500f25114ba4e0ac3e76694dd45a22de56d/library/core/src/iter/traits/iterator.rs:505:5
