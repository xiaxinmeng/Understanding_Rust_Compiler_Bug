
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.23.0-dev running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Expected Def::Macro(..)', rust://src/librustc_resolve/build_reduced_graph.rs:571:17
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at rust://src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at rust://src/libstd/sys_common/backtrace.rs:68
   2: std::panicking::default_hook::{{closure}}
             at rust://src/libstd/sys_common/backtrace.rs:57
             at rust://src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at rust://src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at rust://src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at rust://src/libstd/panicking.rs:538
   6: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::get_macro
             at rust://src/librustc_resolve/build_reduced_graph.rs:571
   7: rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a>>::resolve_macro
             at rust://src/librustc_resolve/macros.rs:310
             at rust://src/libcore/result.rs:468
             at rust://src/librustc_resolve/macros.rs:308
   8: syntax::ext::expand::MacroExpander::expand
             at rust://src/libsyntax/ext/expand.rs:340
   9: syntax::ext::expand::MacroExpander::expand_crate
             at rust://src/libsyntax/ext/expand.rs:240
  10: rustc_driver::driver::phase_2_configure_and_expand::{{closure}}
             at rust://src/librustc_driver/driver.rs:783
  11: rustc_driver::driver::phase_2_configure_and_expand
             at rust://src/librustc/util/common.rs:120
             at rust://src/librustc_driver/driver.rs:742
  12: rustc_driver::driver::compile_input
             at rust://src/librustc_driver/driver.rs:145
  13: rustc_driver::run_compiler
             at rust://src/librustc_driver/lib.rs:253

