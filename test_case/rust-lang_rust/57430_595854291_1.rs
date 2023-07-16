
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:745: cannot create local mono-item for DefId(27:436 ~ noria[a50e]::view[0]::results[0]::{{impl}}[0]::new[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:875:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1053
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1428
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /rustc/96bb8b31c81dc2394317f2f083c3acf8087efea1/src/liballoc/boxed.rs:1031
  12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
             at /rustc/96bb8b31c81dc2394317f2f083c3acf8087efea1/src/libproc_macro/bridge/client.rs:305
  13: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:474
  14: std::panicking::begin_panic
  15: rustc_errors::HandlerInner::bug
  16: rustc_errors::Handler::bug
  17: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  18: rustc::ty::context::tls::with_opt::{{closure}}
  19: rustc::ty::context::tls::with_opt
  20: rustc::util::bug::opt_span_bug_fmt
  21: rustc::util::bug::bug_fmt
  22: rustc_mir::monomorphize::collector::should_monomorphize_locally
  23: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_terminator_kind
  24: rustc_mir::monomorphize::collector::collect_items_rec
  25: rustc_mir::monomorphize::collector::collect_items_rec
  26: rustc_mir::monomorphize::collector::collect_items_rec
  27: rustc_mir::monomorphize::collector::collect_items_rec
  28: rustc_mir::monomorphize::collector::collect_items_rec
  29: rustc_mir::monomorphize::collector::collect_items_rec
  30: rustc_mir::monomorphize::collector::collect_items_rec
  31: rustc_mir::monomorphize::collector::collect_items_rec
  32: rustc_mir::monomorphize::collector::collect_items_rec
  33: rustc_mir::monomorphize::collector::collect_items_rec
  34: rustc_mir::monomorphize::collector::collect_items_rec
  35: rustc_mir::monomorphize::collector::collect_items_rec
  36: rustc_mir::monomorphize::collector::collect_items_rec
  37: rustc_mir::monomorphize::collector::collect_items_rec
  38: rustc_mir::monomorphize::collector::collect_items_rec
  39: rustc_mir::monomorphize::collector::collect_items_rec
  40: rustc_mir::monomorphize::collector::collect_items_rec
  41: rustc_mir::monomorphize::collector::collect_items_rec
  42: rustc_mir::monomorphize::collector::collect_items_rec
  43: rustc_mir::monomorphize::collector::collect_items_rec
  44: rustc_mir::monomorphize::collector::collect_items_rec
  45: rustc_mir::monomorphize::collector::collect_items_rec
  46: rustc_mir::monomorphize::collector::collect_items_rec
  47: rustc_mir::monomorphize::collector::collect_items_rec
  48: rustc_mir::monomorphize::collector::collect_items_rec
  49: rustc_mir::monomorphize::collector::collect_items_rec
  50: rustc_mir::monomorphize::collector::collect_items_rec
  51: rustc_mir::monomorphize::collector::collect_items_rec
  52: rustc_session::utils::<impl rustc_session::session::Session>::time
  53: rustc_mir::monomorphize::collector::collect_crate_mono_items
  54: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  55: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  56: rustc::dep_graph::graph::DepGraph::with_task_impl
  57: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  58: rustc_codegen_ssa::back::symbol_export::exported_symbols_provider_local
  59: rustc::ty::query::__query_compute::exported_symbols
  60: rustc::dep_graph::graph::DepGraph::with_task_impl
  61: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  62: rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root
  63: rustc::dep_graph::graph::DepGraph::with_ignore
  64: rustc_metadata::rmeta::encoder::encode_metadata
  65: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata
  66: rustc::ty::context::TyCtxt::encode_metadata
  67: rustc_interface::passes::start_codegen
  68: rustc::ty::context::tls::enter_global
  69: rustc_interface::queries::Queries::ongoing_codegen
  70: rustc_interface::interface::run_compiler_in_existing_thread_pool
  71: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-nightly (96bb8b31c 2020-03-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental -C target-cpu=native --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [exported_symbols] exported_symbols
end of query stack
error: aborting due to previous error

error: could not compile `noria-server`.
