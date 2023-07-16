
error[E0605]: non-primitive cast: `S<0_usize>` as `*const ()`
 --> src/main.rs:3:5
  |
3 |     S::<0> as *const ();
  |     ^^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
  