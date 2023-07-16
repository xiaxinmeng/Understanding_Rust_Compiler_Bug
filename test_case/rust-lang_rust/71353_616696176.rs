
   0: <rustc_middle::mir::interpret::error::InterpErrorInfo as core::convert::From<rustc_middle::mir::interpret::error::InterpError>>::from
             at src/librustc_middle/mir/interpret/error.rs:268
   1: rustc_mir::interpret::validity::ValidityVisitor<M>::try_visit_primitive
             at src/librustc_mir/interpret/validity.rs:486
   2: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_value
             at src/librustc_mir/interpret/validity.rs:662
      rustc_mir::interpret::validity::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::validate_operand_internal
             at src/librustc_mir/interpret/validity.rs:834
   3: rustc_mir::interpret::validity::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::const_validate_operand
             at src/librustc_mir/interpret/validity.rs:859
      rustc_mir::const_eval::eval_queries::validate_and_turn_into_const::{{closure}}
             at src/librustc_mir/const_eval/eval_queries.rs:192
      rustc_mir::const_eval::eval_queries::validate_and_turn_into_const
             at src/librustc_mir/const_eval/eval_queries.rs:184
      rustc_mir::const_eval::eval_queries::const_eval_validated_provider::{{closure}}
             at src/librustc_mir/const_eval/eval_queries.rs:257
      core::result::Result<T,E>::and_then
             at /home/r/src/rust/rustc/src/libcore/result.rs:729
      rustc_mir::const_eval::eval_queries::const_eval_validated_provider
             at src/librustc_mir/const_eval/eval_queries.rs:257
   4: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
             at /home/r/src/rust/rustc/src/librustc_middle/ty/query/plumbing.rs:362
   5: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at /home/r/src/rust/rustc/src/librustc_query_system/dep_graph/graph.rs:303
      rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at /home/r/src/rust/rustc/src/librustc_query_system/dep_graph/graph.rs:200
   6: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:593
      rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_middle/ty/query/plumbing.rs:71
      rustc_middle::ty::context::tls::enter_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1698
      rustc_middle::ty::context::tls::set_tlv
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1682
      rustc_middle::ty::context::tls::enter_context
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1698
      rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_middle/ty/query/plumbing.rs:71
      rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1786
      rustc_middle::ty::context::tls::with_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1770
      rustc_middle::ty::context::tls::with_context_opt
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1759
      rustc_middle::ty::context::tls::with_context
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1770
      rustc_middle::ty::context::tls::with_related_context
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1783
      rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at /home/r/src/rust/rustc/src/librustc_middle/ty/query/plumbing.rs:60
      rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:583
      rustc_query_system::query::plumbing::with_diagnostics
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:293
      rustc_query_system::query::plumbing::force_query_with_job
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:582
      rustc_query_system::query::plumbing::try_execute_query
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:410
      rustc_query_system::query::plumbing::get_query::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:627
      <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at /home/r/src/rust/rustc/src/librustc_query_system/query/caches.rs:91
      rustc_query_system::query::plumbing::try_get_cached
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:368
      rustc_query_system::query::plumbing::get_query
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:619
   7: rustc_middle::ty::query::TyCtxtAt::const_eval_validated
             at /home/r/src/rust/rustc/src/librustc_middle/ty/query/plumbing.rs:467
      rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_validated
             at /home/r/src/rust/rustc/src/librustc_middle/ty/query/plumbing.rs:430
      rustc_mir::const_eval::eval_queries::const_eval_validated_provider
             at src/librustc_mir/const_eval/eval_queries.rs:234
   8: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
             at src/librustc_middle/ty/query/plumbing.rs:362
   9: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at /home/r/src/rust/rustc/src/librustc_query_system/dep_graph/graph.rs:303
      rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at /home/r/src/rust/rustc/src/librustc_query_system/dep_graph/graph.rs:200
  10: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:593
      rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:71
      rustc_middle::ty::context::tls::enter_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1698
      rustc_middle::ty::context::tls::set_tlv
             at src/librustc_middle/ty/context.rs:1682
      rustc_middle::ty::context::tls::enter_context
             at src/librustc_middle/ty/context.rs:1698
      rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:71
      rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1786
      rustc_middle::ty::context::tls::with_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1770
      rustc_middle::ty::context::tls::with_context_opt
             at src/librustc_middle/ty/context.rs:1759
      rustc_middle::ty::context::tls::with_context
             at src/librustc_middle/ty/context.rs:1770
      rustc_middle::ty::context::tls::with_related_context
             at src/librustc_middle/ty/context.rs:1783
      rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at src/librustc_middle/ty/query/plumbing.rs:60
      rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:583
      rustc_query_system::query::plumbing::with_diagnostics
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:293
      rustc_query_system::query::plumbing::force_query_with_job
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:582
      rustc_query_system::query::plumbing::try_execute_query
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:410
      rustc_query_system::query::plumbing::get_query::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:627
      <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at /home/r/src/rust/rustc/src/librustc_query_system/query/caches.rs:91
      rustc_query_system::query::plumbing::try_get_cached
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:368
      rustc_query_system::query::plumbing::get_query
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:619
  11: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id
             at src/librustc_middle/mir/interpret/queries.rs:0
      rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve
             at src/librustc_middle/mir/interpret/queries.rs:45
  12: rustc_middle::ty::sty::Const::eval
             at /home/r/src/rust/rustc/src/librustc_middle/ty/sty.rs:2366
      <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
             at src/librustc_trait_selection/traits/query/normalize.rs:206
  13: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::sty::Const>::fold_with
             at /home/r/src/rust/rustc/src/librustc_middle/ty/structural_impls.rs:1005
      <rustc_middle::ty::subst::GenericArg as rustc_middle::ty::fold::TypeFoldable>::super_fold_with
             at /home/r/src/rust/rustc/src/librustc_middle/ty/subst.rs:158
      rustc_middle::ty::fold::TypeFoldable::fold_with
             at /home/r/src/rust/rustc/src/librustc_middle/ty/fold.rs:49
  14: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
             at /home/r/src/rust/rustc/src/librustc_trait_selection/traits/query/normalize.rs:62
  15: rustc_traits::normalize_erasing_regions::normalize_generic_arg_after_erasing_regions::{{closure}}
             at src/librustc_traits/normalize_erasing_regions.rs:24
      rustc_infer::infer::InferCtxtBuilder::enter::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_infer/infer/mod.rs:595
      rustc_middle::ty::context::GlobalCtxt::enter_local::{{closure}}::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1530
      rustc_middle::ty::context::tls::enter_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1698
      rustc_middle::ty::context::tls::set_tlv
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1682
      rustc_middle::ty::context::tls::enter_context
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1698
      rustc_middle::ty::context::GlobalCtxt::enter_local::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1530
      rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1786
      rustc_middle::ty::context::tls::with_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1770
      rustc_middle::ty::context::tls::with_context_opt
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1759
      rustc_middle::ty::context::tls::with_context
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1770
      rustc_middle::ty::context::tls::with_related_context
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1783
      rustc_middle::ty::context::GlobalCtxt::enter_local
             at /home/r/src/rust/rustc/src/librustc_middle/ty/context.rs:1522
  16: rustc_infer::infer::InferCtxtBuilder::enter
             at /home/r/src/rust/rustc/src/librustc_infer/infer/mod.rs:594
  17: rustc_traits::normalize_erasing_regions::normalize_generic_arg_after_erasing_regions
             at src/librustc_traits/normalize_erasing_regions.rs:22
  18: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::normalize_generic_arg_after_erasing_regions>::compute
             at src/librustc_middle/ty/query/plumbing.rs:362
  19: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at /home/r/src/rust/rustc/src/librustc_query_system/dep_graph/graph.rs:303
      rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at /home/r/src/rust/rustc/src/librustc_query_system/dep_graph/graph.rs:200
  20: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:593
      rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:71
      rustc_middle::ty::context::tls::enter_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1698
      rustc_middle::ty::context::tls::set_tlv
             at src/librustc_middle/ty/context.rs:1682
      rustc_middle::ty::context::tls::enter_context
             at src/librustc_middle/ty/context.rs:1698
      rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:71
      rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1786
      rustc_middle::ty::context::tls::with_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1770
      rustc_middle::ty::context::tls::with_context_opt
             at src/librustc_middle/ty/context.rs:1759
      rustc_middle::ty::context::tls::with_context
             at src/librustc_middle/ty/context.rs:1770
      rustc_middle::ty::context::tls::with_related_context
             at src/librustc_middle/ty/context.rs:1783
      rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at src/librustc_middle/ty/query/plumbing.rs:60
      rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:583
      rustc_query_system::query::plumbing::with_diagnostics
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:293
      rustc_query_system::query::plumbing::force_query_with_job
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:582
      rustc_query_system::query::plumbing::try_execute_query
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:410
      rustc_query_system::query::plumbing::get_query::{{closure}}
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:627
      <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at /home/r/src/rust/rustc/src/librustc_query_system/query/caches.rs:91
      rustc_query_system::query::plumbing::try_get_cached
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:368
      rustc_query_system::query::plumbing::get_query
             at /home/r/src/rust/rustc/src/librustc_query_system/query/plumbing.rs:619
  21: rustc_middle::ty::query::TyCtxtAt::normalize_generic_arg_after_erasing_regions
             at src/librustc_middle/ty/query/plumbing.rs:467
      rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::normalize_generic_arg_after_erasing_regions
             at src/librustc_middle/ty/query/plumbing.rs:430
      <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_const
             at src/librustc_middle/ty/normalize_erasing_regions.rs:103
  22: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::sty::Const>::fold_with
             at /home/r/src/rust/rustc/src/librustc_middle/ty/structural_impls.rs:1005
      rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
             at /home/r/src/rust/rustc/src/librustc_middle/ty/normalize_erasing_regions.rs:37
  23: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions
             at /home/r/src/rust/rustc/src/librustc_middle/ty/normalize_erasing_regions.rs:82
  24: rustc_mir::interpret::eval_context::InterpCx<M>::subst_from_frame_and_normalize_erasing_regions
             at src/librustc_mir/interpret/eval_context.rs:442
      rustc_mir::interpret::eval_context::InterpCx<M>::subst_from_current_frame_and_normalize_erasing_regions
             at src/librustc_mir/interpret/eval_context.rs:431
      rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand
             at src/librustc_mir/interpret/operand.rs:489
  25: rustc_mir::transform::const_prop::ConstPropagator::check_binary_op::{{closure}}
             at src/librustc_mir/transform/const_prop.rs:534
      rustc_mir::transform::const_prop::ConstPropagator::use_ecx
             at src/librustc_mir/transform/const_prop.rs:417
  26: rustc_mir::transform::const_prop::ConstPropagator::check_binary_op
             at src/librustc_mir/transform/const_prop.rs:534
  27: rustc_mir::transform::const_prop::ConstPropagator::const_prop
             at src/librustc_mir/transform/const_prop.rs:0
      <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_statement
             at src/librustc_mir/transform/const_prop.rs:836
  28: rustc_middle::mir::visit::MutVisitor::super_basic_block_data
             at /home/r/src/rust/rustc/src/librustc_middle/mir/visit.rs:322
      rustc_middle::mir::visit::MutVisitor::visit_basic_block_data
             at /home/r/src/rust/rustc/src/librustc_middle/mir/visit.rs:93
      rustc_middle::mir::visit::MutVisitor::super_body
             at /home/r/src/rust/rustc/src/librustc_middle/mir/visit.rs:275
      rustc_middle::mir::visit::MutVisitor::visit_body
             at /home/r/src/rust/rustc/src/librustc_middle/mir/visit.rs:87
  29: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
             at src/librustc_mir/transform/const_prop.rs:155
  30: rustc_mir::transform::run_passes::{{closure}}
             at src/librustc_mir/transform/mod.rs:165
      rustc_mir::transform::run_passes
             at src/librustc_mir/transform/mod.rs:172
  31: rustc_mir::transform::run_optimization_passes
             at src/librustc_mir/transform/mod.rs:270
  32: rustc_mir::transform::optimized_mir
             at src/librustc_mir/transform/mod.rs:337
