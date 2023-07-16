
error: constant expression depends on a generic parameter
 --> enum.rs:4:12
  |
4 |     Some = std::mem::size_of::<T>(),
  |            ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this may fail depending on what value the parameter takes
