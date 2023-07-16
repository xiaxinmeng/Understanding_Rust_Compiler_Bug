
error[E0597]: `x` does not live long enough
 --> src/main.rs:5:21
  |
5 |     func(|_: &i32| &x);
  |          --------- -^
  |          |         ||
  |          |         |borrowed value does not live long enough
  |          |         returning this value requires that `x` is borrowed for `'static`
  |          value captured here
6 | }
  | - `x` dropped here while still borrowed
