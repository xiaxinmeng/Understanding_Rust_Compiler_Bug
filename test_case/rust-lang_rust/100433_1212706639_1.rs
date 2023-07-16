
error[[E0277]](https://doc.rust-lang.org/stable/error-index.html#E0277): the trait bound `std::collections::hash_map::Iter<'_, u32, u32>: DoubleEndedIterator` is not satisfied
 --> src/lib.rs:5:24
  |
5 |     let _ = map.iter().rev();
  |                        ^^^ the trait `DoubleEndedIterator` is not implemented for `std::collections::hash_map::Iter<'_, u32, u32>`
  |
note: required by a bound in `rev`

For more information about this error, try `rustc --explain E0277`.
