
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /home/joshua/src/rust/src/librustc_hir/definitions.rs:358:9
stack backtrace:
 Documenting rustc_middle v0.0.0 (/home/joshua/src/rust/src/librustc_middle)
   0: backtrace::backtrace::libunwind::trace
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1076
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  11: rust_begin_unwind
             at src/libstd/panicking.rs:385
  12: core::panicking::panic_fmt
             at src/libcore/panicking.rs:86
  13: core::panicking::panic
             at src/libcore/panicking.rs:51
  14: rustc_middle::hir::map::Map::def_kind
             at src/librustc_middle/hir/map/mod.rs:0
  15: rustc_middle::hir::map::provide::{{closure}}
             at src/librustc_middle/hir/map/mod.rs:1071
  16: core::ops::function::FnOnce::call_once
             at ./src/libcore/ops/function.rs:232
  17: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
    Checking rustc_expand v0.0.0 (/home/joshua/src/rust/src/librustc_expand)
  18: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/dep_graph/graph.rs:0
  19: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:72
  20: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
  21: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
  22: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:72
  23: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1702
  24: rustc_middle::ty::context::tls::set_tlv
             at src/librustc_middle/ty/context.rs:1686
  25: rustc_middle::ty::context::tls::enter_context
             at src/librustc_middle/ty/context.rs:1702
  26: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:71
  27: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1790
  28: rustc_middle::ty::context::tls::with_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1774
  29: rustc_middle::ty::context::tls::with_context_opt
             at src/librustc_middle/ty/context.rs:1763
  30: rustc_middle::ty::context::tls::with_context
             at src/librustc_middle/ty/context.rs:1774
  31: rustc_middle::ty::context::tls::with_related_context
             at src/librustc_middle/ty/context.rs:1787
  32: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at src/librustc_middle/ty/query/plumbing.rs:60
  33: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:589
  34: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:296
  35: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:588
  36: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:415
  37: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:639
  38: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:111
  39: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:369
  40: rustc_query_system::query::plumbing::get_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:631
  41: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:731
  42: rustc_middle::ty::query::TyCtxtAt::def_kind
             at src/librustc_middle/ty/query/plumbing.rs:472
  43: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::def_kind
             at src/librustc_middle/ty/query/plumbing.rs:433
  44: rustc_middle::ty::util::<impl rustc_middle::ty::context::TyCtxt>::is_closure
             at src/librustc_middle/ty/util.rs:449
  45: rustc_middle::ty::util::<impl rustc_middle::ty::context::TyCtxt>::closure_base_def_id
             at src/librustc_middle/ty/util.rs:478
  46: rustc_typeck::collect::generics_of
             at src/librustc_typeck/collect.rs:1199
  47: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::generics_of>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:362
  48: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
  49: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/dep_graph/graph.rs:0
  50: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
  51: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
  52: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
  53: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
  54: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1702
  55: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1686
  56: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1702
  57: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
  58: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1790
  59: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1774
  60: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1763
  61: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1774
  62: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1787
  63: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
  64: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:589
  65: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:296
  66: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:588
  67: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:415
  68: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:639
  69: <rustc_query_system::query::caches::ArenaCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:193
  70: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:369
  71: rustc_query_system::query::plumbing::get_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:631
  72: rustc_query_system::query::plumbing::ensure_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:0
  73: rustc_query_system::query::plumbing::ensure_query
             at ./src/librustc_query_system/query/plumbing.rs:741
  74: rustc_middle::ty::query::TyCtxtEnsure::generics_of
             at ./src/librustc_middle/ty/query/plumbing.rs:389
  75: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             at src/librustc_typeck/collect.rs:225
  76: rustc_hir::intravisit::walk_stmt
             at ./src/librustc_hir/intravisit.rs:1051
  77: rustc_hir::intravisit::walk_block
             at ./src/librustc_ast/visit.rs:237
  78: rustc_hir::intravisit::Visitor::visit_block
             at ./src/librustc_hir/intravisit.rs:337
  79: rustc_hir::intravisit::walk_expr
             at ./src/librustc_hir/intravisit.rs:1125
  80: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             at src/librustc_typeck/collect.rs:228
  81: rustc_hir::intravisit::walk_stmt
             at ./src/librustc_hir/intravisit.rs:1051
  82: rustc_hir::intravisit::walk_block
             at ./src/librustc_ast/visit.rs:237
  83: rustc_hir::intravisit::Visitor::visit_block
             at ./src/librustc_hir/intravisit.rs:337
  84: rustc_hir::intravisit::walk_expr
             at ./src/librustc_hir/intravisit.rs:1125
  85: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             at src/librustc_typeck/collect.rs:228
  86: rustc_hir::intravisit::walk_body
             at ./src/librustc_hir/intravisit.rs:486
  87: rustc_hir::intravisit::Visitor::visit_body
             at ./src/librustc_hir/intravisit.rs:303
  88: rustc_hir::intravisit::Visitor::visit_nested_body
             at ./src/librustc_ast/visit.rs:237
  89: rustc_hir::intravisit::walk_fn
             at ./src/librustc_hir/intravisit.rs:927
  90: rustc_hir::intravisit::Visitor::visit_fn
             at ./src/librustc_hir/intravisit.rs:370
  91: rustc_hir::intravisit::walk_item
             at ./src/librustc_hir/intravisit.rs:561
  92: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
             at src/librustc_typeck/collect.rs:201
  93: <rustc_hir::intravisit::DeepVisitor<V> as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item
             at ./src/librustc_hir/intravisit.rs:57
  94: rustc_middle::hir::map::Map::visit_item_likes_in_module
             at ./src/librustc_middle/hir/map/mod.rs:468
  95: rustc_typeck::collect::collect_mod_item_types
             at src/librustc_typeck/collect.rs:58
  96: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_mod_item_types>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:362
  97: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
  98: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/dep_graph/graph.rs:0
  99: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
 100: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
 101: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
 102: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
 103: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1702
 104: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1686
 105: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1702
 106: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 107: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1790
 108: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1774
 109: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1763
 110: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1774
 111: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1787
 112: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
 113: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:589
 114: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:296
 115: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:588
 116: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:415
 117: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:639
 118: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:111
 119: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:369
 120: rustc_query_system::query::plumbing::get_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:631
 121: rustc_query_system::query::plumbing::ensure_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:679
 122: rustc_query_system::query::plumbing::ensure_query
             at ./src/librustc_query_system/query/plumbing.rs:741
 123: rustc_middle::ty::query::TyCtxtEnsure::collect_mod_item_types
             at ./src/librustc_middle/ty/query/plumbing.rs:389
 124: rustc_typeck::check_crate::{{closure}}::{{closure}}
             at src/librustc_typeck/lib.rs:330
 125: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./src/librustc_data_structures/profiling.rs:573
 126: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./src/librustc_session/utils.rs:9
 127: rustc_typeck::check_crate::{{closure}}
             at src/librustc_typeck/lib.rs:328
 128: rustc_session::session::Session::track_errors
             at ./src/librustc_session/session.rs:415
 129: rustc_typeck::check_crate
             at src/librustc_typeck/lib.rs:327
 130: rustc_interface::passes::analysis
             at src/librustc_interface/passes.rs:816
 131: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:362
 132: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
 133: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/dep_graph/graph.rs:0
 134: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
 135: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
 136: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
 137: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
 138: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1702
 139: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1686
 140: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1702
 141: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 142: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1790
 143: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1774
 144: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1763
 145: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1774
 146: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1787
 147: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
 148: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:589
 149: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:296
 150: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:588
 151: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:415
 152: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:639
 153: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:111
 154: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:369
 155: rustc_query_system::query::plumbing::get_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:631
 156: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:731
 157: rustc_middle::ty::query::TyCtxtAt::analysis
             at ./src/librustc_middle/ty/query/plumbing.rs:472
 158: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::analysis
             at ./src/librustc_middle/ty/query/plumbing.rs:433
 159: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}
             at src/librustdoc/core.rs:412
 160: rustc_middle::ty::context::tls::enter_global::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1725
 161: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1702
 162: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1686
 163: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1702
 164: rustc_middle::ty::context::tls::enter_global
             at ./src/librustc_middle/ty/context.rs:1725
 165: rustc_interface::passes::QueryContext::enter
             at ./src/librustc_interface/passes.rs:717
 166: rustdoc::core::run_core::{{closure}}::{{closure}}
             at src/librustdoc/core.rs:411
 167: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./src/librustc_interface/queries.rs:385
 168: rustdoc::core::run_core::{{closure}}
             at src/librustdoc/core.rs:373
 169: rustc_interface::interface::run_compiler_in_existing_thread_pool::{{closure}}
             at ./src/librustc_interface/interface.rs:195
 170: rustc_span::with_source_map
             at ./src/librustc_span/lib.rs:728
 171: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at ./src/librustc_interface/interface.rs:189
 172: rustdoc::core::run_core
             at src/librustdoc/core.rs:372
 173: rustdoc::rust_input::{{closure}}
             at src/librustdoc/lib.rs:523
 174: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./src/libstd/panic.rs:318
 175: std::panicking::try::do_call
             at ./src/libstd/panicking.rs:297
 176: std::panicking::try
             at ./src/libstd/panicking.rs:274
 177: std::panic::catch_unwind
             at ./src/libstd/panic.rs:394
 178: rustc_driver::catch_fatal_errors
             at ./src/librustc_driver/lib.rs:1124
 179: rustdoc::rust_input
             at src/librustdoc/lib.rs:520
 180: rustdoc::main_options
             at src/librustdoc/lib.rs:484
 181: rustdoc::main_args::{{closure}}
             at src/librustdoc/lib.rs:450
 182: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:149
 183: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 184: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:145
 185: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 186: rustc_ast::attr::with_globals::{{closure}}
             at ./src/librustc_ast/attr/mod.rs:44
 187: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 188: rustc_ast::attr::with_globals
             at ./src/librustc_ast/attr/mod.rs:44
 189: rustc_interface::util::spawn_thread_pool::{{closure}}
             at ./src/librustc_interface/util.rs:144
 190: rustc_interface::util::scoped_thread::{{closure}}
             at ./src/librustc_interface/util.rs:119
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: Could not document `rustc_attr`.
