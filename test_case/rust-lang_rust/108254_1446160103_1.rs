text
error[[E0277]](https://doc.rust-lang.org/nightly/error_codes/E0277.html): `{integer}` is not an iterator
 --> src/lib.rs:9:17
  |
9 |     want_fancy(&Some(5));
  |     ----------  ^^^^^^^ `{integer}` is not an iterator
  |     |
  |     required by a bound introduced by this call
  |
  = help: the trait `Iterator` is not implemented for `{integer}`
  = note: if you want to iterate between `start` until a value `end`, use the exclusive range syntax `start..end` or the inclusive range syntax `start..=end`
  = help: the trait `Fancy` is implemented for `&'a T`
note: required for `Option<{integer}>` to implement `Good`
 --> src/lib.rs:4:10
  |
4 | impl <S> Good for Option<S> where S: Iterator {}
  |          ^^^^     ^^^^^^^^^          -------- unsatisfied trait bound introduced here
note: required for `&Option<{integer}>` to implement `Fancy`
 --> src/lib.rs:3:14
  |
3 | impl <'a, T> Fancy for &'a T where T: Good {}
  |              ^^^^^     ^^^^^          ---- unsatisfied trait bound introduced here
note: required by a bound in `want_fancy`
 --> src/lib.rs:6:33
  |
6 | fn want_fancy<F>(f: F) where F: Fancy {}
  |                                 ^^^^^ required by this bound in `want_fancy`
