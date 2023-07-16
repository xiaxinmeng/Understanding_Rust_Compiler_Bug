
/home/thomas/.cargo/bin/cargo test --color=always --no-run --package ron --test large_number test_large_number -- --exact
   Compiling ron v0.5.1 (/home/thomas/Workspace/ron)
error: internal compiler error: src/librustc/ty/query/plumbing.rs:1195: Cannot force dep node: coherent_trait(core[30a9]::ops[0]::drop[0]::Drop[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:635:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc::ty::query::plumbing::force_from_dep_node
  15: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  16: rustc::dep_graph::graph::DepGraph::try_mark_green
  17: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  18: rustc::ty::context::GlobalCtxt::enter_local
  19: rustc::ty::util::<impl rustc::ty::ParamEnv>::can_type_implement_copy
  20: rustc_typeck::coherence::builtin::check_trait
  21: rustc_typeck::coherence::coherent_trait
  22: rustc::ty::query::__query_compute::coherent_trait
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::coherent_trait>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  26: rustc_typeck::check_crate::{{closure}}::{{closure}}
  27: rustc::util::common::time
  28: rustc_typeck::check_crate
  29: rustc_interface::passes::analysis
  30: rustc::ty::query::__query_compute::analysis
  31: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  34: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  35: rustc_interface::passes::create_global_ctxt::{{closure}}
  36: rustc_interface::interface::run_compiler_in_existing_thread_pool
  37: std::thread::local::LocalKey<T>::with
  38: scoped_tls::ScopedKey<T>::set
  39: syntax::with_globals
query stack during panic:
#0 [adt_destructor] processing `value::Number`
#1 [coherent_trait] coherence checking all impls of trait `std::marker::Copy`
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.35.0-nightly (53f2165c5 2019-04-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `ron`.

To learn more, run the command again with --verbose.

Process finished with exit code 101

