rust
  error[E0606]: casting `usize` as `*const T` is invalid
     --> library/core/src/ptr/mod.rs:210:5
      |
  210 |     0 as *const T
      |     ^^^^^^^^^^^^^
  
  error[E0606]: casting `usize` as `*mut T` is invalid
     --> library/core/src/ptr/mod.rs:228:5
      |
  228 |     0 as *mut T
      |     ^^^^^^^^^^^
  