
error[E0308]: mismatched types
 --> src/lib.rs:3:9
  |
2 | /     if x {
3 | |         1
  | |         - expected `()`, found integer
4 | |     } else {
5 | |         2
  | |         - expected `()`, found integer
6 | |     }
  | |_____^ expected `()`, found integer
  |
help: you might have meant to return this value
  |
2 ~     return if x {
3 |         1
4 |     } else {
5 |         2
6 ~     };
  |
