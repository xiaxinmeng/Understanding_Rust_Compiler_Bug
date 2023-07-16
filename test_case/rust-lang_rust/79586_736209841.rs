
    Compiling test v0.0.0 (/checkout/library/test)
error: unused attribute
  --> library/test/src/lib.rs:20:1
   |
20 | #![crate_name = "test"]
   | ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-attributes` implied by `-D warnings`

error: crate-level attribute should be in the root module
  --> library/test/src/lib.rs:20:1
   |
20 | #![crate_name = "test"]
   | ^^^^^^^^^^^^^^^^^^^^^^^
