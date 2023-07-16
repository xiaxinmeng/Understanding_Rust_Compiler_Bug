
error[E0428]: the name `__breakpoint` is defined multiple times
  --> crates/core_arch/src/arm/armclang.rs:49:1
   |
22 | pub unsafe fn __breakpoint<const VAL: i32>() {
   | -------------------------------------------- previous definition of the value `__breakpoint` here
...
49 | pub unsafe fn __breakpoint<const VAL: i32>() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `__breakpoint` redefined here
   |
   = note: `__breakpoint` must be defined only once in the value namespace of this module
