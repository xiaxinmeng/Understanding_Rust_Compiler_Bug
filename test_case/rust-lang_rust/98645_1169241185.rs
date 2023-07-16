
error[E0277]: the trait bound `Thing: From<u32>` is not satisfied
  --> src/main.rs:21:33
   |
21 |     BigThing::new(value, other, thing.try_into().unwrap())
   |     -------------               ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<u32>` is not implemented for `Thing`
   |     |
   |     required by a bound introduced by this call
   |
   = note: required because of the requirements on the impl of `Into<Thing>` for `u32`
   = note: required because of the requirements on the impl of `TryFrom<u32>` for `Thing`
   = note: required because of the requirements on the impl of `TryInto<Thing>` for `u32`

For more information about this error, try `rustc --explain E0277`.
