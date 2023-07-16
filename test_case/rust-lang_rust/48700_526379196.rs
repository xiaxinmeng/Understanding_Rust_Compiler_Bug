`error: failed to remove /mnt/d/devs/Rust/bootloader/target/x86_64-bootloader/release/deps/bootloader-589b595378e59014.bootloader.3pdwijpa-cgu.0.rcgu.o: Input/output error (os error 5)

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:540:13
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::emit_db
  10: rustc_errors::Handler::err
  11: rustc_codegen_ssa::back::link::remove
  12: rustc_codegen_ssa::back::link::link_binary
  13: rustc::util::common::time
  14: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
  15: rustc_interface::queries::Query<T>::compute
  16: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::link
  17: rustc_interface::interface::run_compiler_in_existing_thread_pool
  18: std::thread::local::LocalKey<T>::with
  19: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (760226733 2019-08-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug -C opt-level=3 -C panic=abort -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden
