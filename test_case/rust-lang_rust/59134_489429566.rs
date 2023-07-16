
thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:620:9
stack backtrace:
query stack during panic:
end of query stack

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.1 (fc50f328b 2019-04-24) running on x86_64-pc-windows-msvc

{"message":"src\\librustc\\ty\\context.rs:541: node_type: no type for node `expr <Self>::SC_NEGATE_REQS (id=50362)`","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src\\librustc\\ty\\context.rs:541: node_type: no type for node `expr <Self>::SC_NEGATE_REQS (id=50362)`\n\n"}
thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:620:9
stack backtrace:
   0: std::sys_common::alloc::realloc_fallback
   1: std::panicking::take_hook
   2: std::panicking::take_hook
   3: <rustc::ty::sty::Binder<rustc::ty::ProjectionPredicate<'tcx>> as rustc::ty::ToPredicate<'tcx>>::to_predicate
   4: std::panicking::rust_panic_with_hook
   5: <rustc_errors::emitter::ColorConfig as core::fmt::Debug>::fmt
   6: rustc_errors::Handler::bug
   7: rustc::util::bug::bug_fmt
   8: rustc::ty::wf::object_region_bounds
   9: rustc::ty::wf::object_region_bounds
  10: rustc::ty::wf::object_region_bounds
  11: rustc::util::bug::bug_fmt
  12: rustc::util::bug::bug_fmt
  13: rustc::ty::context::TypeckTables::node_type
  14: rustc::ty::context::TypeckTables::expr_ty_adjusted
  15: rustc_save_analysis::SaveContext::get_expr_data
  16: <unknown>
  17: <unknown>
  18: <unknown>
  19: <unknown>
  20: <unknown>
  21: <unknown>
  22: <unknown>
  23: <unknown>
  24: <rustc_save_analysis::DumpHandler<'a> as rustc_save_analysis::SaveHandler>::save
  25: <env_logger::fmt::WriteStyle as core::default::Default>::default
  26: rustc_driver::enable_save_analysis
  27: rustc_driver::enable_save_analysis
  28: <env_logger::fmt::WriteStyle as core::default::Default>::default
  29: <rustc_driver::CompilationFailure as core::fmt::Debug>::fmt
  30: <rustc_driver::CompilationFailure as core::fmt::Debug>::fmt
  31: <env_logger::fmt::WriteStyle as core::default::Default>::default
  32: rustc_driver::driver::compile_input
  33: rustc_driver::run_compiler
  34: <rustc_driver::CompilationFailure as core::fmt::Debug>::fmt
  35: rustc_driver::run_compiler
  36: git_libgit2_opts
  37: git_libgit2_opts
  38: _rust_maybe_catch_panic
  39: git_libgit2_opts
  40: std::sys::windows::thread::Thread::new
  41: BaseThreadInitThunk
  42: RtlUserThreadStart
query stack during panic:
end of query stack
{"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.1 (fc50f328b 2019-04-24) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden
