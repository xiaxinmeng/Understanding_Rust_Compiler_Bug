
error[E0308]: mismatched types
 --> file.rs:2:16
  |
2 | const D: i32 = 1usize;
  |                ^^^^^^ expected i32, found usize

error[E0308]: mismatched types
 --> file.rs:3:16
  |
3 | const E: i32 = C;
  |                ^ expected i32, found usize

error[E0308]: mismatched types
 --> file.rs:4:15
  |
4 | const G: i8 = 1usize;
  |               ^^^^^^ expected i8, found usize

error[E0308]: mismatched types
 --> file.rs:5:15
  |
5 | const H: i8 = C;
  |               ^ expected i8, found usize

error[E0308]: mismatched types
 --> file.rs:8:18
  |
8 |     let c: i32 = 1i8;
  |                  ^^^ expected i32, found i8
help: change the type of the numeric literal from `i8` to `i32`
  |
8 |     let c: i32 = 1i32;
  |                  ^^^^

error[E0308]: mismatched types
 --> file.rs:9:17
  |
9 |     let x: u8 = C;
  |                 ^ expected u8, found usize
help: you can convert an `usize` to `u8` or panic if it the converted value wouldn't fit
  |
9 |     let x: u8 = C.try_into().unwrap();
  |                 ^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> file.rs:10:17
   |
10 |     let x: u8 = D;
   |                 ^ expected u8, found i32
help: you can convert an `i32` to `u8` or panic if it the converted value wouldn't fit
   |
10 |     let x: u8 = D.try_into().unwrap();
   |                 ^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
  --> file.rs:11:17
   |
11 |     let x: u8 = E;
   |                 ^ expected u8, found i32
help: you can convert an `i32` to `u8` or panic if it the converted value wouldn't fit
   |
11 |     let x: u8 = E.try_into().unwrap();
   |                 ^^^^^^^^^^^^^^^^^^^^^
