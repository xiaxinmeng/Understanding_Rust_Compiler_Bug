text
error[E0283]: type annotations needed for `Container`
  --> src/main.rs:28:7
   |
22 |     let c = Container { i: 42, b: true };
   |         - consider giving `c` a type
...
28 |     c.as_ref() == &42u8;
   |       ^^^^^^ cannot infer type for struct `Container`
   |
   = note: cannot resolve `Container: std::convert::AsRef<_>`
