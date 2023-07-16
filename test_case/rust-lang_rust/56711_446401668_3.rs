
error: reached the type-length limit while instantiating `repeat::<[closure@src/main.rs:6:27: 6:33 f:&[closure@src/main.rs...`                                                                           
 --> src/main.rs:1:1
  |
1 | / fn repeat(n: i64, f: impl Fn()) {
2 | |     if n > 0 {
3 | |         f();
4 | |         for _ in 0..n {
... |
8 | |     }
9 | | }
  | |_^
  |
  = note: consider adding a `#![type_length_limit="2097152"]` attribute to your crate
