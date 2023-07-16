
error[E0275]: overflow evaluating the requirement `<Source as SelectDsl<Selection>>::Output == _`

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:990:27
stack backtrace:
   0: std::panicking::begin_panic
             at ./library/std/src/panicking.rs:519:12
   1: rustc_errors::HandlerInner::panic_if_treat_err_as_bug
             at ./compiler/rustc_errors/src/lib.rs:990:27
   2: rustc_errors::HandlerInner::bump_err_count
             at ./compiler/rustc_errors/src/lib.rs:980:9
   3: rustc_errors::HandlerInner::emit_diagnostic
             at ./compiler/rustc_errors/src/lib.rs:796:13
   4: rustc_errors::Handler::emit_diagnostic
             at ./compiler/rustc_errors/src/lib.rs:715:9
   5: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
             at ./compiler/rustc_errors/src/diagnostic_builder.rs:101:9
   6: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_overflow_error
             at ./compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:206:9
   7: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_overflow_error_cycle
             at ./compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:222:9
   8: <rustc_trait_selection::traits::fulfill::FulfillProcessor as rustc_data_structures::obligation_forest::ObligationProcessor>::process_backedge
             at ./compiler/rustc_trait_selection/src/traits/fulfill.rs:321:13
   9: rustc_data_structures::obligation_forest::ObligationForest<O>::find_cycles_from_node
             at ./compiler/rustc_data_structures/src/obligation_forest/mod.rs:598:21
  10: rustc_data_structures::obligation_forest::ObligationForest<O>::find_cycles_from_node
             at ./compiler/rustc_data_structures/src/obligation_forest/mod.rs:591:25
  11: rustc_data_structures::obligation_forest::ObligationForest<O>::process_cycles
             at ./compiler/rustc_data_structures/src/obligation_forest/mod.rs:573:17
  12: rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
             at ./compiler/rustc_data_structures/src/obligation_forest/mod.rs:478:13
  13: rustc_trait_selection::traits::fulfill::FulfillmentContext::select
             at ./compiler/rustc_trait_selection/src/traits/fulfill.rs:133:17
  14: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
             at ./compiler/rustc_trait_selection/src/traits/fulfill.rs:235:9
  15: rustc_infer::infer::canonical::query_response::<impl rustc_infer::infer::InferCtxt>::make_query_response
             at ./compiler/rustc_infer/src/infer/canonical/query_response.rs:115:27
  16: rustc_infer::infer::canonical::query_response::<impl rustc_infer::infer::InferCtxt>::make_canonicalized_query_response
             at ./compiler/rustc_infer/src/infer/canonical/query_response.rs:61:30
  17: <rustc_infer::infer::InferCtxtBuilder as rustc_trait_selection::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::{{closure}}
             at ./compiler/rustc_trait_selection/src/infer.rs:129:17
  18: rustc_infer::infer::InferCtxtBuilder::enter_with_canonical::{{closure}}
             at ./compiler/rustc_infer/src/infer/mod.rs:576:13
  19: rustc_infer::infer::InferCtxtBuilder::enter
             at ./compiler/rustc_infer/src/infer/mod.rs:583:9
  20: rustc_infer::infer::InferCtxtBuilder::enter_with_canonical
             at ./compiler/rustc_infer/src/infer/mod.rs:573:9
  21: <rustc_infer::infer::InferCtxtBuilder as rustc_trait_selection::infer::InferCtxtBuilderExt>::enter_canonical_trait_query
             at ./compiler/rustc_trait_selection/src/infer.rs:123:9
  22: rustc_traits::normalize_projection_ty::normalize_projection_ty
             at ./compiler/rustc_traits/src/normalize_projection_ty.rs:24:5
  23: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::normalize_projection_ty>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  24: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
  25: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:200:9
  26: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:608:17
  27: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  28: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  29: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  30: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  31: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1739:50
  32: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1723:9
  33: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1739:9
  34: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  35: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1783:13
  36: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1767:40
  37: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1756:22
  38: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1767:9
  39: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1780:9
  40: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  41: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
  42: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
  43: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
  44: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
  45: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
  46: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
  47: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
  48: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
  49: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
  50: rustc_middle::ty::query::TyCtxtAt::normalize_projection_ty
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
  51: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::normalize_projection_ty
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
  52: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty::{{closure}}
             at ./compiler/rustc_trait_selection/src/traits/query/normalize.rs:172:23
  53: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
             at ./compiler/rustc_trait_selection/src/traits/query/normalize.rs:111:19
  54: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::fold_with
             at ./compiler/rustc_middle/src/ty/structural_impls.rs:969:9
  55: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
             at ./compiler/rustc_trait_selection/src/traits/query/normalize.rs:65:22
  56: rustdoc::clean::normalize::{{closure}}
             at ./src/librustdoc/clean/mod.rs:1514:9
  57: rustc_infer::infer::InferCtxtBuilder::enter
             at ./compiler/rustc_infer/src/infer/mod.rs:583:9
  58: rustdoc::clean::normalize
             at ./src/librustdoc/clean/mod.rs:1513:22
  59: rustdoc::clean::clean_qpath
             at ./src/librustdoc/clean/mod.rs:1404:45
  60: <rustc_hir::hir::Ty as rustdoc::clean::Clean<rustdoc::clean::types::Type>>::clean
             at ./src/librustdoc/clean/mod.rs:1477:32
  61: <(&rustc_hir::hir::Item,core::option::Option<rustc_span::symbol::Ident>) as rustdoc::clean::Clean<alloc::vec::Vec<rustdoc::clean::types::Item>>>::clean::{{closure}}
             at ./src/librustdoc/clean/mod.rs:2005:38
  62: rustdoc::core::DocContext::with_param_env
             at ./src/librustdoc/core.rs:88:19
  63: <(&rustc_hir::hir::Item,core::option::Option<rustc_span::symbol::Ident>) as rustdoc::clean::Clean<alloc::vec::Vec<rustdoc::clean::types::Item>>>::clean
             at ./src/librustdoc/clean/mod.rs:1987:9
  64: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean::{{closure}}
             at ./src/librustdoc/clean/mod.rs:235:48
  65: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
             at ./library/core/src/ops/function.rs:280:13
  66: core::option::Option<T>::map
             at ./library/core/src/option.rs:453:29
  67: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::next
             at ./library/core/src/iter/adapters/map.rs:99:9
  68: <core::iter::adapters::fuse::Fuse<I> as core::iter::adapters::fuse::FuseImpl<I>>::next
             at ./library/core/src/iter/adapters/fuse.rs:407:9
  69: <core::iter::adapters::fuse::Fuse<I> as core::iter::traits::iterator::Iterator>::next
             at ./library/core/src/iter/adapters/fuse.rs:65:9
  70: <core::iter::adapters::flatten::FlattenCompat<I,U> as core::iter::traits::iterator::Iterator>::next
             at ./library/core/src/iter/adapters/flatten.rs:268:19
  71: <core::iter::adapters::flatten::Flatten<I> as core::iter::traits::iterator::Iterator>::next
             at ./library/core/src/iter/adapters/flatten.rs:169:9
  72: alloc::vec::Vec<T,A>::extend_desugared
             at ./library/alloc/src/vec.rs:2618:35
  73: <alloc::vec::Vec<T,A> as alloc::vec::SpecExtend<T,I>>::spec_extend
             at ./library/alloc/src/vec.rs:2541:9
  74: <alloc::vec::Vec<T,A> as core::iter::traits::collect::Extend<T>>::extend
             at ./library/alloc/src/vec.rs:2251:9
  75: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean
             at ./src/librustdoc/clean/mod.rs:235:9
  76: rustdoc::clean::utils::krate
             at ./src/librustdoc/clean/utils.rs:44:22
  77: rustdoc::core::run_global_ctxt::{{closure}}
             at ./src/librustdoc/core.rs:560:53
  78: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:570:9
  79: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:9:9
  80: rustdoc::core::run_global_ctxt
             at ./src/librustdoc/core.rs:560:21
  81: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustdoc/core.rs:469:21
  82: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:725:42
  83: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1739:50
  84: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1723:9
  85: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1739:9
  86: rustc_interface::passes::QueryContext::enter
             at ./compiler/rustc_interface/src/passes.rs:725:9
  87: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustdoc/core.rs:468:17
  88: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:570:9
  89: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:9:9
  90: rustdoc::core::run_core::{{closure}}::{{closure}}
             at ./src/librustdoc/core.rs:467:46
  91: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:415:19
  92: rustdoc::core::run_core::{{closure}}
             at ./src/librustdoc/core.rs:426:9
  93: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:196:13
  94: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:765:5
  95: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:190:5
  96: rustdoc::core::run_core
             at ./src/librustdoc/core.rs:425:5
  97: rustdoc::main_options
             at ./src/librustdoc/lib.rs:531:53
  98: rustdoc::main_args::{{closure}}
             at ./src/librustdoc/lib.rs:464:17
  99: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:152:13
 100: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 101: rustc_span::with_session_globals
             at ./compiler/rustc_span/src/lib.rs:94:5
 102: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:150:9
 103: rustc_interface::util::scoped_thread::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:125:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
