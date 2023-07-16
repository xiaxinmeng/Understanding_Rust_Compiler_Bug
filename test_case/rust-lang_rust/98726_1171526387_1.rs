
error[E0582]: binding for associated type `Output` references an anonymous lifetime, which does not appear in the trait input types
 --> src/lib.rs:3:20
  |
3 |     F: FnOnce() -> Box<FnOnce(&())>,
  |                    ^^^^^^^^^^^^^^^^
  |
  = note: lifetimes appearing in an associated type are not considered constrained
