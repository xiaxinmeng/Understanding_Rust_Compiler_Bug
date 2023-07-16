
error[E0308]: mismatched types
 --> src/main.rs:5:24
  |
5 |     let _fref: &fn() = &foo; //~ERROR expected fn pointer, found fn item
  |                        ^^^^ expected fn pointer, found fn item
  |
  = note: expected type `&fn()`
             found type `&fn() {foo}`
