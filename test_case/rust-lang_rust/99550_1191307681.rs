plain
[RUSTC-TIMING] addr2line test:false 0.543
[RUSTC-TIMING] core test:false 31.724
[RUSTC-TIMING] gimli test:false 5.569
[RUSTC-TIMING] object test:false 5.824
error[E0252]: the name `ZX_TIME_INFINITE` is defined multiple times
  --> library/std/src/sys/unix/locks/fuchsia_mutex.rs:47:23
   |
47 |     ZX_TIME_INFINITE, ZX_TIME_INFINITE,
   |     ----------------  ^^^^^^^^^^^^^^^^-
   |     |                 |
   |     |                 `ZX_TIME_INFINITE` reimported here
   |     |                 help: remove unnecessary import
   |     previous import of the value `ZX_TIME_INFINITE` here
   |
   = note: `ZX_TIME_INFINITE` must be defined only once in the value namespace of this module

error: unused import: `ZX_TIME_INFINITE`
  --> library/std/src/sys/unix/locks/fuchsia_mutex.rs:47:23
   |
47 |     ZX_TIME_INFINITE, ZX_TIME_INFINITE,
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0252`.
[RUSTC-TIMING] std test:false 2.890
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:13:30
