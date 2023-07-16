
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: JoinPathsError { inner: JoinPathsError }', C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\libcore\result.rs:845
stack backtrace:
   0:     0x7ffea9fdfe70 - std::panicking::Location::line::h8c528bbdf4eef8a3
   1:     0x7ffea9fdf2f2 - std::panicking::Location::line::h8c528bbdf4eef8a3
   2:     0x7ffea9fe2d3d - std::panicking::rust_panic_with_hook::h7abc6e334345e341
   3:     0x7ffea9fe2b88 - std::panicking::begin_panic_fmt::h661e01bdd0e9618c
   4:     0x7ffea9fe2af4 - std::panicking::begin_panic_fmt::h661e01bdd0e9618c
   5:     0x7ffea9fe2a89 - rust_begin_unwind
   6:     0x7ffea9ff3d87 - core::panicking::panic_fmt::hf3abcb5bc11ff49d
   7:     0x7ffeaa240bd6 - <unknown>
   8:     0x7ffeaa32752c - rustc_driver::driver::count_nodes::h761a1dba824a4f1e
   9:     0x7ffeaa31b822 - rustc_driver::driver::count_nodes::h761a1dba824a4f1e
  10:     0x7ffeaa30f4b5 - rustc_driver::driver::compile_input::h44e43be0fc34a1a1
  11:     0x7ffeaa3658a7 - rustc_driver::run_compiler::h5e34a930c0929e5c
  12:     0x7ffeaa23e1ce - <unknown>
  13:     0x7ffea9fe5e01 - _rust_maybe_catch_panic
  14:     0x7ffeaa26c836 - <unknown>
  15:     0x7ffea9fdd33e - std::sys::imp::thread::Thread::new::h7a1b2b1b6bab390a
  16:     0x7ffede858363 - BaseThreadInitThunk
