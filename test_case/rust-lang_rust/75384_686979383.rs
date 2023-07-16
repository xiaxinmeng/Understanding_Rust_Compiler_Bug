
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
    Finished release [optimized + debuginfo] target(s) in 0.25s
    Finished release [optimized + debuginfo] target(s) in 0.62s
   Compiling core v0.0.0 (/home/kadmin/projects/rust/library/core)
error: internal compiler error: compiler/rustc_middle/src/hir/map/collector.rs:251:17: inconsistent DepNode at `"library/core/src/num/f32.rs:636:36: 636:39"` for `::f32[0]::{{impl}}[0]::to_int_unchecked[0]::Int[0]`: current_dep_node_owner=::f32[0]::{{impl}}[0]::to_int_unchecked[0] (DefId(0:136 ~ core[8057]::f32[0]::{{impl}}[0]::to_int_unchecked[0])), hir_id.owner=::f32[0]::{{impl}}[0]::to_int_unchecked[0]::Int[0] (DefId(0:137 ~ core[8057]::f32[0]::{{impl}}[0]::to_int_unchecked[0]::Int[0]))
   --> library/core/src/num/f32.rs:636:36
    |
636 |     pub unsafe fn to_int_unchecked<Int>(self) -> Int
    |                                    ^^^

thread 'rustc' panicked at 'Box<Any>', /home/kadmin/projects/rust/compiler/rustc_errors/src/lib.rs:891:9
stack backtrace:
   0: std::panicking::begin_panic
             at ./library/std/src/panicking.rs:505
   1: rustc_errors::HandlerInner::span_bug
             at ./compiler/rustc_errors/src/lib.rs:891
   2: rustc_errors::Handler::span_bug
             at ./compiler/rustc_errors/src/lib.rs:633
   3: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
             at ./compiler/rustc_middle/src/util/bug.rs:32
   4: rustc_middle::ty::context::tls::with_opt::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1790
   5: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1742
   6: rustc_middle::ty::context::tls::with_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1790
   7: rustc_middle::util::bug::opt_span_bug_fmt
             at ./compiler/rustc_middle/src/util/bug.rs:29
   8: rustc_middle::util::bug::span_bug_fmt
             at ./compiler/rustc_middle/src/util/bug.rs:21
   9: rustc_middle::hir::map::collector::NodeCollector::insert_with_hash
             at ./compiler/rustc_middle/src/hir/map/collector.rs:251
  10: rustc_middle::hir::map::collector::NodeCollector::insert
             at ./compiler/rustc_middle/src/hir/map/collector.rs:236
  11: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_generic_param
             at ./compiler/rustc_middle/src/hir/map/collector.rs:364
  12: rustc_hir::intravisit::walk_generics
             at ./compiler/rustc_hir/src/intravisit.rs:871
  13: rustc_hir::intravisit::Visitor::visit_generics
             at ./compiler/rustc_hir/src/intravisit.rs:361
  14: rustc_hir::intravisit::walk_impl_item
             at ./compiler/rustc_hir/src/intravisit.rs:1000
  15: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_impl_item::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/hir/map/collector.rs:391
  16: rustc_middle::hir::map::collector::NodeCollector::with_parent
             at ./compiler/rustc_middle/src/hir/map/collector.rs:271
  17: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_impl_item::{{closure}}
             at ./compiler/rustc_middle/src/hir/map/collector.rs:390
  18: rustc_middle::hir::map::collector::NodeCollector::with_dep_node_owner
             at ./compiler/rustc_middle/src/hir/map/collector.rs:291
  19: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_impl_item
             at ./compiler/rustc_middle/src/hir/map/collector.rs:387
  20: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_impl_item
             at ./compiler/rustc_middle/src/hir/map/collector.rs:317
  21: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_impl_item_ref
             at ./compiler/rustc_middle/src/hir/map/collector.rs:545
  22: rustc_hir::intravisit::walk_item
             at ./compiler/rustc_hir/src/intravisit.rs:609
  23: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/hir/map/collector.rs:347
  24: rustc_middle::hir::map::collector::NodeCollector::with_parent
             at ./compiler/rustc_middle/src/hir/map/collector.rs:271
  25: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}
             at ./compiler/rustc_middle/src/hir/map/collector.rs:340
  26: rustc_middle::hir::map::collector::NodeCollector::with_dep_node_owner
             at ./compiler/rustc_middle/src/hir/map/collector.rs:291
  27: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item
             at ./compiler/rustc_middle/src/hir/map/collector.rs:338
  28: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item
             at ./compiler/rustc_middle/src/hir/map/collector.rs:309
  29: rustc_hir::intravisit::walk_mod
             at ./compiler/rustc_hir/src/intravisit.rs:480
  30: rustc_hir::intravisit::Visitor::visit_mod
             at ./compiler/rustc_hir/src/intravisit.rs:328
  31: rustc_hir::intravisit::walk_item
             at ./compiler/rustc_hir/src/intravisit.rs:570
  32: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/hir/map/collector.rs:347
  33: rustc_middle::hir::map::collector::NodeCollector::with_parent
             at ./compiler/rustc_middle/src/hir/map/collector.rs:271
  34: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}
             at ./compiler/rustc_middle/src/hir/map/collector.rs:340
  35: rustc_middle::hir::map::collector::NodeCollector::with_dep_node_owner
             at ./compiler/rustc_middle/src/hir/map/collector.rs:291
  36: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item
             at ./compiler/rustc_middle/src/hir/map/collector.rs:338
  37: <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item
             at ./compiler/rustc_middle/src/hir/map/collector.rs:309
  38: rustc_hir::intravisit::walk_mod
             at ./compiler/rustc_hir/src/intravisit.rs:480
  39: rustc_hir::intravisit::Visitor::visit_mod
             at ./compiler/rustc_hir/src/intravisit.rs:328
  40: rustc_hir::intravisit::walk_crate
             at ./compiler/rustc_hir/src/intravisit.rs:466
  41: rustc_middle::hir::map::index_hir
             at ./compiler/rustc_middle/src/hir/map/mod.rs:984
  42: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::{{closure}}
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:249
  43: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:71
  44: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1725
  45: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1709
  46: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1725
  47: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:71
  48: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1753
  49: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1742
  50: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1753
  51: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:68
  52: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:249
  53: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:336
  54: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:591
  55: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72
  56: stacker::maybe_grow
             at /home/kadmin/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.11/src/lib.rs:52
  57: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16
  58: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72
  59: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1725
  60: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1709
  61: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1725
  62: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:71
  63: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1769
  64: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1753
  65: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1742
  66: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1753
  67: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1766
  68: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:60
  69: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:589
  70: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:296
  71: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:588
  72: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:465
  73: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:639
  74: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:110
  75: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:369
  76: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:631
  77: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:731
  78: rustc_middle::ty::query::TyCtxtAt::index_hir
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:491
  79: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::index_hir
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:452
  80: rustc_middle::hir::provide::{{closure}}
             at ./compiler/rustc_middle/src/hir/mod.rs:78
  81: core::ops::function::FnOnce::call_once
             at ./library/core/src/ops/function.rs:227
  82: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::{{closure}}
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:249
  83: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:71
  84: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1725
  85: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1709
  86: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1725
  87: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:71
  88: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1753
  89: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1742
  90: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1753
  91: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:68
  92: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:249
  93: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:336
  94: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:591
  95: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72
  96: stacker::maybe_grow
             at /home/kadmin/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.11/src/lib.rs:52
  97: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16
  98: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72
  99: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1725
 100: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1709
 101: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1725
 102: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:71
 103: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1769
 104: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1753
 105: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1742
 106: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1753
 107: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1766
 108: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:60
 109: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:589
 110: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:296
 111: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:588
 112: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:465
 113: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:639
 114: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:110
 115: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:369
 116: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:631
 117: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:731
 118: rustc_middle::ty::query::TyCtxtAt::hir_owner
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:491
 119: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::hir_owner
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:452
 120: rustc_middle::hir::map::Map::find_entry
             at ./compiler/rustc_middle/src/hir/map/mod.rs:270
 121: rustc_middle::hir::map::Map::find
             at ./compiler/rustc_middle/src/hir/map/mod.rs:506
 122: rustc_middle::hir::map::Map::expect_item
             at ./compiler/rustc_middle/src/hir/map/mod.rs:731
 123: rustc_middle::hir::map::Map::visit_item_likes_in_module
             at ./compiler/rustc_middle/src/hir/map/mod.rs:463
 124: rustc_passes::hir_id_validator::check_crate::{{closure}}
             at ./compiler/rustc_passes/src/hir_id_validator.rs:19
 125: core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
             at ./library/core/src/iter/traits/iterator.rs:642
 126: core::iter::traits::iterator::Iterator::fold
             at ./library/core/src/iter/traits/iterator.rs:1998
 127: core::iter::traits::iterator::Iterator::for_each
             at ./library/core/src/iter/traits/iterator.rs:645
 128: rustc_passes::hir_id_validator::check_crate
             at ./compiler/rustc_passes/src/hir_id_validator.rs:17
 129: rustc_interface::passes::analysis
             at ./compiler/rustc_interface/src/passes.rs:794
 130: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:381
 131: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::{{closure}}
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:249
 132: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:71
 133: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1725
 134: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1709
 135: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1725
 136: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:71
 137: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1753
 138: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1742
 139: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1753
 140: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:68
 141: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:249
 142: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
 143: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72
 144: stacker::maybe_grow
             at /home/kadmin/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.11/src/lib.rs:52
 145: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16
 146: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72
 147: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1725
 148: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1709
 149: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1725
 150: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:71
 151: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1769
 152: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1753
 153: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1742
 154: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1753
 155: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1766
 156: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:60
 157: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:589
 158: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:296
 159: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:588
 160: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:465
 161: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:639
 162: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:110
 163: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:369
 164: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:631
 165: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:731
 166: rustc_middle::ty::query::TyCtxtAt::analysis
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:491
 167: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::analysis
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:452
 168: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:383
 169: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:722
 170: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1725
 171: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1709
 172: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1725
 173: rustc_interface::passes::QueryContext::enter
             at ./compiler/rustc_interface/src/passes.rs:722
 174: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:383
 175: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:385
 176: rustc_driver::run_compiler::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:284
 177: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:191
 178: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:750
 179: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:185
 180: rustc_interface::interface::run_compiler::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:207
 181: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:149
 182: scoped_tls::ScopedKey<T>::set
             at /home/kadmin/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 183: rustc_span::with_session_globals
             at ./compiler/rustc_span/src/lib.rs:91
 184: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:145
 185: rustc_interface::util::scoped_thread::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:120
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C debug-assertions=on -C overflow-checks=off -C incremental -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [index_hir] index HIR
#1 [hir_owner] HIR owner of `{{misc}}#0`
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

error: could not compile `core`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/home/kadmin/projects/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "1" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/kadmin/projects/rust/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
