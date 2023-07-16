
4: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   5: std::panicking::begin_panic
   6: compiletest::runtest::ProcRes::fatal
   7: compiletest::runtest::TestCx::fatal_proc_rec
   8: compiletest::runtest::TestCx::run_revision
   9: compiletest::runtest::run
  10: <F as alloc::boxed::FnBox<A>>::call_box
  11: <F as alloc::boxed::FnBox<A>>::call_box
             at src/libtest/lib.rs:1490
             at /rustc/744b374ab6197d7d06994b06a9598bbd3210efce/src/liballoc/boxed.rs:749
  12: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:87
  13: test::run_test::run_test_inner::{{closure}}
             at /rustc/744b374ab6197d7d06994b06a9598bbd3210efce/src/libstd/panicking.rs:276
             at /rustc/744b374ab6197d7d06994b06a9598bbd3210efce/src/libstd/panic.rs:388
             at src/libtest/lib.rs:1452
