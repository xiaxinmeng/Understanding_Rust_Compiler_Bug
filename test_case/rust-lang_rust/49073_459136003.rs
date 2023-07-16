
error[E0597]: `i` does not live long enough
 --> src/main.rs:6:14
  |
6 |     let b = &i;
  |              ^ borrowed value does not live long enough
7 |     assert_eq!("", to_string(&b));
8 | }
  | - `i` dropped here while still borrowed
  |
  = note: values in a scope are dropped in the opposite order they are created
