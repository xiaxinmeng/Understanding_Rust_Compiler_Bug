
error: `Nat` is forbidden as the type of a const generic parameter
 --> src/lib.rs:7:17
  |
7 | fn foo<const N: Nat>() {}
  |                 ^^^
  |
  = note: the only supported types are integers, `bool` and `char`
  = help: more complex types are supported with `#![feature(adt_const_params)]`
