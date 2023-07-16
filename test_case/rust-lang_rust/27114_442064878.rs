
   Compiling playground v0.0.1 (/playground)
error[E0477]: the type `&'b i32` does not fulfill the required lifetime
 --> src/main.rs:4:5
  |
4 |     foo::<&'b i32>();
  |     ^^^^^^^^^^^^^^
  |
  = note: type must satisfy the static lifetime
