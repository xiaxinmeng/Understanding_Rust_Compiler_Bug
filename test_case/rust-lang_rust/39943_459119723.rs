
error[E0597]: `x` does not live long enough
 --> src/main.rs:4:8
  |
4 |     f(&x);
  |        ^ borrowed value does not live long enough
5 | }
  | - `x` dropped here while still borrowed
  |
  = note: values in a scope are dropped in the opposite order they are created
