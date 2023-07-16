
error[E0308]: mismatched types
 --> src/main.rs:3:9
  |
3 |     x = |c| c + 1;
  |         ^^^^^^^^^ expected a closure, found a different closure
  |
  = note: expected type `|_| -> _`
             found type `|_| -> _`
note: no two closures, even if identical, have the same type
 --> src/main.rs:3:9
  |
2 |     let mut x = |c| c + 1;
  |                 --------- this was the expected closure
3 |     x = |c| c + 1;
  |         ^^^^^^^^^ but this closure was found
  = help: consider boxing your closure and/or using it as a trait object
