rust
error[E0391]: cycle detected when const-evaluating `<impl at src/main.rs:9:1: 11:2>::Arr::{{constant}}`
  --> src/main.rs:10:21
   |
10 |     type Arr = [u8; Self::C];
   |                     ^^^^^^^
   |
note: ...which requires processing `<impl at src/main.rs:9:1: 11:2>::Arr::{{constant}}`...
  --> src/main.rs:10:21
   |
10 |     type Arr = [u8; Self::C];
   |                     ^^^^^^^
   = note: ...which again requires const-evaluating `<impl at src/main.rs:9:1: 11:2>::Arr::{{constant}}`, completing the cycle
note: cycle used when processing `<impl at src/main.rs:9:1: 11:2>`
  --> src/main.rs:9:1
   |
9  | impl Tr for str {
   | ^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
