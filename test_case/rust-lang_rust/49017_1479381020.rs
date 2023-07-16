
error[[E0277]](https://doc.rust-lang.org/stable/error_codes/E0277.html): the trait bound `for<'a> &'a <Self as Outer>::Inner: Encodable` is not satisfied
  --> src/main.rs:23:17
   |
23 |     type Inner: Inner;
   |                 ^^^^^ the trait `for<'a> Encodable` is not implemented for `&'a <Self as Outer>::Inner`
   |
   = help: the trait `Encodable` is implemented for `&'a HashMap<K, V, S>`
note: required by a bound in `Inner`
  --> src/main.rs:18:23
   |
16 | pub trait Inner
   |           ----- required by a bound in this
17 | where
18 |     for<'a> &'a Self: Encodable,
   |                       ^^^^^^^^^ required by this bound in `Inner`
