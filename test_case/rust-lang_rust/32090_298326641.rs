
error[E0308]: mismatched types
  --> src/utils.rs:84:9
   |
84 |         assert!(num, bools_to_byte(byte_to_bools(num)));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected bool, found u8
   |
   = note: this error originates in a macro outside of the current crate
