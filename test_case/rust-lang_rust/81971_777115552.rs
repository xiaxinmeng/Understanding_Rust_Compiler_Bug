
error[E0308]: mismatched types
 --> src/lib.rs:2:18
  |
1 | fn test<T, i32: From<T>>(t: T) -> i32 {
  |            --- this type parameter
2 |     let x: i32 = 0i32;
  |            ---   ^^^^ expected type parameter `i32`, found `i32`
  |            |
  |            expected due to this
  |
  = note: expected type parameter `i32` (type parameter `i32`)
                       found type `i32` (`i32`)
