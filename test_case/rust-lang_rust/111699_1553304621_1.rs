
error[[E0308]](https://doc.rust-lang.org/nightly/error_codes/E0308.html): mismatched types
 --> src/main.rs:9:39
  |
9 |         assert_eq!(*arith_offset(ptr, true), 1);
  |                     ------------      ^^^^ expected `isize`, found `bool`
  |                     |
  |                     arguments to this function are incorrect
  |
note: function defined here
 --> /rustc/e9e1bbc7a820c472b39d3de54b3049bf14050655/library/core/src/intrinsics.rs:1457:12
