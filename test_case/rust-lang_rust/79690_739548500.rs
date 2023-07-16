
error: internal compiler error: compiler/rustc_mir/src/interpret/validity.rs:918:17: Unexpected error during validation: unable to turn pointer into raw bytes

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
stack backtrace:
   0: std::panicking::begin_panic
             at ./library/std/src/panicking.rs:519:12
   1: rustc_errors::HandlerInner::bug
             at ./compiler/rustc_errors/src/lib.rs:958:9
   2: rustc_errors::Handler::bug
             at ./compiler/rustc_errors/src/lib.rs:675:9
   3: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
             at ./compiler/rustc_middle/src/util/bug.rs:33:34
   4: rustc_middle::ty::context::tls::with_opt::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1785:40
   5: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1737:22
   6: rustc_middle::ty::context::tls::with_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1785:9
   7: rustc_middle::util::bug::opt_span_bug_fmt
             at ./compiler/rustc_middle/src/util/bug.rs:29:5
   8: rustc_middle::util::bug::bug_fmt
             at ./compiler/rustc_middle/src/util/bug.rs:14:5
   9: rustc_mir::interpret::validity::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::validate_operand_internal
             at ./compiler/rustc_mir/src/interpret/validity.rs:918:17
  10: rustc_mir::interpret::validity::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::const_validate_operand
             at ./compiler/rustc_mir/src/interpret/validity.rs:941:9
  11: rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider
             at ./compiler/rustc_mir/src/const_eval/eval_queries.rs:398:25
  12: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_allocation_raw>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  13: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:302:14
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:201:9
  15: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:608:17
  16: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  17: stacker::maybe_grow
             at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  18: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  19: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  20: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1720:50
  21: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1704:9
  22: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1720:9
  23: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  24: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1764:13
  25: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1748:40
  26: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1737:22
  27: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1748:9
  28: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
  29: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  30: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
  31: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
  32: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
  33: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
  34: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
  35: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
  36: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
  37: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
  38: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
  39: rustc_middle::ty::query::TyCtxtAt::eval_to_allocation_raw
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
  40: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::eval_to_allocation_raw
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
  41: rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider
             at ./compiler/rustc_mir/src/const_eval/eval_queries.rs:234:5
  42: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_const_value_raw>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  43: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:302:14
  44: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:201:9
  45: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:608:17
  46: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  47: stacker::maybe_grow
             at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  48: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  49: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  50: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1720:50
  51: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1704:9
  52: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1720:9
  53: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  54: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1764:13
  55: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1748:40
  56: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1737:22
  57: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1748:9
  58: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
  59: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  60: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
  61: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
  62: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
  63: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
  64: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
  65: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
  66: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
  67: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
  68: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
  69: rustc_middle::ty::query::TyCtxtAt::eval_to_const_value_raw
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
  70: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::eval_to_const_value_raw
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
  71: rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider
             at ./compiler/rustc_mir/src/const_eval/eval_queries.rs:211:15
  72: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_const_value_raw>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  73: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:302:14
  74: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:201:9
  75: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:608:17
  76: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  77: stacker::maybe_grow
             at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  78: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  79: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  80: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1720:50
  81: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1704:9
  82: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1720:9
  83: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  84: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1764:13
  85: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1748:40
  86: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1737:22
  87: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1748:9
  88: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
  89: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  90: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
  91: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
  92: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
  93: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
  94: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
  95: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
  96: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
  97: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
  98: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id
  99: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_poly
             at ./compiler/rustc_middle/src/mir/interpret/queries.rs:22:9
 100: <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item
             at ./compiler/rustc_lint/src/builtin.rs:1496:25
 101: <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item
             at ./compiler/rustc_lint/src/passes.rs:115:13
 102: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}::{{closure}}
             at ./compiler/rustc_lint/src/late.rs:145:17
 103: rustc_lint::late::LateContextAndPass<T>::with_param_env
             at ./compiler/rustc_lint/src/late.rs:75:9
 104: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}
             at ./compiler/rustc_lint/src/late.rs:144:13
 105: rustc_lint::late::LateContextAndPass<T>::with_lint_attrs
             at ./compiler/rustc_lint/src/late.rs:63:9
 106: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item
             at ./compiler/rustc_lint/src/late.rs:143:9
 107: rustc_hir::intravisit::Visitor::visit_nested_item
             at ./compiler/rustc_hir/src/intravisit.rs:273:9
 108: rustc_hir::intravisit::walk_mod
             at ./compiler/rustc_hir/src/intravisit.rs:500:9
 109: rustc_lint::late::LateContextAndPass<T>::process_mod
             at ./compiler/rustc_lint/src/late.rs:81:9
 110: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_mod
             at ./compiler/rustc_lint/src/late.rs:251:13
 111: rustc_hir::intravisit::walk_crate
             at ./compiler/rustc_hir/src/intravisit.rs:486:5
 112: rustc_lint::late::late_lint_pass_crate::{{closure}}
             at ./compiler/rustc_lint/src/late.rs:444:9
 113: rustc_lint::late::LateContextAndPass<T>::with_lint_attrs
             at ./compiler/rustc_lint/src/late.rs:63:9
 114: rustc_lint::late::late_lint_pass_crate
             at ./compiler/rustc_lint/src/late.rs:439:5
 115: rustc_lint::late::late_lint_crate
             at ./compiler/rustc_lint/src/late.rs:458:9
 116: rustc_lint::late::check_crate::{{closure}}::{{closure}}
             at ./compiler/rustc_lint/src/late.rs:488:17
 117: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:570:9
 118: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:9:9
 119: rustc_lint::late::check_crate::{{closure}}
             at ./compiler/rustc_lint/src/late.rs:486:13
 120: rustc_data_structures::sync::join
             at ./compiler/rustc_data_structures/src/sync.rs:159:14
 121: rustc_lint::late::check_crate
             at ./compiler/rustc_lint/src/late.rs:484:5
 122: rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:901:29
 123: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:570:9
 124: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:9:9
 125: rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:900:25
 126: core::ops::function::FnOnce::call_once
             at ./library/core/src/ops/function.rs:227:5
 127: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./library/std/src/panic.rs:322:9
 128: std::panicking::try::do_call
             at ./library/std/src/panicking.rs:379:40
 129: std::panicking::try
             at ./library/std/src/panicking.rs:343:19
 130: std::panic::catch_unwind
             at ./library/std/src/panic.rs:396:14
 131: rustc_interface::passes::analysis::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:887:17
 132: core::ops::function::FnOnce::call_once
             at ./library/core/src/ops/function.rs:227:5
 133: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./library/std/src/panic.rs:322:9
 134: std::panicking::try::do_call
             at ./library/std/src/panicking.rs:379:40
 135: std::panicking::try
             at ./library/std/src/panicking.rs:343:19
 136: std::panic::catch_unwind
             at ./library/std/src/panic.rs:396:14
 137: rustc_interface::passes::analysis::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:883:9
 138: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:570:9
 139: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:9:9
 140: rustc_interface::passes::analysis
             at ./compiler/rustc_interface/src/passes.rs:882:5
 141: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
 142: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:302:14
 143: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:335:9
 144: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:600:17
 145: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
 146: stacker::maybe_grow
             at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
 147: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
 148: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
 149: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1720:50
 150: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1704:9
 151: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1720:9
 152: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
 153: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1764:13
 154: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1748:40
 155: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1737:22
 156: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1748:9
 157: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
 158: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
 159: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
 160: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
 161: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
 162: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
 163: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
 164: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
 165: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
 166: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
 167: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
 168: rustc_middle::ty::query::TyCtxtAt::analysis
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
 169: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::analysis
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
 170: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:440:59
 171: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:725:42
 172: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1720:50
 173: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1704:9
 174: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1720:9
 175: rustc_interface::passes::QueryContext::enter
             at ./compiler/rustc_interface/src/passes.rs:725:9
 176: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:440:13
 177: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:413:19
 178: rustc_driver::run_compiler::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:341:22
 179: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:196:13
 180: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:765:5
 181: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:190:5
 182: rustc_interface::interface::run_compiler::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:212:12
 183: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:152:13
 184: scoped_tls::ScopedKey<T>::set
             at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 185: rustc_span::with_session_globals
             at ./compiler/rustc_span/src/lib.rs:94:5
 186: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:150:9
 187: rustc_interface::util::scoped_thread::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:125:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib

query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `G`
#1 [eval_to_const_value_raw] simplifying constant for the type system `G`
#2 [eval_to_const_value_raw] simplifying constant for the type system `G`
#3 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error; 6 warnings emitted
