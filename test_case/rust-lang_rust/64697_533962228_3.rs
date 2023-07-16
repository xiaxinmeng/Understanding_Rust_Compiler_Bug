text
error[E0597]: `x` does not live long enough
 --> src/lib.rs:7:9
  |
7 |     foo(&x, &(baz as _));
  |     ----^^--------------
  |     |   |
  |     |   borrowed value does not live long enough
  |     argument requires that `x` is borrowed for `'static`
8 | }
  | - `x` dropped here while still borrowed
