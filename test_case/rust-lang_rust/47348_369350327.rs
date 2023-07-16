
   Compiling playground v0.0.1 (file:///playground)
error[E0667]: `impl Trait` is not allowed in path parameters
 --> src/main.rs:9:31
  |
9 | fn test0(foo: impl Trait) -> <impl Trait as std::ops::Sub>::Output {
  |                               ^^^^^^^^^^
