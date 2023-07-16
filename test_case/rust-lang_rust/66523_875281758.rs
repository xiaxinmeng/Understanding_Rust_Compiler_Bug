
error[E0720]: cannot resolve opaque type
 --> src/lib.rs:1:17
  |
1 | fn bar() -> Vec<impl Copy> {
  |                 ^^^^^^^^^ cannot resolve opaque type
2 |     panic!()
  |     -------- this returned value is of `!` type
  |
  = help: this error will resolve once the item's body returns a concrete type
