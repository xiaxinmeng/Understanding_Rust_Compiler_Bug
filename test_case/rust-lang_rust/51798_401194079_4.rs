
   Compiling rust-issue-51798-example-child v1.0.0 (file:///mnt/c/Users/_/0dmg/child)
   Compiling rust-issue-51798-example-parent v1.0.0 (file:///mnt/c/Users/_/0dmg)
error: internal compiler error: no type-dependent def for method call
  --> src/lib.rs:10:9
   |
10 |         v.clear();
   |         ^^^^^^^^^

thread 'rustc' panicked at 'LocalTableInContext: key not found', libcore/option.rs:960:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
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
             at libstd/panicking.rs:515
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:426
   7: rust_begin_unwind
             at libstd/panicking.rs:337
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:92
   9: core::option::expect_failed
             at libcore/option.rs:960
  10: <rustc_privacy::TypePrivacyVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  11: rustc::hir::intravisit::walk_block
  12: <rustc_privacy::TypePrivacyVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  13: <rustc_privacy::TypePrivacyVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_nested_body
  14: <rustc_privacy::TypePrivacyVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  15: rustc::hir::intravisit::walk_item
  16: <rustc_privacy::TypePrivacyVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  17: rustc_privacy::privacy_access_levels
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::privacy_access_levels<'tcx>>::compute
  19: rustc::ty::context::tls::with_context
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: rustc::ty::context::tls::with_related_context
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  24: rustc::util::common::time
  25: rustc::ty::context::tls::enter_context
  26: <std::thread::local::LocalKey<T>>::with
  27: rustc::ty::context::TyCtxt::create_and_enter
  28: rustc_driver::driver::compile_input
  29: rustc_driver::run_compiler_with_pool
  30: <scoped_tls::ScopedKey<T>>::set
  31: syntax::with_globals
query stack during panic:
#0 [privacy_access_levels] privacy access levels
end of query stack
error: aborting due to previous error


error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (cd494c1f0 2018-06-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `rust-issue-51798-example-parent`.

To learn more, run the command again with --verbose.
