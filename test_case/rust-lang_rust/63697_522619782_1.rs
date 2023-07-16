
   Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:5:20
  |
5 |     let m = Member(i32::from(0_u8));
  |                    ^^^^^^^^^^^^^^^ expected u16, found i32
help: you can convert an `i32` to `u16` and panic if the converted value wouldn't fit
  |
5 |     let m = Member(i32::from(0_u8).try_into().unwrap());
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
