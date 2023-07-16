
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
1 | fn foo() {
  |          - help: try adding a return type: `-> u8`
2 |     0u8
  |     ^^^ expected return type (), found u8
...
6 |     let _: u8 = foo();
  |                 ----- expected u8 here as well
  |
  = note: expected type `()`
             found type `u8`
