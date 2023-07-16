
error: an inner attribute is not permitted in this context
  |
  = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.

thread 'rustc' panicked at 'internal error: entered unreachable code', libsyntax/ext/expand.rs:258:18
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at libstd/sys_common/backtrace.rs:59
             at libstd/panicking.rs:380
   3: std::panicking::default_hook
             at libstd/panicking.rs:396
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:576
   5: std::panicking::begin_panic
   6: syntax::ext::expand::MacroExpander::expand_crate
   7: rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}}
   8: rustc_driver::driver::phase_2_configure_and_expand_inner
   9: rustc_driver::driver::compile_input
  10: rustc_driver::run_compiler

error: internal compiler error: unexpected panic
