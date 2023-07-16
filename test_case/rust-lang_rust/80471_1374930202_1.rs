
error[E0741]: `Box<Nat>` must be annotated with `#[derive(PartialEq, Eq)]` to be used as the type of a const parameter
 --> src/lib.rs:8:17
  |
8 | fn foo<const N: Nat>() {}
  |                 ^^^
