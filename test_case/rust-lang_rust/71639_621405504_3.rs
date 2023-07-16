
thread 'rustc' panicked at 'index out of bounds: the len is 2 but the index is 2', src/librustc_span/hygiene.rs:181:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1504
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:515
  12: rust_begin_unwind
             at src/libstd/panicking.rs:419
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:111
  14: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:69
  15: rustc_span::hygiene::HygieneData::expn_data
  16: rustc_span::hygiene::HygieneData::with
  17: rustc_codegen_ssa::back::write::SharedEmitterMain::check
  18: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::join_codegen
  19: rustc_interface::queries::Linker::link
  20: rustc_interface::interface::run_compiler_in_existing_thread_pool
  21: scoped_tls::ScopedKey<T>::set
  22: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (b2e36e6c2 2020-04-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C lto -C codegen-units=1 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `asm_error`.
