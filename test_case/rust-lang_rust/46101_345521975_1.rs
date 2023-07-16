
error: non-ident macro paths are experimental (see issue #35896)
 --> 1.rs:3:10
  |
3 | #[derive(Foo::Anything)]
  |          ^^^^^^^^^^^^^
  |
  = help: add #![feature(use_extern_macros)] to the crate attributes to enable

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.23.0-nightly (aabfed5e0 2017-11-17) running on x86_64-apple-darwin

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'no entry found for key', src/libcore/option.rs:874:4
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: std::panicking::begin_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::option::expect_failed
  10: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::macro_def_scope
  11: rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a>>::resolve_invoc
  12: _ZN6syntax3ext6expand13MacroExpander6expand17h0e8dd401d8088c1eE.llvm.BAEFEFD9
  13: syntax::ext::expand::MacroExpander::expand_crate
  14: _ZN12rustc_driver6driver28phase_2_configure_and_expand28_$u7b$$u7b$closure$u7d$$u7d$17h519a719775ba2532E.llvm.E320C00D
  15: rustc::util::common::time
  16: rustc_driver::driver::compile_input
  17: rustc_driver::run_compiler
