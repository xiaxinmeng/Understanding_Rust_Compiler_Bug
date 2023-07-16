
warning: `extern` block uses type `(u32, u32)`, which is not FFI-safe
  --> packages/std/src/imports.rs:43:37
   |
43 |     fn db_next(iterator_id: u32) -> (u32, u32);
   |                                     ^^^^^^^^^^ not FFI-safe
   |
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider using a struct instead
   = note: tuples have unspecified layout
