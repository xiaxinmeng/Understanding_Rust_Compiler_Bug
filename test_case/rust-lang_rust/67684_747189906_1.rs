
warning: function is never used: `expr`
  --> perses_node_priority_with_dfs_delta_reduced_mutant.rs:25:4
   |
25 | fn expr<A>() -> impl Parser
   |    ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: function is never used: `parse_mode_impl`
  --> perses_node_priority_with_dfs_delta_reduced_mutant.rs:31:4
   |
31 | fn parse_mode_impl<A>()
   |    ^^^^^^^^^^^^^^^

error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:43:32: could not fully normalize `<impl Parser as Parser>::PartialState`

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::bug
   2: rustc_errors::Handler::bug
   3: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
   4: rustc_middle::ty::context::tls::with_opt::{{closure}}
   5: rustc_middle::ty::context::tls::with_opt
   6: rustc_middle::util::bug::opt_span_bug_fmt
   7: rustc_middle::util::bug::bug_fmt
   8: rustc_infer::infer::InferCtxtBuilder::enter
   9: rustc_traits::normalize_erasing_regions::normalize_generic_arg_after_erasing_regions
  10: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  11: rustc_data_structures::stack::ensure_sufficient_stack
  12: rustc_query_system::query::plumbing::get_query_impl
  13: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
  14: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  15: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  16: rustc_mir::transform::run_passes
  17: rustc_mir::transform::run_optimization_passes
  18: rustc_mir::transform::optimized_mir
  19: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  21: rustc_data_structures::stack::ensure_sufficient_stack
  22: rustc_query_system::query::plumbing::get_query_impl
  23: rustc_metadata::rmeta::encoder::EncodeContext::encode_optimized_mir
  24: <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_item
  25: rustc_hir::hir::Crate::visit_all_item_likes
  26: rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root
  27: rustc_metadata::rmeta::encoder::encode_metadata_impl
  28: rustc_data_structures::sync::join
  29: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata
  30: rustc_middle::ty::context::TyCtxt::encode_metadata
  31: rustc_interface::passes::QueryContext::enter
  32: rustc_interface::queries::Queries::ongoing_codegen
  33: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  34: rustc_span::with_source_map
  35: rustc_interface::interface::create_compiler_and_run
  36: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (b32e6e6ac 2020-12-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type lib

query stack during panic:
#0 [normalize_generic_arg_after_erasing_regions] normalizing `<impl Parser as Parser>::PartialState`
#1 [optimized_mir] optimizing MIR for `parse_mode_impl`
end of query stack
error: aborting due to previous error; 2 warnings emitted

