
error[E0308]: mismatched types
 --> src/main.rs:6:9
  |
6 |     x = bar;
  |         ^^^ expected fn item, found a different fn item
  |
  = note: expected type `fn() {foo}`
             found type `fn() {bar}
