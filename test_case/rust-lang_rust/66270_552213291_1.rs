
error: expected type, found `0`
 --> src/main.rs:2:10
  |
2 |     foo: 0,
  |          ^ expected type

thread 'rustc' panicked at 'no index for a field', src/libcore/option.rs:1187:5
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
             at src/libcore/fmt/mod.rs:1030
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
             at src/libcore/panicking.rs:141
  15: core::option::expect_failed
             at src/libcore/option.rs:1187
  16: rustc::ty::<impl rustc::ty::context::TyCtxt>::field_index
  17: rustc::middle::mem_categorization::MemCategorizationContext::cat_pattern_
  18: rustc_typeck::check::regionck::RegionCtxt::link_pattern
  19: <rustc_typeck::check::regionck::RegionCtxt as rustc::hir::intravisit::Visitor>::visit_local
  20: rustc::hir::intravisit::walk_expr
  21: <rustc_typeck::check::regionck::RegionCtxt as rustc::hir::intravisit::Visitor>::visit_expr
  22: rustc_typeck::check::regionck::RegionCtxt::visit_fn_body
  23: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt>::regionck_fn
  24: rustc::ty::context::tls::with_context::{{closure}}
  25: rustc_typeck::check::typeck_tables_of
  26: rustc::ty::query::__query_compute::typeck_tables_of
  27: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  28: rustc::dep_graph::graph::DepGraph::with_task_impl
  29: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  30: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  31: rustc_typeck::check::typeck_item_bodies
  32: rustc::ty::query::__query_compute::typeck_item_bodies
  33: rustc::dep_graph::graph::DepGraph::with_task_impl
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  35: rustc_typeck::check_crate
  36: rustc_interface::passes::analysis
  37: rustc::ty::query::__query_compute::analysis
  38: rustc::dep_graph::graph::DepGraph::with_task_impl
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  40: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  41: rustc_interface::passes::create_global_ctxt::{{closure}}
  42: rustc_interface::passes::BoxedGlobalCtxt::enter
  43: rustc_interface::interface::run_compiler_in_existing_thread_pool
  44: std::thread::local::LocalKey<T>::with
  45: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (ac162c6ab 2019-11-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

error: could not compile `example`.

To learn more, run the command again with --verbose.
