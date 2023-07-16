`
  Compiling crashtest v0.1.0 (/tmp/crashtest)
thread 'rustc' panicked at 'assertion failed: !are_upstream_rust_objects_already_included(sess)', src/librustc_codegen_llvm/back/link.rs:1402:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:70
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:58
             at src/libstd/panicking.rs:200
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:215
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:482
   6: std::panicking::begin_panic
   7: rustc_codegen_llvm::back::link::link_natively
   8: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link::{{closure}}
   9: rustc::util::common::time
  10: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
  11: rustc_driver::driver::compile_input
  12: rustc_driver::run_compiler_with_pool
  13: <scoped_tls::ScopedKey<T>>::set
  14: rustc_driver::run_compiler
  15: <scoped_tls::ScopedKey<T>>::set
  16: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  17: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:92
  18: <F as alloc::boxed::FnBox<A>>::call_box
  19: std::sys::unix::thread::Thread::new::thread_start
             at /rustc/9eac386342c601b14311b435f2b6d314fc817bb5/src/liballoc/boxed.rs:734
             at src/libstd/sys_common/thread.rs:14
             at src/libstd/sys/unix/thread.rs:81
  20: start_thread
  21: clone
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-nightly (9eac38634 2018-12-31) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C lto -C target-cpu=native --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `crashtest`.

To learn more, run the command again with --verbose.
