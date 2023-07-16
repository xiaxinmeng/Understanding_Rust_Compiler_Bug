
error[E0308]: mismatched types
 --> src/main.rs:3:9
  |
2 |     let mut x = |c| c + 1;
  |                 --------- this closure was expected
3 |     x = |c| c + 1;
  |         ^^^^^^^^^ expected closure, found a different closure
  |
  = note: expected type `[closure]`
             found type `[closure]`
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
