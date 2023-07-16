
lunch-box. perf focus  '{do_mir_borrowck}' --tree-callees --tree-min-percent 1
Matcher    : {do_mir_borrowck}
Matches    : 152
Not Matches: 776
Percentage : 16%

Tree
| matched `{do_mir_borrowck}` (16% total, 0% self)
: | rustc_mir::borrow_check::nll::compute_regions (12% total, 0% self)
: : | rustc_mir::borrow_check::nll::type_check::type_check (10% total, 0% self)
: : : | rustc_mir::borrow_check::nll::type_check::type_check_internal (10% total, 0% self)
: : : : | rustc_mir::borrow_check::nll::type_check::type_check::_$u7b$$u7b$closure$u7d$$u7d$::haa6458083ae8ea7a (7% total, 0% self)
: : : : : | rustc_mir::borrow_check::nll::type_check::liveness::generate (7% total, 0% self)
: : : : : : | rustc_mir::borrow_check::nll::type_check::TypeChecker::fully_perform_op (5% total, 0% self)
: : : : : : : | rustc::infer::InferCtxt::take_and_reset_region_constraints (2% total, 0% self)
: : : : : : : : | rustc::infer::region_constraints::RegionConstraintCollector::take_and_reset_data (2% total, 0% self)
: : : : : : : : : | <rustc_data_structures::unify::UnificationTable<K>>::new_key (2% total, 0% self)
: : : : : : : : : : | <rustc_data_structures::snapshot_vec::SnapshotVec<D>>::push (1% total, 1% self)
: : : : : : : | rustc::infer::InferCtxt::commit_if_ok (2% total, 0% self)
: : : : : : : : | rustc::traits::query::dropck_outlives::<impl rustc::infer::at::At<'cx, 'gcx, 'tcx>>::dropck_outlives (2% total, 0% self)
: : : : : : : : : | rustc::infer::canonical::<impl rustc::infer::InferCtxt<'cx, 'gcx, 'tcx>>::instantiate_query_result (1% total, 0% self)
: : : : : : | <rustc_mir::dataflow::at_location::FlowAtLocation<BD>>::each_state_bit (1% total, 1% self)
: : : : | <rustc_mir::borrow_check::nll::type_check::TypeVerifier<'a, 'b, 'gcx, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_mir (1% total, 0% self)
: : | rustc_mir::borrow_check::nll::region_infer::RegionInferenceContext::solve (1% total, 0% self)
