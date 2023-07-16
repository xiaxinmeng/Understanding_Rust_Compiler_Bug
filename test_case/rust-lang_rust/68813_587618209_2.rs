
    Checking rust-lightning-bitcoinrpc v0.0.1 (/home/joshua/src/ice-repros/68813)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:378:21
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:188
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:205
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:468
  12: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:373
  13: rust_begin_unwind
             at src/libstd/panicking.rs:302
  14: core::panicking::panic_fmt
             at src/libcore/panicking.rs:139
  15: core::panicking::panic
             at src/libcore/panicking.rs:70
  16: rustc_typeck::check::typeck_tables_of
  17: rustc::ty::query::__query_compute::typeck_tables_of
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  21: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  22: rustc::dep_graph::graph::DepGraph::with_task_impl
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  24: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt>::note_obligation_cause
  25: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt>::report_selection_error
  26: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt>::report_fulfillment_errors
  27: rustc_typeck::check::FnCtxt::resolve_generator_interiors
  28: rustc::ty::context::tls::with_context::{{closure}}
  29: rustc_typeck::check::typeck_tables_of
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query
  33: rustc::ty::query::plumbing::force_from_dep_node
  34: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  35: rustc::dep_graph::graph::DepGraph::try_mark_green
  36: rustc::dep_graph::graph::DepGraph::try_mark_green_and_read
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  38: rustc_typeck::collect::checked_type_of
  39: rustc_typeck::collect::type_of
  40: rustc::ty::query::__query_compute::type_of
  41: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  42: rustc::dep_graph::graph::DepGraph::with_task_impl
  43: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  44: rustc::hir::intravisit::walk_expr
  45: rustc::hir::intravisit::Visitor::visit_fn
  46: rustc::hir::intravisit::walk_item
  47: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_item
  48: rustc::hir::map::Map::visit_item_likes_in_module
  49: rustc_typeck::collect::collect_mod_item_types
  50: rustc::ty::query::__query_compute::collect_mod_item_types
  51: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  52: rustc::dep_graph::graph::DepGraph::with_task_impl
  53: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  55: rustc_typeck::check_crate::{{closure}}::{{closure}}
  56: rustc::util::common::time
  57: rustc_typeck::check_crate
  58: rustc_interface::passes::analysis
  59: rustc::ty::query::__query_compute::analysis
  60: rustc::dep_graph::graph::DepGraph::with_task_impl
  61: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  62: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  63: rustc_interface::passes::create_global_ctxt::{{closure}}
  64: rustc_interface::passes::BoxedGlobalCtxt::enter
  65: rustc_interface::interface::run_compiler_in_existing_thread_pool
  66: std::thread::local::LocalKey<T>::with
  67: scoped_tls::ScopedKey<T>::set
  68: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0 (73528e339 2019-12-16) running on x86_64-unknown-linux-musl

note: compiler flags: -C debuginfo=2 -C incremental -C target-feature=-crt-static --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck_tables_of] processing `lightning_net_tokio::setup_outbound`
#1 [typeck_tables_of] processing `lightning_net_tokio::setup_outbound::{{closure}}#0`
#2 [typeck_tables_of] processing `f`
#3 [typeck_tables_of] processing `f::{{closure}}#0`
#4 [type_of] processing `f::{{closure}}#0`
#5 [collect_mod_item_types] collecting item types in top-level module
#6 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `rust-lightning-bitcoinrpc`
