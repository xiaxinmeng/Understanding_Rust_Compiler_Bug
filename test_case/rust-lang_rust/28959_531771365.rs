
error[E0606]: casting `&for<'r> fn(&'r usize) -> usize {heap_size}` as `*const for<'r> fn(&'r usize) -> usize` is invalid
 --> src/main.rs:5:45
  |
5 | const f: *const fn(*const usize) -> usize = (&heap_size as *const fn(&usize) -> usize) as *const fn(*const usize) -> usize;
  |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

