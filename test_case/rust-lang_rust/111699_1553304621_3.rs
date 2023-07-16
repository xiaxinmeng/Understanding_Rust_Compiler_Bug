
error[[E0308]](https://doc.rust-lang.org/nightly/error_codes/E0308.html): mismatched types
 --> src/main.rs:5:23
  |
5 |     unsafe { floorf32(true); }
  |              -------- ^^^^ expected `f32`, found `bool`
  |              |
  |              arguments to this function are incorrect
  |
note: function defined here
 --> /rustc/e9e1bbc7a820c472b39d3de54b3049bf14050655/library/core/src/intrinsics.rs:1749:12
