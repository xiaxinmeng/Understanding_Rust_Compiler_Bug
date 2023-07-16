
$ RUST_BACKTRACE=1 cargo build --verbose
   Compiling paths v0.1.0 (file:///<somewhere>/paths)
     Running `rustc --crate-name paths src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=6e46ca38d9a06af0 -C extra-filename=-6e46ca38d9a06af0 --out-dir /<somewhere>/paths/target/debug/deps -C incremental=/<somewhere>/paths/target/debug/incremental -L dependency=/<somewhere>/paths/target/debug/deps`
error: internal compiler error: librustc_mir/hair/pattern/_match.rs:959: impossible case reached

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
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
   4: core::ops::function::Fn::call
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:403
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: <std::thread::local::LocalKey<T>>::try_with
  11: <std::thread::local::LocalKey<T>>::with
  12: rustc::ty::context::tls::with
  13: rustc::ty::context::tls::with_opt
  14: rustc::session::opt_span_bug_fmt
  15: rustc::session::bug_fmt
  16: rustc_mir::hair::pattern::_match::specialize
  17: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  18: rustc_mir::hair::pattern::_match::is_useful_specialized
  19: <core::iter::Map<I, F> as core::iter::iterator::Iterator>::try_fold
  20: rustc_mir::hair::pattern::_match::is_useful
  21: rustc_mir::hair::pattern::_match::is_useful_specialized
  22: <core::iter::Map<I, F> as core::iter::iterator::Iterator>::try_fold
  23: rustc_mir::hair::pattern::_match::is_useful
  24: <rustc_mir::hair::pattern::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  25: <rustc_mir::hair::pattern::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  26: rustc::session::Session::track_errors
  27: rustc_mir::hair::pattern::check_match::check_match
  28: rustc::dep_graph::graph::DepGraph::with_task_impl
  29: rustc_errors::Handler::track_diagnostics
  30: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  31: rustc::ty::maps::<impl rustc::ty::maps::queries::check_match<'tcx>>::force
  32: rustc::ty::maps::<impl rustc::ty::maps::queries::check_match<'tcx>>::try_get
  33: rustc::ty::maps::TyCtxtAt::check_match
  34: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::check_match
  35: rustc::hir::intravisit::Visitor::visit_nested_body
  36: rustc::hir::Crate::visit_all_item_likes
  37: rustc_mir::hair::pattern::check_match::check_crate
  38: <std::thread::local::LocalKey<T>>::with
  39: <std::thread::local::LocalKey<T>>::with
  40: rustc::ty::context::TyCtxt::create_and_enter
  41: rustc_driver::driver::compile_input
  42: rustc_driver::run_compiler_impl
  43: syntax::with_globals
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.2 (594fb253c 2018-06-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `paths`.

Caused by:
  process didn't exit successfully: `rustc --crate-name paths src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=6e46ca38d9a06af0 -C extra-filename=-6e46ca38d9a06af0 --out-dir /<somewhere>/paths/target/debug/deps -C incremental=/<somewhere>/paths/target/debug/incremental -L dependency=/<somewhere>/paths/target/debug/deps` (exit code: 101)
