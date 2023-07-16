
error[E0080]: constant evaluation error: no mir for `std::rt::lang_start_internal`
  --> /Users/eric/Proj/rust/rust/src/libstd/rt.rs:74:5
   |
74 |     lang_start_internal(&move || main().report(), argc, argv)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no mir for `std::rt::lang_start_internal`
   |
   = note: inside call to `std::rt::lang_start::<()>`
