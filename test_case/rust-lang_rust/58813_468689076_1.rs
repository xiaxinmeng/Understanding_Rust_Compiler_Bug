
/home/udi/.cargo/bin/cargo run --color=always --package hello --bin hello
   Compiling hello v0.1.0 (/home/udi/prg/Rust/hello)
thread 'main' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:146:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:211
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:495
   6: std::panicking::begin_panic
   7: rustc::ty::query::job::QueryJob::await
   8: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
   9: rustc_typeck::collect::super_predicates_of
  10: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::super_predicates_of<'tcx>>::compute
  11: rustc::ty::context::tls::with_context
  12: rustc::dep_graph::graph::DepGraph::with_task_impl
  13: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  14: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  15: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query
  16: rustc::ty::query::plumbing::force_from_dep_node
  17: rustc::dep_graph::graph::DepGraph::try_mark_green
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  20: <rustc_typeck::collect::CollectItemTypesVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  21: rustc::hir::Crate::visit_all_item_likes
  22: rustc::util::common::time
  23: rustc_typeck::check_crate
  24: rustc::ty::context::tls::enter_context
  25: <std::thread::local::LocalKey<T>>::with
  26: rustc::ty::context::TyCtxt::create_and_enter
  27: rustc_driver::driver::compile_input
  28: rustc_driver::run_compiler_with_pool
  29: <scoped_tls::ScopedKey<T>>::set
  30: rustc_driver::run_compiler
  31: rustc_driver::monitor::{{closure}}
  32: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  33: rustc_driver::run
  34: rustc_driver::main
  35: std::rt::lang_start::{{closure}}
  36: std::panicking::try::do_call
             at src/libstd/rt.rs:59
             at src/libstd/panicking.rs:310
  37: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  38: std::rt::lang_start_internal
             at src/libstd/panicking.rs:289
             at src/libstd/panic.rs:398
             at src/libstd/rt.rs:58
  39: main
  40: __libc_start_main
  41: <unknown>
query stack during panic:
#0 [super_predicates_of] computing the supertraits of `T1`
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0 (9fda7c223 2019-01-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `hello`.

To learn more, run the command again with --verbose.

Process finished with exit code 101

