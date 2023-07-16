#0  0x00007fd06c5bbdbe in rustc_middle::ty::fold::<impl rustc_middle::ty::context::TyCtxt>::replace_bound_vars ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#1  0x00007fd06c55f458 in rustc_infer::infer::InferCtxt::commit_if_ok () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#2  0x00007fd06c575dc7 in rustc_trait_selection::traits::project::poly_project_and_unify_type ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#3  0x00007fd06c5901f1 in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#4  0x00007fd06c5b7b40 in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#5  0x00007fd06c5ab22f in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#6  0x00007fd06c590298 in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#7  0x00007fd06c5b7b40 in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#8  0x00007fd06c5ab22f in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#9  0x00007fd06c590298 in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#10 0x00007fd06c5b7b40 in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#11 0x00007fd06c5ab22f in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#12 0x00007fd06c590298 in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#13 0x00007fd06c5b7b40 in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#14 0x00007fd06c5ab22f in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#15 0x00007fd06c590298 in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#16 0x00007fd06c5b7b40 in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#17 0x00007fd06c5ab22f in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#18 0x00007fd06c590298 in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#19 0x00007fd06c5b7b40 in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#20 0x00007fd06c5ab22f in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
<snip>
#58 0x00007fd06c5b7b40 in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#59 0x00007fd06c5ab52a in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#60 0x00007fd06c564192 in rustc_infer::infer::InferCtxt::probe () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#61 0x00007fd06c5b7d23 in rustc_trait_selection::traits::select::SelectionContext::evaluate_candidate ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#62 0x00007fd06c5acaee in rustc_trait_selection::traits::select::SelectionContext::evaluate_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#63 0x00007fd06c57d788 in rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#64 0x00007fd06c5bd538 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_anon_task ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#65 0x00007fd06c5abdc5 in rustc_trait_selection::traits::select::SelectionContext::evaluate_trait_predicate_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#66 0x00007fd06c590138 in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#67 0x00007fd06c5b7b40 in rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#68 0x00007fd06c5634d7 in rustc_infer::infer::InferCtxt::probe () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#69 0x00007fd06c29539d in rustc_infer::infer::InferCtxtBuilder::enter_with_canonical ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#70 0x00007fd06c29b3ab in rustc_traits::evaluate_obligation::evaluate_obligation::h9c3d4e486cbe91f8 ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#71 0x00007fd06cd83378 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#72 0x00007fd06cd163ca in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#73 0x00007fd06c2e06e9 in rustc_query_system::query::plumbing::get_query ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#74 0x00007fd06c338a14 in <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::evaluate_obligation ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#75 0x00007fd06c5685c9 in <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#76 0x00007fd06c5689e0 in <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#77 0x00007fd06c5850fd in rustc_trait_selection::traits::fulfill::FulfillProcessor::process_trait_obligation ()                                                                     [34/1837]
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#78 0x00007fd06c583cf6 in rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#79 0x00007fd06c59b86b in rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#80 0x00007fd06c58327c in rustc_trait_selection::traits::fulfill::FulfillmentContext::select::h5feaa97543394f08 ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#81 0x00007fd06c5838c6 in <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#82 0x00007fd06c29634d in rustc_infer::infer::canonical::query_response::_$LT$impl$u20$rustc_infer..infer..InferCtxt$GT$::make_canonicalized_query_response::h505642823a5ca85b ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#83 0x00007fd06c293555 in <rustc_infer::infer::InferCtxtBuilder as rustc_trait_selection::infer::InferCtxtBuilderExt>::enter_canonical_trait_query ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#84 0x00007fd06c299b09 in rustc_traits::type_op::type_op_prove_predicate::he50eda9b8e2602e2 ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#85 0x00007fd06cd7a808 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#86 0x00007fd06cd1629a in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#87 0x00007fd06c2f94dd in rustc_query_system::query::plumbing::get_query ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#88 0x00007fd06c338a51 in <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_op_prove_predicate ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#89 0x00007fd06c5c137e in rustc_trait_selection::traits::query::type_op::prove_predicate::<impl rustc_trait_selection::traits::query::type_op::QueryTypeOp for rustc_middle::traits::query::t
ype_op::ProvePredicate>::perform_query () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#90 0x00007fd06c1bbb8f in rustc_trait_selection::traits::query::type_op::QueryTypeOp::fully_perform_into ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#91 0x00007fd06c1e54b7 in <rustc_middle::ty::ParamEnvAnd<Q> as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#92 0x00007fd06c1f9d53 in rustc_borrowck::type_check::canonical::<impl rustc_borrowck::type_check::TypeChecker>::prove_predicate ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#93 0x00007fd06c1f980d in rustc_borrowck::type_check::canonical::<impl rustc_borrowck::type_check::TypeChecker>::normalize_and_prove_instantiated_predicates ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#94 0x00007fd06c1fe709 in <rustc_borrowck::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_constant ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#95 0x00007fd06c200bd3 in <rustc_borrowck::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_body ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#96 0x00007fd06c1facf0 in rustc_borrowck::type_check::type_check () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#97 0x00007fd06c1c65dc in rustc_borrowck::nll::compute_regions () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#98 0x00007fd06c1d562c in rustc_borrowck::do_mir_borrowck () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#99 0x00007fd06c1ad083 in rustc_infer::infer::InferCtxtBuilder::enter () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#100 0x00007fd06c1d4a44 in rustc_borrowck::mir_borrowck () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#101 0x00007fd06c1cfd91 in core::ops::function::FnOnce::call_once () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#102 0x00007fd06cd6b731 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#103 0x00007fd06cd13423 in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#104 0x00007fd06c2cccbd in rustc_query_system::query::plumbing::try_execute_query::hb362ed745d75a2d5 ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#105 0x00007fd06c336269 in <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#106 0x00007fd06bd691d8 in rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#107 0x00007fd06c768caa in rustc_interface::passes::analysis () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#108 0x00007fd06cd6fb7f in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#109 0x00007fd06cd13559 in rustc_data_structures::stack::ensure_sufficient_stack ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#110 0x00007fd06cc5749b in rustc_query_system::query::plumbing::try_execute_query::h784269645558384c ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#111 0x00007fd06cd2d9f2 in <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#112 0x00007fd06c75fb19 in rustc_interface::passes::QueryContext::enter ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#113 0x00007fd06c746af4 in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#114 0x00007fd06c733e15 in rustc_span::with_source_map () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#115 0x00007fd06c74641c in scoped_tls::ScopedKey<T>::set () from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#116 0x00007fd06c734e4d in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#117 0x00007fd06c7326b5 in core::ops::function::FnOnce::call_once{{vtable-shim}} ()
   from /home/khuey/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so
#118 0x00007fd06a3d9df3 in alloc::boxed::{impl#44}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> ()
    at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/alloc/src/boxed.rs:1638
#119 alloc::boxed::{impl#44}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> ()
    at /rustc/308dffd25cb55bbb4a1fbee9822cf82c6a5d012d/library/alloc/src/boxed.rs:1638
#120 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:106
#121 0x00007fd06a2e9609 in start_thread () from /lib/x86_64-linux-gnu/libpthread.so.0
#122 0x00007fd06a1fd293 in clone () from /lib/x86_64-linux-gnu/libc.so.6
