
thread 'rustc' panicked at 'bad input type for cast', src/libcore/option.rs:1038:5d                                                                                         
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::continue_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::option::expect_failed
  10: <rustc_mir::transform::qualify_consts::IsNotConst as rustc_mir::transform::qualify_consts::Qualif>::in_rvalue
  11: rustc_mir::transform::qualify_consts::Checker::assign
  12: rustc::mir::visit::Visitor::super_statement
  13: <rustc_mir::transform::qualify_consts::QualifyAndPromoteConstants as rustc_mir::transform::MirPass>::run_pass
  14: rustc_mir::transform::run_passes::{{closure}}
  15: rustc_mir::transform::run_passes
  16: rustc_mir::transform::mir_validated
  17: rustc::ty::query::__query_compute::mir_validated
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_validated<'tcx>>::compute
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  21: rustc_mir::borrow_check::mir_borrowck
  22: rustc::ty::query::__query_compute::mir_borrowck
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_borrowck<'tcx>>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  26: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::par_body_owners
  27: rustc::util::common::time
  28: <std::thread::local::LocalKey<T>>::with
  29: rustc::ty::context::TyCtxt::create_and_enter
  30: rustc_driver::driver::phase_3_run_analysis_passes
  31: rustc_driver::driver::compile_input
  32: rustc_driver::run_compiler_with_pool
  33: <scoped_tls::ScopedKey<T>>::set
  34: rustc_driver::run_compiler
  35: <scoped_tls::ScopedKey<T>>::set
  36: syntax::with_globals
  37: __rust_maybe_catch_panic
  38: <F as alloc::boxed::FnBox<A>>::call_box
  39: std::sys_common::thread::start_thread
  40: std::sys::unix::thread::Thread::new::thread_start
  41: _pthread_body
  42: _pthread_start
query stack during panic:
#0 [mir_validated] processing `<panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::box_me_up`
#1 [mir_borrowck] processing `<panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::box_me_up`
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z osx-rpath-install-name -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C incremental -C prefer-dynamic -C debug-assertions=n -C link-args=-Wl,-rpath,@loader_path/../lib --crate-type dylib --crate-type rlib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `std`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/Users/acko034/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "/Users/acko034/rust/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
failed to run: /Users/acko034/rust/build/bootstrap/debug/bootstrap build -i --stage 1 src/libstd
Build completed unsuccessfully in 0:04:24
