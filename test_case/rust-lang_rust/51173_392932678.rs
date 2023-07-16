
thread 'main' panicked at 'no entry found for key', libcore/option.rs:960:5
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:467
   6: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:350
   7: rust_begin_unwind
             at libstd/panicking.rs:328
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:71
   9: core::option::expect_failed
             at libcore/option.rs:960
  10: rustc_resolve::macros::<impl rustc_resolve::Resolver<'a>>::resolve_macro_to_def_inner
  11: rustc_resolve::macros::<impl rustc_resolve::Resolver<'a>>::resolve_macro_to_def
  12: rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a>>::resolve_invoc
  13: syntax::ext::expand::MacroExpander::expand
  14: syntax::ext::expand::MacroExpander::expand_crate
  15: rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}}
  16: rustc::util::common::time
  17: rustc_driver::driver::phase_2_configure_and_expand
  18: rustc_driver::driver::compile_input
  19: rustc_driver::run_compiler_with_pool
  20: syntax::with_globals
  21: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  22: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  23: rustc_driver::run
  24: rustc_driver::main
  25: std::rt::lang_start::{{closure}}
  26: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  27: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  28: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:374
             at libstd/rt.rs:58
  29: main
  30: __libc_start_main
  31: <unknown>
query stack during panic:
end of query stack
