
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:335:21
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:207
   3: std::panicking::default_hook
             at libstd/panicking.rs:223
   4: rustc::util::common::panic_hook
             at librustc/util/common.rs:51
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:403
   6: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:349
   7: rust_begin_unwind
             at libstd/panicking.rs:325
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:72
   9: core::panicking::panic
             at libcore/panicking.rs:51
  10: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::get_module
             at /home/mw/3-rust/src/libcore/macros.rs:20
             at librustc_resolve/build_reduced_graph.rs:555
  11: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::macro_def_scope
             at librustc_resolve/build_reduced_graph.rs:574
  12: rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a>>::resolve_invoc
             at librustc_resolve/macros.rs:305
  13: syntax::ext::expand::MacroExpander::expand
             at libsyntax/ext/expand.rs:290
  14: syntax::ext::expand::MacroExpander::expand_crate
             at libsyntax/ext/expand.rs:245
  15: rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}}
             at librustc_driver/driver.rs:794
  16: rustc::util::common::time
             at /home/mw/3-rust/src/librustc/util/common.rs:139
             at /home/mw/3-rust/src/librustc/util/common.rs:133
  17: rustc_driver::driver::phase_2_configure_and_expand
             at librustc_driver/driver.rs:753
             at librustc_driver/driver.rs:605
  18: rustc_driver::driver::compile_input
             at librustc_driver/driver.rs:128
  19: rustc_driver::run_compiler
             at librustc_driver/lib.rs:520
