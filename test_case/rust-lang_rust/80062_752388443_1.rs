
error[E0308]: mismatched types
 --> src/main.rs:4:20
  |
4 | fn sof<usize>() -> usize {}
  |    --- -----       ^^^^^ expected type parameter `usize`, found `()`
  |    |   |
  |    |   this type parameter
  |    implicitly returns `()` as its body has no tail or `return` expression
  |
  = note: expected type parameter `usize`
                  found unit type `()`

error[E0308]: mismatched types
 --> src/main.rs:6:17
  |
5 | fn test<T>() {
  |         - this type parameter
6 |     let _: [u8; sof::<T>()];
  |                 ^^^^^^^^^^ expected `usize`, found type parameter `T`
  |
  = note:        expected type `usize`
          found type parameter `T`

error: aborting due to 2 previous errors; 1 warning emitted
