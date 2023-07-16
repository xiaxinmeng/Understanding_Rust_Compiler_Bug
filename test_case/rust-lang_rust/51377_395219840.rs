
[santiago@archlinux inflate (master)]$ perf focus '{do_mir_borrowck}' --tree-callees --tree-min-percent 3
Matcher    : {do_mir_borrowck}
Matches    : 2290
Not Matches: 331
Percentage : 87%

Tree
| matched `{do_mir_borrowck}` (87% total, 0% self)
: | rustc_mir::borrow_check::nll::compute_regions (72% total, 0% self)
: : | rustc_mir::borrow_check::nll::type_check::type_check_internal (64% total, 0% self)
: : : | rustc_mir::borrow_check::nll::type_check::type_check::_$u7b$$u7b$closure$u7d$$u7d$::h5e644cf9693979bb (63% total, 0% self)
: : : : | rustc_mir::borrow_check::nll::type_check::liveness::generate (63% total, 59% self)
: : | rustc_mir::borrow_check::nll::region_infer::RegionInferenceContext::solve (6% total, 0% self)
: : : | rustc::util::common::time (6% total, 0% self)
: : : : | rustc_mir::borrow_check::nll::region_infer::RegionInferenceContext::solve_inner (6% total, 0% self)
: : : : : | rustc_mir::borrow_check::nll::region_infer::RegionInferenceContext::compute_region_values (6% total, 0% self)
: : : : : : | <rustc_data_structures::bitvec::SparseBitMatrix<R, C>>::merge (6% total, 0% self)
: : : : : : : | <alloc::btree::map::BTreeMap<K, V>>::entry (5% total, 1% self)
: : : : : : : : | alloc::btree::search::search_tree (3% total, 0% self)
: : : : : : : : : | alloc::btree::search::search_tree (3% total, 3% self)
: | rustc::mir::visit::Visitor::visit_mir (5% total, 3% self)
: | <rustc_mir::borrow_check::MirBorrowckCtxt<'cx, 'gcx, 'tcx> as rustc_mir::dataflow::DataflowResultsConsumer<'cx, 'tcx>>::visit_statement_entry (4% total, 0% self)
: | rustc_mir::dataflow::do_dataflow (3% total, 0% self)
