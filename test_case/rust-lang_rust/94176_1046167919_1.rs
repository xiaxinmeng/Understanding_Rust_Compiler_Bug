
error[E0308]: mismatched types
 --> src/main.rs:3:32
  |
3 | pub fn test(a: Option<u32>) -> Option<u32> {
  |        ----                    ^^^^^^^^^^^ expected enum `Option`, found `()`
  |        |
  |        implicitly returns `()` as its body has no tail or `return` expression
  |
  = note:   expected enum `Option<u32>`
          found unit type `()`
