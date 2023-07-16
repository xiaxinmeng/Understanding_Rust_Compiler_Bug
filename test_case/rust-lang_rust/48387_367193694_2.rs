
error: internal compiler error: Error constructed but not emitted

thread 'rustc' panicked at 'explicit panic', librustc_errors/diagnostic_builder.rs:242:13
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:380
   3: std::panicking::default_hook
             at libstd/panicking.rs:396
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:576
   5: std::panicking::begin_panic
   6: <rustc_errors::diagnostic_builder::DiagnosticBuilder<'a> as core::ops::drop::Drop>::drop
   7: syntax::ext::expand::MacroExpander::expand_invoc
   8: syntax::ext::expand::MacroExpander::expand
   9: syntax::ext::expand::MacroExpander::expand_crate
  10: rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}}
  11: rustc::util::common::time
  12: rustc_driver::driver::phase_2_configure_and_expand
  13: rustc_driver::driver::compile_input
  14: rustc_driver::run_compiler
