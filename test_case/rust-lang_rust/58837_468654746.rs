
    Checking rustc_interface v0.0.0 (/home/centril/programming/rust/src/librustc_interface)
thread 'rustc' panicked at 'src/librustc/hir/def.rs:257: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
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
   7: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   8: rustc::ty::context::tls::with_opt::{{closure}}
   9: rustc::ty::context::tls::with_context_opt
  10: rustc::ty::context::tls::with_opt
  11: rustc::util::bug::opt_span_bug_fmt
  12: rustc::util::bug::bug_fmt
  13: rustc::hir::def::Def::def_id::{{closure}}
  14: rustc::hir::def::Def::def_id
  15: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::populate_module_if_necessary
  16: rustc_resolve::resolve_imports::<impl rustc_resolve::Resolver<'a>>::resolve_ident_in_module_unadjusted_ext
  17: rustc_resolve::Resolver::resolve_ident_in_module_ext
  18: rustc_resolve::resolve_imports::ImportResolver::resolve_import::{{closure}}
  19: rustc_resolve::resolve_imports::ImportResolver::resolve_imports
  20: rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a>>::resolve_imports
  21: syntax::ext::expand::MacroExpander::expand_fragment
  22: syntax::ext::expand::MacroExpander::expand_crate
  23: rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}}
  24: rustc::util::common::time
  25: rustc_driver::driver::phase_2_configure_and_expand
  26: rustc_driver::driver::compile_input
  27: rustc_driver::run_compiler_with_pool
  28: <scoped_tls::ScopedKey<T>>::set
  29: rustc_driver::run_compiler
  30: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-beta.7 (08f107300 2019-02-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib

note: some of the compiler flags provided by cargo are hidden
