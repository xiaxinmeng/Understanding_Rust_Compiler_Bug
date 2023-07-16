rust
error[[E0597]](https://doc.rust-lang.org/stable/error-index.html#E0597): `get_to` does not live long enough
 --> src/lib.rs:8:5
  |
7 | fn test_fun<'b, T>(get_to: T) where T: Get<T<'b> = T> + 'b {
  |             -- lifetime `'b` defined here
8 |     get_to.get(); // So this could be called with something besides &'b get_to
  |     ^^^^^^^^^^^^
  |     |
  |     borrowed value does not live long enough
  |     argument requires that `get_to` is borrowed for `'b`
9 | }
  | - `get_to` dropped here while still borrowed
