
   Compiling sp-arithmetic v2.0.0-rc3 (https://github.com/paritytech/substrate?rev=31c3e06ded197bdf28130ac0c5310283b2d1b5b3#31c3e06d)
   Compiling primitive-types v0.7.2
   Compiling primitive-types v0.6.2
thread 'rustc' panicked at 'if we got here, it must be const', src/librustc_mir/transform/const_prop.rs:1010:34
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1076
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:217
  10: rustc_driver::report_ice
  11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /rustc/5db778affee7c6600c8e7a177c48282dab3f6292/src/liballoc/boxed.rs:1095
  12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
             at /rustc/5db778affee7c6600c8e7a177c48282dab3f6292/src/libproc_macro/bridge/client.rs:318
  13: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:530
  14: rust_begin_unwind
             at src/libstd/panicking.rs:437
  15: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  16: core::option::expect_failed
             at src/libcore/option.rs:1261
  17: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_terminator::{{closure}}
  18: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_terminator
  19: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_body
  20: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  21: rustc_mir::transform::run_passes
  22: rustc_mir::transform::run_optimization_passes
  23: rustc_mir::transform::optimized_mir
  24: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute
  25: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  26: rustc_data_structures::stack::ensure_sufficient_stack
  27: rustc_query_system::query::plumbing::get_query_impl
  28: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir
  29: rustc_mir::monomorphize::collector::collect_neighbours
  30: rustc_mir::monomorphize::collector::collect_items_rec
  31: rustc_session::utils::<impl rustc_session::session::Session>::time
  32: rustc_mir::monomorphize::collector::collect_crate_mono_items
  33: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  34: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_and_partition_mono_items>::compute
  35: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  36: rustc_data_structures::stack::ensure_sufficient_stack
  37: rustc_query_system::query::plumbing::get_query_impl
  38: rustc_codegen_ssa::back::symbol_export::exported_symbols_provider_local
  39: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::exported_symbols>::compute
  40: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  41: rustc_data_structures::stack::ensure_sufficient_stack
  42: rustc_query_system::query::plumbing::get_query_impl
  43: rustc_metadata::rmeta::encoder::encode_metadata_impl
  44: rustc_data_structures::sync::join
  45: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata
  46: rustc_middle::ty::context::TyCtxt::encode_metadata
  47: rustc_interface::passes::start_codegen
  48: rustc_middle::ty::context::tls::enter_global
  49: rustc_interface::queries::Queries::ongoing_codegen
  50: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  51: rustc_span::with_source_map
  52: rustc_interface::interface::run_compiler_in_existing_thread_pool
  53: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-nightly (5db778aff 2020-07-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `<fixed64::Fixed64 as std::ops::Div>::div`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
#2 [exported_symbols] exported_symbols
end of query stack
error: could not compile `sp-arithmetic`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
