sh
$ cat > assertion_ice.rs
static UTF8_CHAR_WIDTH: [u8; 0] = [];

pub fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
$ rustc +nightly assertion_ice.rs --emit mir --crate-type lib
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', src/librustc/mir/interpret/value.rs:319:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::continue_panic_fmt
   8: std::panicking::begin_panic_fmt
   9: rustc::mir::interpret::value::Scalar<Tag>::to_bits
  10: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_value
  11: rustc_mir::interpret::validity::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::validate_operand
  12: <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_statement
  13: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  14: rustc_mir::transform::run_passes
  15: rustc_mir::transform::run_optimization_passes
  16: rustc_mir::transform::optimized_mir
  17: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  18: rustc::dep_graph::graph::DepGraph::with_task_impl
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  20: rustc_mir::monomorphize::collector::collect_items_rec
  21: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  22: rustc::util::common::time
  23: rustc_mir::monomorphize::collector::collect_crate_mono_items
  24: rustc::util::common::time
  25: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  26: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  29: rustc_codegen_ssa::back::symbol_export::exported_symbols_provider_local
  30: rustc::ty::query::__query_compute::exported_symbols
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  33: rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root
  34: rustc::ty::context::tls::with_context::{{closure}}
  35: rustc_metadata::rmeta::encoder::encode_metadata
  36: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata
  37: rustc::ty::context::TyCtxt::encode_metadata
  38: rustc_interface::passes::start_codegen::{{closure}}
  39: rustc_interface::passes::start_codegen
  40: rustc::ty::context::tls::enter_global
  41: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  42: rustc_interface::passes::create_global_ctxt::{{closure}}
  43: rustc_interface::passes::BoxedGlobalCtxt::enter
  44: rustc_interface::queries::Query<T>::compute
  45: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  46: rustc_interface::interface::run_compiler_in_existing_thread_pool
  47: std::thread::local::LocalKey<T>::with
  48: scoped_tls::ScopedKey<T>::set
  49: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (25d8a9494 2019-11-29) running on x86_64-apple-darwin

note: compiler flags: --crate-type lib

query stack during panic:
#0 [optimized_mir] processing `utf8_char_width`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
#2 [exported_symbols] exported_symbols
end of query stack
