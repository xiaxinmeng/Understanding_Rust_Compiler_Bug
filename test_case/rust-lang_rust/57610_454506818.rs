
warning: unreachable arm
 --> src/test/ui/pattern/const-pat-ice.rs:9:18
  |
9 |         &&&42 => {},
  |                  ^^
  |
  = note: #[warn(unreachable_code)] on by default

warning: unreachable pattern
  --> src/test/ui/pattern/const-pat-ice.rs:10:9
   |
10 |         FOO => {},
   |         ^^^
   |
   = note: #[warn(unreachable_patterns)] on by default

thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:1068:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_mir::hair::pattern::_match::is_useful
   8: rustc_mir::hair::pattern::_match::is_useful_specialized
   9: <core::iter::Map<I, F> as core::iter::iterator::Iterator>::try_fold
  10: rustc_mir::hair::pattern::_match::is_useful
  11: rustc_mir::hair::pattern::_match::is_useful_specialized
  12: <core::iter::Map<I, F> as core::iter::iterator::Iterator>::try_fold
  13: rustc_mir::hair::pattern::_match::is_useful
  14: rustc_mir::hair::pattern::check_match::check_arms
  15: rustc_mir::hair::pattern::_match::MatchCheckCtxt::create_and_enter
  16: <rustc_mir::hair::pattern::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  17: <rustc_mir::hair::pattern::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  18: <rustc_mir::hair::pattern::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_body
  19: rustc::session::Session::track_errors
  20: rustc_mir::hair::pattern::check_match::check_match
  21: rustc::ty::query::__query_compute::check_match
  22: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::check_match<'tcx>>::compute
  23: rustc::dep_graph::graph::DepGraph::with_task_impl
  24: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  25: <rustc_mir::hair::pattern::check_match::OuterVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_body
  26: rustc::hir::intravisit::walk_item
  27: rustc::hir::Crate::visit_all_item_likes
  28: rustc_mir::hair::pattern::check_match::check_crate
  29: rustc::util::common::time
  30: <std::thread::local::LocalKey<T>>::with
  31: rustc::ty::context::TyCtxt::create_and_enter
  32: rustc_driver::driver::compile_input
  33: <scoped_tls::ScopedKey<T>>::set
  34: rustc_driver::run_compiler
  35: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [check_match] processing `main`
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
