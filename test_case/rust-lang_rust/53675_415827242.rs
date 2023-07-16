rust
error: cannot determine resolution for the macro `panic`
  --> tests/panic.rs:46:9
   |
46 |         panic!()
   |         ^^^^^
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the macro `panic`
  --> tests/panic.rs:37:9
   |
37 |         assert!(!function.is_null());
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: import resolution is stuck, try simplifying macro imports

error: aborting due to 2 previous errors