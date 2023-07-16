
error[E0720]: cannot resolve opaque type
 --> src/lib.rs:3:19
  |
3 | fn magic() -> Vec<impl Trait> {
  |                   ^^^^^^^^^^ cannot resolve opaque type
4 |     panic!()
  |     -------- this returned value is of `!` type
  |
  = help: this error will resolve once the item's body returns a concrete type
