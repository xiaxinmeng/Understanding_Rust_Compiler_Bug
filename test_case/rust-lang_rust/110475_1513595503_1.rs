
  24:     0xffffa865f308 - <rustc_trait_selection[59d5fff7e50533b4]::traits::select::SelectionContext>::candidate_from_obligation
  25:     0xffffa8653904 - <rustc_trait_selection[59d5fff7e50533b4]::traits::select::SelectionContext>::select_from_obligation
  26:     0xffffa865e974 - <rustc_trait_selection[59d5fff7e50533b4]::traits::select::SelectionContext>::select
  27:     0xffffa8521564 - <rustc_trait_selection[59d5fff7e50533b4]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  28:     0xffffa8520820 - <rustc_trait_selection[59d5fff7e50533b4]::traits::fulfill::FulfillProcessor as rustc_data_structures[7f1a4c6933e5419]::obligation_forest::ObligationProcessor>::process_obligation
  29:     0xffffa84db470 - <rustc_data_structures[7f1a4c6933e5419]::obligation_forest::ObligationForest<rustc_trait_selection[59d5fff7e50533b4]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[59d5fff7e50533b4]::traits::fulfill::FulfillProcessor>
  30:     0xffffa851e104 - <rustc_trait_selection[59d5fff7e50533b4]::traits::fulfill::FulfillmentContext as rustc_infer[8facaf8255ab7cac]::traits::engine::TraitEngine>::select_where_possible
  31:     0xffffa77519f0 - <dyn rustc_infer[8facaf8255ab7cac]::traits::engine::TraitEngine as rustc_infer[8facaf8255ab7cac]::traits::engine::TraitEngineExt>::select_all_or_error
  32:     0xffffa77eeb58 - rustc_traits[88a431509884da59]::codegen::codegen_select_candidate
  33:     0xffffa7cbbb64 - rustc_query_system[7175af1eaca55fc6]::query::plumbing::try_execute_query::<rustc_query_impl[9bf706fa0936a2ef]::queries::codegen_select_candidate, rustc_query_impl[9bf706fa0936a2ef]::plumbing::QueryCtxt>
  34:     0xffffa7b789b8 - <rustc_query_impl[9bf706fa0936a2ef]::Queries as rustc_middle[c197f7dd5ada7ea7]::ty::query::QueryEngine>::codegen_select_candidate
  35:     0xffffa69d81b8 - rustc_ty_utils[e19ef8fcd720bdb9]::instance::inner_resolve_instance
  36:     0xffffa69d7758 - rustc_ty_utils[e19ef8fcd720bdb9]::instance::resolve_instance
  37:     0xffffa7c86000 - rustc_query_system[7175af1eaca55fc6]::query::plumbing::try_execute_query::<rustc_query_impl[9bf706fa0936a2ef]::queries::resolve_instance, rustc_query_impl[9bf706fa0936a2ef]::plumbing::QueryCtxt>
  38:     0xffffa7b859fc - <rustc_query_impl[9bf706fa0936a2ef]::Queries as rustc_middle[c197f7dd5ada7ea7]::ty::query::QueryEngine>::resolve_instance
  39:     0xffffa8885cd4 - <rustc_middle[c197f7dd5ada7ea7]::ty::instance::Instance>::resolve_opt_const_arg
  40:     0xffffa8885808 - <rustc_middle[c197f7dd5ada7ea7]::ty::instance::Instance>::resolve
  41:     0xffffa6f5e6b0 - rustc_mir_transform[d2e7dbab7faafc68]::inline::cycle::mir_callgraph_reachable::process
[ like 400 identical calls ]
 425:     0xffffa6f5e934 - rustc_mir_transform[d2e7dbab7faafc68]::inline::cycle::mir_callgraph_reachable::process
 426:     0xffffa6f5e2c4 - rustc_mir_transform[d2e7dbab7faafc68]::inline::cycle::mir_callgraph_reachable
 427:     0xffffa7cb6054 - rustc_query_system[7175af1eaca55fc6]::query::plumbing::try_execute_query::<rustc_query_impl[9bf706fa0936a2ef]::queries::mir_callgraph_reachable, rustc_query_impl[9bf706fa0936a2ef]::plumbing::QueryCtxt>
 428:     0xffffa7b73ee8 - <rustc_query_impl[9bf706fa0936a2ef]::Queries as rustc_middle[c197f7dd5ada7ea7]::ty::query::QueryEngine>::mir_callgraph_reachable
 429:     0xffffa6efde84 - <rustc_mir_transform[d2e7dbab7faafc68]::inline::Inliner>::try_inlining
 430:     0xffffa6efcf60 - <rustc_mir_transform[d2e7dbab7faafc68]::inline::Inliner>::process_blocks
 431:     0xffffa6efca28 - <rustc_mir_transform[d2e7dbab7faafc68]::inline::Inline as rustc_middle[c197f7dd5ada7ea7]::mir::MirPass>::run_pass
 432:     0xffffa6f396ec - rustc_mir_transform[d2e7dbab7faafc68]::pass_manager::run_passes_inner
 433:     0xffffa6e51ee8 - rustc_mir_transform[d2e7dbab7faafc68]::optimized_mir
