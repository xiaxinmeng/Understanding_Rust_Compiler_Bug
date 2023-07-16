
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_mir::borrow_check::universal_regions::UniversalRegionIndices::to_region_vid
   8: rustc_mir::borrow_check::universal_regions::UniversalRegionIndices::fold_to_region_vids::{{closure}}
   9: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
  10: rustc_mir::borrow_check::universal_regions::UniversalRegions::new
  11: rustc_mir::borrow_check::nll::replace_regions_in_mir
  12: rustc_mir::borrow_check::do_mir_borrowck
  13: rustc_infer::infer::InferCtxtBuilder::enter
  14: rustc_mir::borrow_check::mir_borrowck
  15: core::ops::function::FnOnce::call_once
  16: rustc_query_system::query::plumbing::get_query_impl
  17: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
  18: rustc_mir::transform::mir_drops_elaborated_and_const_checked
  19: rustc_query_system::query::plumbing::get_query_impl
  20: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  21: rustc_mir::transform::inner_mir_for_ctfe
  22: rustc_mir::transform::mir_for_ctfe
  23: rustc_query_system::query::plumbing::get_query_impl
  24: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_for_ctfe
  25: <rustc_mir::const_eval::machine::CompileTimeInterpreter as rustc_mir::interpret::machine::Machine>::load_mir
  26: rustc_mir::interpret::eval_context::InterpCx<M>::load_mir
  27: rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider
  28: rustc_query_system::query::plumbing::get_query_impl
  29: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
  30: rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider
  31: rustc_query_system::query::plumbing::get_query_impl
  32: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
