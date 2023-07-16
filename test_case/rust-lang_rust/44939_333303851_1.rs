
error[E0308]: mismatched types
 --> src/main.rs:6:19
  |
6 |     types_eq(foo, bar);
  |                   ^^^ expected fn item, found a different fn item
  |
  = note: expected type `fn(u8) -> u8 {main::foo}`
             found type `fn(u8) -> u8 {main::bar}`
