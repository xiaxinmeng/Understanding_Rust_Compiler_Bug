
thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:990:27
stack backtrace:
   0: std::panicking::begin_panic
             at /home/joshua/rustc/library/std/src/panicking.rs:519:12
   1: rustc_errors::HandlerInner::panic_if_treat_err_as_bug
             at /home/joshua/rustc/compiler/rustc_errors/src/lib.rs:990:27
   2: rustc_errors::HandlerInner::bump_err_count
             at /home/joshua/rustc/compiler/rustc_errors/src/lib.rs:980:9
   3: rustc_errors::HandlerInner::emit_diagnostic
             at /home/joshua/rustc/compiler/rustc_errors/src/lib.rs:796:13
   4: rustc_errors::Handler::emit_diagnostic
             at /home/joshua/rustc/compiler/rustc_errors/src/lib.rs:715:9
   5: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
             at /home/joshua/rustc/compiler/rustc_errors/src/diagnostic_builder.rs:101:9
   6: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_overflow_error
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:206:9
   7: rustc_trait_selection::traits::select::SelectionContext::check_recursion_limit
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/select/mod.rs:956:21
   8: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs:32:9
   9: rustc_trait_selection::traits::select::SelectionContext::select
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/select/mod.rs:321:31
  10: rustc_trait_selection::traits::fulfill::FulfillProcessor::process_trait_obligation
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/fulfill.rs:623:15
  11: rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/fulfill.rs:391:21
  12: <rustc_trait_selection::traits::fulfill::FulfillProcessor as rustc_data_structures::obligation_forest::ObligationProcessor>::process_obligation
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/fulfill.rs:307:9
  13: rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
             at /home/joshua/rustc/compiler/rustc_data_structures/src/obligation_forest/mod.rs:448:19
  14: rustc_trait_selection::traits::fulfill::FulfillmentContext::select
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/fulfill.rs:133:17
  15: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/fulfill.rs:235:9
  16: rustc_infer::infer::canonical::query_response::<impl rustc_infer::infer::InferCtxt>::make_query_response
             at /home/joshua/rustc/compiler/rustc_infer/src/infer/canonical/query_response.rs:115:27
  17: rustc_infer::infer::canonical::query_response::<impl rustc_infer::infer::InferCtxt>::make_canonicalized_query_response
             at /home/joshua/rustc/compiler/rustc_infer/src/infer/canonical/query_response.rs:61:30
  18: <rustc_infer::infer::InferCtxtBuilder as rustc_trait_selection::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::{{closure}}
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/infer.rs:129:17
  19: rustc_infer::infer::InferCtxtBuilder::enter_with_canonical::{{closure}}
             at /home/joshua/rustc/compiler/rustc_infer/src/infer/mod.rs:576:13
  20: rustc_infer::infer::InferCtxtBuilder::enter
             at /home/joshua/rustc/compiler/rustc_infer/src/infer/mod.rs:583:9
  21: rustc_infer::infer::InferCtxtBuilder::enter_with_canonical
             at /home/joshua/rustc/compiler/rustc_infer/src/infer/mod.rs:573:9
  22: <rustc_infer::infer::InferCtxtBuilder as rustc_trait_selection::infer::InferCtxtBuilderExt>::enter_canonical_trait_query
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/infer.rs:123:9
  23: rustc_traits::normalize_projection_ty::normalize_projection_ty
             at /home/joshua/rustc/compiler/rustc_traits/src/normalize_projection_ty.rs:24:5
  24: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::normalize_projection_ty>::compute
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  25: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at /home/joshua/rustc/compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
  26: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at /home/joshua/rustc/compiler/rustc_query_system/src/dep_graph/graph.rs:200:9
  27: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at /home/joshua/rustc/compiler/rustc_query_system/src/query/plumbing.rs:608:17
  28: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  29: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  30: rustc_data_structures::stack::ensure_sufficient_stack
             at /home/joshua/rustc/compiler/rustc_data_structures/src/stack.rs:16:5
  31: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  32: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1739:50
  33: rustc_middle::ty::context::tls::set_tlv
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1723:9
  34: rustc_middle::ty::context::tls::enter_context
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1739:9
  35: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  36: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1783:13
  37: rustc_middle::ty::context::tls::with_context::{{closure}}
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1767:40
  38: rustc_middle::ty::context::tls::with_context_opt
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1756:22
  39: rustc_middle::ty::context::tls::with_context
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1767:9
  40: rustc_middle::ty::context::tls::with_related_context
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1780:9
  41: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  42: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at /home/joshua/rustc/compiler/rustc_query_system/src/query/plumbing.rs:598:9
  43: rustc_query_system::query::plumbing::with_diagnostics
             at /home/joshua/rustc/compiler/rustc_query_system/src/query/plumbing.rs:302:18
  44: rustc_query_system::query::plumbing::force_query_with_job
             at /home/joshua/rustc/compiler/rustc_query_system/src/query/plumbing.rs:597:51
  45: rustc_query_system::query::plumbing::try_execute_query
             at /home/joshua/rustc/compiler/rustc_query_system/src/query/plumbing.rs:426:16
  46: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at /home/joshua/rustc/compiler/rustc_query_system/src/query/plumbing.rs:645:23
  47: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at /home/joshua/rustc/compiler/rustc_query_system/src/query/caches.rs:114:79
  48: rustc_query_system::query::plumbing::try_get_cached
             at /home/joshua/rustc/compiler/rustc_query_system/src/query/plumbing.rs:379:5
  49: rustc_query_system::query::plumbing::get_query_impl
             at /home/joshua/rustc/compiler/rustc_query_system/src/query/plumbing.rs:637:5
  50: rustc_query_system::query::plumbing::get_query
             at /home/joshua/rustc/compiler/rustc_query_system/src/query/plumbing.rs:739:5
  51: rustc_middle::ty::query::TyCtxtAt::normalize_projection_ty
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
  52: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::normalize_projection_ty
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
  53: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty::{{closure}}
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/query/normalize.rs:172:23
  54: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/query/normalize.rs:111:19
  55: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::fold_with
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/structural_impls.rs:969:9
  56: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
             at /home/joshua/rustc/compiler/rustc_trait_selection/src/traits/query/normalize.rs:65:22
  57: rustdoc::clean::normalize::{{closure}}
             at /home/joshua/rustc/src/librustdoc/clean/mod.rs:1514:9
  58: rustc_infer::infer::InferCtxtBuilder::enter
             at /home/joshua/rustc/compiler/rustc_infer/src/infer/mod.rs:583:9
  59: rustdoc::clean::normalize
             at /home/joshua/rustc/src/librustdoc/clean/mod.rs:1513:22
  60: <&rustc_middle::ty::TyS as rustdoc::clean::Clean<rustdoc::clean::types::Type>>::clean
             at /home/joshua/rustc/src/librustdoc/clean/mod.rs:1534:18
  61: <(rustc_span::def_id::DefId,rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig>) as rustdoc::clean::Clean<rustdoc::clean::types::FnDecl>>::clean
             at /home/joshua/rustc/src/librustdoc/clean/mod.rs:996:28
  62: <rustc_middle::ty::AssocItem as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean
             at /home/joshua/rustc/src/librustdoc/clean/mod.rs:1152:32
  63: rustdoc::clean::inline::build_impl::{{closure}}
             at /home/joshua/rustc/src/librustdoc/clean/inline.rs:410:30
  64: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
             at /home/joshua/rustc/library/core/src/ops/function.rs:269:13
  65: core::iter::traits::iterator::Iterator::find_map::check::{{closure}}
             at /home/joshua/rustc/library/core/src/iter/traits/iterator.rs:2257:32
  66: core::iter::adapters::map::map_try_fold::{{closure}}
             at /home/joshua/rustc/library/core/src/iter/adapters/map.rs:87:21
  67: core::iter::adapters::map::map_try_fold::{{closure}}
             at /home/joshua/rustc/library/core/src/iter/adapters/map.rs:87:21
  68: core::iter::traits::iterator::Iterator::try_fold
             at /home/joshua/rustc/library/core/src/iter/traits/iterator.rs:1888:21
  69: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold
             at /home/joshua/rustc/library/core/src/iter/adapters/map.rs:113:9
  70: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold
             at /home/joshua/rustc/library/core/src/iter/adapters/map.rs:113:9
  71: core::iter::traits::iterator::Iterator::find_map
             at /home/joshua/rustc/library/core/src/iter/traits/iterator.rs:2263:9
  72: <core::iter::adapters::filter_map::FilterMap<I,F> as core::iter::traits::iterator::Iterator>::next
             at /home/joshua/rustc/library/core/src/iter/adapters/filter_map.rs:61:9
  73: alloc::vec::Vec<T,A>::extend_desugared
             at /home/joshua/rustc/library/alloc/src/vec.rs:2618:35
  74: <alloc::vec::Vec<T,A> as alloc::vec::SpecExtend<T,I>>::spec_extend
             at /home/joshua/rustc/library/alloc/src/vec.rs:2541:9
  75: <alloc::vec::Vec<T> as alloc::vec::SpecFromIterNested<T,I>>::from_iter
             at /home/joshua/rustc/library/alloc/src/vec.rs:2320:9
  76: <alloc::vec::Vec<T> as alloc::vec::SpecFromIter<T,I>>::from_iter
             at /home/joshua/rustc/library/alloc/src/vec.rs:2343:9
  77: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at /home/joshua/rustc/library/alloc/src/vec.rs:2181:9
  78: core::iter::traits::iterator::Iterator::collect
             at /home/joshua/rustc/library/core/src/iter/traits/iterator.rs:1670:9
  79: rustdoc::clean::inline::build_impl
             at /home/joshua/rustc/src/librustdoc/clean/inline.rs:406:13
  80: rustdoc::passes::collect_trait_impls::collect_trait_impls::{{closure}}
             at /home/joshua/rustc/src/librustdoc/passes/collect_trait_impls.rs:33:17
  81: rustc_data_structures::profiling::VerboseTimingGuard::run
             at /home/joshua/rustc/compiler/rustc_data_structures/src/profiling.rs:570:9
  82: rustc_session::utils::<impl rustc_session::session::Session>::time
             at /home/joshua/rustc/compiler/rustc_session/src/utils.rs:9:9
  83: rustdoc::passes::collect_trait_impls::collect_trait_impls
             at /home/joshua/rustc/src/librustdoc/passes/collect_trait_impls.rs:32:13
  84: rustdoc::core::run_global_ctxt::{{closure}}
             at /home/joshua/rustc/src/librustdoc/core.rs:655:56
  85: rustc_data_structures::profiling::VerboseTimingGuard::run
             at /home/joshua/rustc/compiler/rustc_data_structures/src/profiling.rs:570:9
  86: rustc_session::utils::<impl rustc_session::session::Session>::time
             at /home/joshua/rustc/compiler/rustc_session/src/utils.rs:9:9
  87: rustdoc::core::run_global_ctxt
             at /home/joshua/rustc/src/librustdoc/core.rs:655:21
  88: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustdoc/core.rs:469:21
  89: rustc_interface::passes::QueryContext::enter::{{closure}}
             at /home/joshua/rustc/compiler/rustc_interface/src/passes.rs:725:42
  90: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1739:50
  91: rustc_middle::ty::context::tls::set_tlv
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1723:9
  92: rustc_middle::ty::context::tls::enter_context
             at /home/joshua/rustc/compiler/rustc_middle/src/ty/context.rs:1739:9
  93: rustc_interface::passes::QueryContext::enter
             at /home/joshua/rustc/compiler/rustc_interface/src/passes.rs:725:9
  94: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustdoc/core.rs:468:17
  95: rustc_data_structures::profiling::VerboseTimingGuard::run
             at /home/joshua/rustc/compiler/rustc_data_structures/src/profiling.rs:570:9
  96: rustc_session::utils::<impl rustc_session::session::Session>::time
             at /home/joshua/rustc/compiler/rustc_session/src/utils.rs:9:9
  97: rustdoc::core::run_core::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustdoc/core.rs:467:46
  98: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at /home/joshua/rustc/compiler/rustc_interface/src/queries.rs:415:19
  99: rustdoc::core::run_core::{{closure}}
             at /home/joshua/rustc/src/librustdoc/core.rs:426:9
 100: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at /home/joshua/rustc/compiler/rustc_interface/src/interface.rs:196:13
 101: rustc_span::with_source_map
             at /home/joshua/rustc/compiler/rustc_span/src/lib.rs:765:5
 102: rustc_interface::interface::create_compiler_and_run
             at /home/joshua/rustc/compiler/rustc_interface/src/interface.rs:190:5
 103: rustdoc::core::run_core
             at /home/joshua/rustc/src/librustdoc/core.rs:425:5
 104: rustdoc::main_options
             at /home/joshua/rustc/src/librustdoc/lib.rs:531:53
 105: rustdoc::main_args::{{closure}}
             at /home/joshua/rustc/src/librustdoc/lib.rs:464:17
 106: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at /home/joshua/rustc/compiler/rustc_interface/src/util.rs:152:13
 107: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 108: rustc_span::with_session_globals
             at /home/joshua/rustc/compiler/rustc_span/src/lib.rs:94:5
 109: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at /home/joshua/rustc/compiler/rustc_interface/src/util.rs:150:9
 110: rustc_interface::util::scoped_thread::{{closure}}
             at /home/joshua/rustc/compiler/rustc_interface/src/util.rs:125:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
