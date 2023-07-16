
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
 --> src/lib.rs:4:5
  |
4 |     core::intrinsics::transmute(value)
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: source type: `T` (this type does not have a fixed size)
  = note: target type: `U` (this type does not have a fixed size)
