
error[E0623]: lifetime mismatch
 --> src/main.rs:2:13
  |
1 | fn foo(mut first: &u8, second: &u8) {
  |                   ---          --- these two types are declared with different lifetimes...
2 |     first = second;
  |             ^^^^^^ ...but data from `second` flows into `first` here
  |
  = note: each elided lifetime in input position becomes a distinct lifetime
help: consider introducing a named lifetime parameter
  |
1 | fn foo<'a>(mut first: &'a u8, second: &'a u8) {
  |       ++++             ++              ++
