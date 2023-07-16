
error[E0605]: non-primitive cast: `Enum` as `u8`
  --> src/main.rs:21:20
   |
21 |   println!("{:?}", Enum::Unit as u8);
   |                    ^^^^^^^^^^^^^^^^
   |
   = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
