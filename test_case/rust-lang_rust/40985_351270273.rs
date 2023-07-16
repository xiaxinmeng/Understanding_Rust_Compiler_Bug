
error[E0605]: non-primitive cast: `{float}` as `u32`
 --> src/main.rs:3:22
  |
3 |     println!("{:?}", 2.0 as u32);
  |                      ^^^^^^^^^^
  |
  = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
