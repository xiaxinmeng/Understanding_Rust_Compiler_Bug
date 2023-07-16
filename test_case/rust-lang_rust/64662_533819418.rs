

error: internal compiler error: src/librustc/ty/mod.rs:2400: enum discriminant depends on generic arguments
 --> src/lib.rs:3:7
  |
3 |   B = foo(),
  |       ^^^^^

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:578:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::begin_panic
  13: rustc_errors::Handler::span_bug
  14: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  15: rustc::ty::context::tls::with_opt::{{closure}}
  16: rustc::ty::context::tls::with_context_opt
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::span_bug_fmt
  20: rustc::ty::AdtDef::eval_explicit_discr
  21: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_item
  22: rustc::hir::map::Map::visit_item_likes_in_module
  23: rustc_typeck::collect::collect_mod_item_types
  24: rustc::ty::query::__query_compute::collect_mod_item_types
  25: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
  27: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  28: rustc_typeck::check_crate::{{closure}}::{{closure}}
  29: rustc::util::common::time
  30: rustc_typeck::check_crate
  31: rustc_interface::passes::analysis
  32: rustc::ty::query::__query_compute::analysis
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  34: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  35: rustc_interface::passes::create_global_ctxt::{{closure}}
  36: rustc_interface::interface::run_compiler_in_existing_thread_pool
  37: std::thread::local::LocalKey<T>::with
  38: scoped_tls::ScopedKey<T>::set
  39: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (97e58c0d3 2019-09-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_mod_item_types] collecting item types in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0019, E0282.
For more information about an error, try `rustc --explain E0019`.
error: could not compile `playground`.
