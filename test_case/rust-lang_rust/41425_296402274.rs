rust
error[E0308]: mismatched types
  --> file.rs:11:28
   |
11 |   fn plus_one(x: i32) -> i32 {
   |  ____________________________^
12 | |     x + 1;
13 | | }
   | |_^ expected (), found i32
   |
   = note: expected type `i32`
              found type `()`
