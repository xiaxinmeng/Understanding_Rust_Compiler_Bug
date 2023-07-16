
   Compiling quick-error-ice v0.1.0 (/home/milkey/github/quick-error-ice)
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: DistinctSources(DistinctSources { begin: (Real("src/main.rs"), BytePos(0)), end: (Macros("::quick_error::quick_error"), BytePos(7612215)) })', src/librustc_errors/lib.rs:197:29
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: <unknown>
   3: <unknown>
   4: <unknown>
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::result::unwrap_failed
  10: <unknown>
  11: <unknown>
  12: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic
  13: <rustc_errors::json::JsonEmitter as rustc_errors::emitter::Emitter>::emit_diagnostic
  14: rustc_errors::HandlerInner::emit_diagnostic
  15: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
  16: <unknown>
  17: <rustc_resolve::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_lifetime
  18: <rustc_resolve::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_ty
  19: <unknown>
  20: <unknown>
  21: <rustc_resolve::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_item
  22: <unknown>
  23: <unknown>
  24: <unknown>
  25: <unknown>
  26: <unknown>
  27: <unknown>
  28: <unknown>
  29: rustc::ty::context::TyCtxt::object_lifetime_defaults
  30: <unknown>
  31: <unknown>
  32: <unknown>
  33: <unknown>
  34: <unknown>
  35: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
  36: <unknown>
  37: <unknown>
  38: <unknown>
  39: <unknown>
  40: <unknown>
  41: <unknown>
  42: <unknown>
  43: rustc_typeck::check_crate
  44: <unknown>
  45: <unknown>
  46: <unknown>
  47: <unknown>
  48: <unknown>
  49: <unknown>
  50: <unknown>
  51: <unknown>
  52: <unknown>
  53: __rust_maybe_catch_panic
  54: <unknown>
  55: <unknown>
  56: <unknown>
  57: <unknown>
  58: start_thread
  59: __clone
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-dev (2cf24ab89 2020-01-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [resolve_lifetimes] resolving lifetimes
#1 [object_lifetime_defaults_map] looking up lifetime defaults for a region
#2 [generics_of] processing `main`
#3 [collect_mod_item_types] collecting item types in top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `quick-error-ice`.

To learn more, run the command again with --verbose.
