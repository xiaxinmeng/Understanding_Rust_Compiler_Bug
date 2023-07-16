plain
   Compiling libffi v3.2.0
error[E0599]: no method named `base_addr` found for enum `std::option::Option` in the current scope
  --> src/tools/miri/src/shims/ffi_support.rs:82:38
   |
82 |                     let addr = alloc.base_addr();
   |                                      ^^^^^^^^^ method not found in `Option<AllocRef<'_, '_, Provenance, AllocExtra>>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `miri` due to previous error
error: could not compile `miri` due to previous error
thread 'main' panicked at 'in-tree tool', test.rs:599:14
Build completed unsuccessfully in 0:00:13
