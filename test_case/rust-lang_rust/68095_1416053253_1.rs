
error[E0271]: type mismatch resolving `<Vec<{integer}> as IntoIterator>::Item == &{integer}`
 --> src/main.rs:2:30
  |
2 |     let _ = [1].iter().chain(vec![1]);
  |                        ----- ^^^^^^^ expected `&{integer}`, found integer
  |                        |
  |                        required by a bound introduced by this call
  |
note: required by a bound in `std::iter::Iterator::chain`
 --> /rustc/f3126500f25114ba4e0ac3e76694dd45a22de56d/library/core/src/iter/traits/iterator.rs:505:5
