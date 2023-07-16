
error: internal compiler error: src/librustc_middle/ty/subst.rs:568:17: const parameter `D/#0` (Const { ty: usize, val: Param(D/#0) }/0) out of range when substituting substs=[]

thread 'rustc' panicked at 'Box<Any>', /home/joshua/rustc/library/std/src/macros.rs:13:23
stack backtrace:
   0: std::panicking::begin_panic
             at /home/joshua/rustc/library/std/src/panicking.rs:497
   1: rustc_errors::HandlerInner::span_bug
             at /home/joshua/rustc/library/std/src/macros.rs:13
   2: rustc_errors::Handler::span_bug
             at /home/joshua/rustc/src/librustc_errors/lib.rs:624
   3: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/util/bug.rs:32
   4: rustc_middle::ty::context::tls::with_opt::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1789
   5: rustc_middle::ty::context::tls::with_context_opt
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1741
   6: rustc_middle::ty::context::tls::with_opt
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1789
   7: rustc_middle::util::bug::opt_span_bug_fmt
             at /home/joshua/rustc/src/librustc_middle/util/bug.rs:29
   8: rustc_middle::util::bug::span_bug_fmt
             at /home/joshua/rustc/src/librustc_middle/util/bug.rs:21
   9: rustc_middle::ty::subst::SubstFolder::const_for_param
  10: <rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::TypeFolder>::fold_const
             at /home/joshua/rustc/src/librustc_middle/ty/subst.rs:501
  11: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::consts::Const>::fold_with
             at /home/joshua/rustc/src/librustc_middle/ty/structural_impls.rs:1087
  12: <T as rustc_middle::ty::subst::Subst>::subst_spanned
             at /home/joshua/rustc/src/librustc_middle/ty/subst.rs:428
  13: rustc_middle::ty::subst::Subst::subst
             at /home/joshua/rustc/src/librustc_middle/ty/subst.rs:409
  14: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions
             at /home/joshua/rustc/src/librustc_middle/ty/normalize_erasing_regions.rs:80
  15: rustc_mir::interpret::eval_context::InterpCx<M>::subst_from_frame_and_normalize_erasing_regions
             at /home/joshua/rustc/src/librustc_mir/interpret/eval_context.rs:476
  16: rustc_mir::interpret::eval_context::InterpCx<M>::subst_from_current_frame_and_normalize_erasing_regions
             at /home/joshua/rustc/src/librustc_mir/interpret/eval_context.rs:465
  17: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand
             at /home/joshua/rustc/src/librustc_mir/interpret/operand.rs:519
  18: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
             at /home/joshua/rustc/src/librustc_mir/interpret/step.rs:166
  19: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::statement
             at /home/joshua/rustc/src/librustc_mir/interpret/step.rs:89
  20: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step
             at /home/joshua/rustc/src/librustc_mir/interpret/step.rs:65
  21: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run
             at /home/joshua/rustc/src/librustc_mir/interpret/step.rs:34
  22: rustc_mir::const_eval::eval_queries::eval_body_using_ecx
             at /home/joshua/rustc/src/librustc_mir/const_eval/eval_queries.rs:57
  23: rustc_mir::const_eval::eval_queries::const_eval_raw_provider::{{closure}}
             at /home/joshua/rustc/src/librustc_mir/const_eval/eval_queries.rs:319
  24: core::result::Result<T,E>::and_then
             at /home/joshua/rustc/library/core/src/result.rs:708
  25: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
             at /home/joshua/rustc/src/librustc_mir/const_eval/eval_queries.rs:319
  26: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_raw>::compute
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:381
  27: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at /home/joshua/rustc/src/librustc_query_system/dep_graph/graph.rs:303
  28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at /home/joshua/rustc/src/librustc_query_system/dep_graph/graph.rs:200
  29: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:599
  30: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:72
  31: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.11/src/lib.rs:52
  32: rustc_data_structures::stack::ensure_sufficient_stack
             at /home/joshua/rustc/src/librustc_data_structures/stack.rs:16
  33: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:72
  34: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1724
  35: rustc_middle::ty::context::tls::set_tlv
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1708
  36: rustc_middle::ty::context::tls::enter_context
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1724
  37: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:71
  38: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1768
  39: rustc_middle::ty::context::tls::with_context::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1752
  40: rustc_middle::ty::context::tls::with_context_opt
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1741
  41: rustc_middle::ty::context::tls::with_context
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1752
  42: rustc_middle::ty::context::tls::with_related_context
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1765
  43: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:60
  44: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:589
  45: rustc_query_system::query::plumbing::with_diagnostics
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:296
  46: rustc_query_system::query::plumbing::force_query_with_job
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:588
  47: rustc_query_system::query::plumbing::try_execute_query
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:415
  48: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:639
  49: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at /home/joshua/rustc/src/librustc_query_system/query/caches.rs:110
  50: rustc_query_system::query::plumbing::try_get_cached
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:369
  51: rustc_query_system::query::plumbing::get_query_impl
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:631
  52: rustc_query_system::query::plumbing::get_query
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:731
  53: rustc_middle::ty::query::TyCtxtAt::const_eval_raw
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:491
  54: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_raw
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:452
  55: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
             at /home/joshua/rustc/src/librustc_mir/const_eval/eval_queries.rs:262
  56: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:381
  57: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at /home/joshua/rustc/src/librustc_query_system/dep_graph/graph.rs:303
  58: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at /home/joshua/rustc/src/librustc_query_system/dep_graph/graph.rs:200
  59: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:599
  60: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:72
  61: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.11/src/lib.rs:52
  62: rustc_data_structures::stack::ensure_sufficient_stack
             at /home/joshua/rustc/src/librustc_data_structures/stack.rs:16
  63: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:72
  64: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1724
  65: rustc_middle::ty::context::tls::set_tlv
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1708
  66: rustc_middle::ty::context::tls::enter_context
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1724
  67: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:71
  68: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1768
  69: rustc_middle::ty::context::tls::with_context::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1752
  70: rustc_middle::ty::context::tls::with_context_opt
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1741
  71: rustc_middle::ty::context::tls::with_context
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1752
  72: rustc_middle::ty::context::tls::with_related_context
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1765
  73: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:60
  74: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:589
  75: rustc_query_system::query::plumbing::with_diagnostics
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:296
  76: rustc_query_system::query::plumbing::force_query_with_job
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:588
  77: rustc_query_system::query::plumbing::try_execute_query
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:415
  78: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:639
  79: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at /home/joshua/rustc/src/librustc_query_system/query/caches.rs:110
  80: rustc_query_system::query::plumbing::try_get_cached
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:369
  81: rustc_query_system::query::plumbing::get_query_impl
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:631
  82: rustc_query_system::query::plumbing::get_query
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:731
  83: rustc_middle::ty::query::TyCtxtAt::const_eval_validated
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:491
  84: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_validated
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:452
  85: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
             at /home/joshua/rustc/src/librustc_mir/const_eval/eval_queries.rs:239
  86: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:381
  87: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at /home/joshua/rustc/src/librustc_query_system/dep_graph/graph.rs:303
  88: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at /home/joshua/rustc/src/librustc_query_system/dep_graph/graph.rs:200
  89: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:599
  90: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:72
  91: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.11/src/lib.rs:52
  92: rustc_data_structures::stack::ensure_sufficient_stack
             at /home/joshua/rustc/src/librustc_data_structures/stack.rs:16
  93: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:72
  94: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1724
  95: rustc_middle::ty::context::tls::set_tlv
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1708
  96: rustc_middle::ty::context::tls::enter_context
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1724
  97: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:71
  98: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1768
  99: rustc_middle::ty::context::tls::with_context::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1752
 100: rustc_middle::ty::context::tls::with_context_opt
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1741
 101: rustc_middle::ty::context::tls::with_context
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1752
 102: rustc_middle::ty::context::tls::with_related_context
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1765
 103: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at /home/joshua/rustc/src/librustc_middle/ty/query/plumbing.rs:60
 104: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:589
 105: rustc_query_system::query::plumbing::with_diagnostics
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:296
 106: rustc_query_system::query::plumbing::force_query_with_job
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:588
 107: rustc_query_system::query::plumbing::try_execute_query
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:415
 108: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:639
 109: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at /home/joshua/rustc/src/librustc_query_system/query/caches.rs:110
 110: rustc_query_system::query::plumbing::try_get_cached
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:369
 111: rustc_query_system::query::plumbing::get_query_impl
             at /home/joshua/rustc/src/librustc_query_system/query/plumbing.rs:631
 112: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id
 113: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_poly
             at /home/joshua/rustc/src/librustc_middle/mir/interpret/queries.rs:22
 114: <rustc_hir::hir::Ty as rustdoc::clean::Clean<rustdoc::clean::types::Type>>::clean
             at /home/joshua/rustc/src/librustdoc/clean/mod.rs:1367
 115: <rustdoc::doctree::Typedef as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean
             at /home/joshua/rustc/src/librustdoc/clean/mod.rs:2042
 116: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean::{{closure}}
             at /home/joshua/rustc/src/librustdoc/clean/mod.rs:247
 117: core::iter::adapters::map_fold::{{closure}}
             at /home/joshua/rustc/library/core/src/iter/adapters/mod.rs:825
 118: core::iter::traits::iterator::Iterator::fold
             at /home/joshua/rustc/library/core/src/iter/traits/iterator.rs:2004
 119: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
             at /home/joshua/rustc/library/core/src/iter/adapters/mod.rs:865
 120: core::iter::traits::iterator::Iterator::for_each
             at /home/joshua/rustc/library/core/src/iter/traits/iterator.rs:651
 121: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend
             at /home/joshua/rustc/library/alloc/src/vec.rs:2165
 122: <alloc::vec::Vec<T> as core::iter::traits::collect::Extend<T>>::extend
             at /home/joshua/rustc/library/alloc/src/vec.rs:2088
 123: <rustdoc::doctree::Module as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean
             at /home/joshua/rustc/src/librustdoc/clean/mod.rs:247
 124: rustdoc::clean::utils::krate
             at /home/joshua/rustc/src/librustdoc/clean/utils.rs:43
 125: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustdoc/core.rs:516
 126: rustc_interface::passes::QueryContext::enter::{{closure}}
             at /home/joshua/rustc/src/librustc_interface/passes.rs:721
 127: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1724
 128: rustc_middle::ty::context::tls::set_tlv
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1708
 129: rustc_middle::ty::context::tls::enter_context
             at /home/joshua/rustc/src/librustc_middle/ty/context.rs:1724
 130: rustc_interface::passes::QueryContext::enter
             at /home/joshua/rustc/src/librustc_interface/passes.rs:721
 131: rustdoc::core::run_core::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustdoc/core.rs:451
 132: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at /home/joshua/rustc/src/librustc_interface/queries.rs:385
 133: rustdoc::core::run_core::{{closure}}
             at /home/joshua/rustc/src/librustdoc/core.rs:413
 134: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at /home/joshua/rustc/src/librustc_interface/interface.rs:191
 135: rustc_span::with_source_map
             at /home/joshua/rustc/src/librustc_span/lib.rs:743
 136: rustc_interface::interface::create_compiler_and_run
             at /home/joshua/rustc/src/librustc_interface/interface.rs:185
 137: rustdoc::core::run_core
             at /home/joshua/rustc/src/librustdoc/core.rs:412
 138: rustdoc::main_options
             at /home/joshua/rustc/src/librustdoc/lib.rs:504
 139: rustdoc::main_args::{{closure}}
             at /home/joshua/rustc/src/librustdoc/lib.rs:438
 140: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at /home/joshua/rustc/src/librustc_interface/util.rs:148
 141: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 142: rustc_span::with_session_globals
             at /home/joshua/rustc/src/librustc_span/lib.rs:91
 143: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at /home/joshua/rustc/src/librustc_interface/util.rs:144
 144: rustc_interface::util::scoped_thread::{{closure}}
             at /home/joshua/rustc/src/librustc_interface/util.rs:119
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
