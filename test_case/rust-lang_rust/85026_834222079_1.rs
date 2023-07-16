
error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
 --> src/lib.rs:3:1
  |
3 | impl dyn Copy {}
  | ^^^^^^^^^^^^^^^^ impl for type defined outside of crate.
  |
  = note: define and implement a trait or new type instead

error[E0118]: no nominal type found for inherent implementation
 --> src/lib.rs:4:6
  |
4 | impl dyn UnwindSafe {}
  |      ^^^^^^^^^^^^^^ impl requires a nominal type
  |
  = note: either implement a trait on it or create a newtype to wrap it instead
