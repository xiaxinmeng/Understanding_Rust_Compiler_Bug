
error[E0741]: `Nat` must be annotated with `#[derive(PartialEq, Eq)]` to be used as the type of a const parameter
 --> src/lib.rs:9:17
  |
9 | fn foo<const N: Nat>() {}
  |                 ^^^ `Nat` doesn't derive both `PartialEq` and `Eq`
