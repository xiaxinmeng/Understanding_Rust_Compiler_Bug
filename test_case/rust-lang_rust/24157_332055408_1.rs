
error[E0308]: mismatched types
 --> src/main.rs:8:40
  |
5 | fn is_one(a: u8) {
  |                  - help: possibly return type missing here?: `-> bool `
...
8 |         _ => {panic!("It's not one!"); false},
  |                                        ^^^^^ expected (), found bool
  |
  = note: expected type `()`
             found type `bool`
