
error[E0308]: mismatched types
 --> src/main.rs:3:9
  |
3 |     x = |c| c + 1;
  |         ^^^^^^^^^ expected closure, found a different closure
  |
  = note: expected type `[closure@src/main.rs:2:17: 2:26]`
             found type `[closure@src/main.rs:3:9: 3:18]`
note: no two closures, even if identical, have the same type
 --> src/main.rs:3:9
  |
3 |     x = |c| c + 1;
  |         ^^^^^^^^^
help: consider boxing your closure and/or using it as a trait object
 --> src/main.rs:3:9
  |
3 |     x = |c| c + 1;
  |         ^^^^^^^^^
