
INFO:rustdoc: finished with rustc
INFO:rustdoc: loading plugins...
INFO:rustdoc: Executing passes/plugins
INFO:rustdoc: going to format
INFO:rustdoc::html::render: emitting source files
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.24.0-dev running on x86_64-pc-windows-msvc

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 18446744073709551615', D:\rust-lang\rust\src\liballoc\vec.rs:1551:10
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys_common::backtrace::_print
   1: std::panicking::Location::column
   2: std::panicking::Location::column
   3: std::panicking::rust_panic_with_hook
   4: std::panicking::begin_panic
   5: std::panicking::begin_panic_fmt
   6: rust_begin_unwind
   7: core::panicking::panic_fmt
   8: core::panicking::panic_bounds_check
   9: rustdoc::html::render::get_index_type_name
  10: rustdoc::html::render::get_index_type
  11: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  12: <rustdoc::html::render::Cache as rustdoc::fold::DocFolder>::fold_item
  13: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  14: <rustc::ty::VariantDef as rustdoc::clean::Clean<rustdoc::clean::Item>>::clean
  15: <rustdoc::html::render::Cache as rustdoc::fold::DocFolder>::fold_item
  16: rustdoc::html::render::run


------------------------------------------

thread '[rustdoc] rustdoc\issue-46976.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2776:8
