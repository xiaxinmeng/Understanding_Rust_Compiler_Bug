
error: casting `*const T` as `usize` is invalid
  --> src/refset.rs:17:9
   |
17 |         (self.0 as *const T as usize).hash(state);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: `*const T` may include non-zero sized metadata, such as a slice length or `dyn Trait` vtable
   = help: cast through a pointer that has no metadata first:
             (self.0 as *const T as *const () as usize).hash(state);
