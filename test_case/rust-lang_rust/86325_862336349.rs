
------------------------------------------
DEBUG rustc_mir::const_eval::eval_queries promoted=None
DEBUG rustc_mir::const_eval::eval_queries eval_body_using_ecx: GlobalId { instance: Instance { def: Item(WithOptConstParam { did: DefId(0:5 ~ issue_49595[317d]::lit::A), const_param_did: None }), substs: [] }, promoted: None }, ParamEnv { caller_bounds: [], reveal: UserFacing }
DEBUG rustc_mir::const_eval::eval_queries eval_body_using_ecx done: MemPlace { ptr: alloc2, align: Align { pow2: 3 }, meta: None }
DEBUG rustc_mir::const_eval::eval_queries mk_eval_cx: ParamEnv { caller_bounds: [], reveal: UserFacing }
DEBUG rustc_mir::const_eval::eval_queries promoted=Some(promoted[1])
thread 'rustc' panicked at 'attempted to read from stolen value', /media/ellen-nyan/Nyoomies/rust/compiler/rustc_data_structures/src/steal.rs:37:21
stack backtrace:
   0: std::panicking::begin_panic
             at ./library/std/src/panicking.rs:541:12
   1: rustc_data_structures::steal::Steal<T>::borrow::{{closure}}
             at ./compiler/rustc_data_structures/src/steal.rs:37:21
   2: core::cell::Ref<T>::map
             at ./library/core/src/cell.rs:1336:22
   3: rustc_data_structures::steal::Steal<T>::borrow
             at ./compiler/rustc_data_structures/src/steal.rs:36:9
   4: rustc_mir::borrow_check::mir_borrowck::{{closure}}
             at ./compiler/rustc_mir/src/borrow_check/mod.rs:109:38
   5: rustc_infer::infer::InferCtxtBuilder::enter
             at ./compiler/rustc_infer/src/infer/mod.rs:580:9
   6: rustc_mir::borrow_check::mir_borrowck
             at ./compiler/rustc_mir/src/borrow_check/mod.rs:108:27
   7: rustc_mir::borrow_check::provide::{{closure}}
             at ./compiler/rustc_mir/src/borrow_check/mod.rs:91:17
   8: core::ops::function::FnOnce::call_once
             at ./library/core/src/ops/function.rs:227:5
   9: rustc_query_system::query::config::QueryVtable<CTX,K,V>::compute
             at ./compiler/rustc_query_system/src/query/config.rs:44:9
  10: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:568:67
  11: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:76:46
  12: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
  13: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
  14: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
  15: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:76:13
  16: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1738:40
  17: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1727:22
  18: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1738:9
  19: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:73:9
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::with_ignore
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:175:9
  21: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
             at ./compiler/rustc_query_system/src/query/plumbing.rs:568:22
  22: rustc_query_system::query::plumbing::try_execute_query::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:497:21
  23: core::option::Option<T>::map
             at ./library/core/src/option.rs:489:29
  24: rustc_query_system::query::plumbing::try_execute_query::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:495:13
  25: stacker::maybe_grow
             at /home/ellen-nyan/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  26: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  27: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:133:17
  28: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
  29: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
  30: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
  31: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:132:13
  32: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1754:13
  33: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1738:40
  34: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1727:22
  35: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1738:9
  36: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1751:9
  37: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query
             at ./compiler/rustc_query_impl/src/plumbing.rs:121:9
  38: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:493:22
  39: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:699:5
  40: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:832:9
  41: rustc_middle::ty::query::TyCtxtAt::mir_borrowck
             at ./compiler/rustc_middle/src/ty/query/mod.rs:206:17
  42: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::mir_borrowck
             at ./compiler/rustc_middle/src/ty/query/mod.rs:187:17
  43: rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider
             at ./compiler/rustc_mir/src/const_eval/eval_queries.rs:301:13
  44: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::eval_to_allocation_raw>::compute
             at ./compiler/rustc_query_impl/src/plumbing.rs:363:17
  45: rustc_query_system::query::config::QueryVtable<CTX,K,V>::compute
             at ./compiler/rustc_query_system/src/query/config.rs:44:9
  46: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:568:67
  47: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:76:46
  48: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
  49: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
  50: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
  51: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:76:13
  52: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1738:40
  53: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1727:22
  54: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1738:9
  55: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:73:9
  56: rustc_query_system::dep_graph::graph::DepGraph<K>::with_ignore
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:175:9
  57: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
             at ./compiler/rustc_query_system/src/query/plumbing.rs:568:22
  58: rustc_query_system::query::plumbing::try_execute_query::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:497:21
  59: core::option::Option<T>::map
             at ./library/core/src/option.rs:489:29
  60: rustc_query_system::query::plumbing::try_execute_query::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:495:13
  61: stacker::maybe_grow
             at /home/ellen-nyan/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  62: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  63: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:133:17
  64: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
  65: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
  66: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
  67: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:132:13
  68: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1754:13
  69: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1738:40
  70: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1727:22
  71: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1738:9
  72: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1751:9
  73: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query
             at ./compiler/rustc_query_impl/src/plumbing.rs:121:9
  74: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:493:22
  75: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:699:5
  76: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:832:9
  77: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
             at ./compiler/rustc_query_impl/src/plumbing.rs:552:17
  78: rustc_middle::ty::query::TyCtxtAt::eval_to_allocation_raw
             at ./compiler/rustc_middle/src/ty/query/mod.rs:206:17
  79: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::eval_to_allocation_raw
             at ./compiler/rustc_middle/src/ty/query/mod.rs:187:17
  80: rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider
             at ./compiler/rustc_mir/src/const_eval/eval_queries.rs:259:15
  81: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::eval_to_allocation_raw>::compute
             at ./compiler/rustc_query_impl/src/plumbing.rs:363:17
  82: rustc_query_system::query::config::QueryVtable<CTX,K,V>::compute
             at ./compiler/rustc_query_system/src/query/config.rs:44:9
  83: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:568:67
  84: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:76:46
  85: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
  86: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
  87: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
  88: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:76:13
  89: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1738:40
  90: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1727:22
  91: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1738:9
  92: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:73:9
  93: rustc_query_system::dep_graph::graph::DepGraph<K>::with_ignore
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:175:9
  94: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
             at ./compiler/rustc_query_system/src/query/plumbing.rs:568:22
  95: rustc_query_system::query::plumbing::try_execute_query::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:497:21
  96: core::option::Option<T>::map
             at ./library/core/src/option.rs:489:29
  97: rustc_query_system::query::plumbing::try_execute_query::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:495:13
  98: stacker::maybe_grow
             at /home/ellen-nyan/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  99: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
 100: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:133:17
 101: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
 102: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
 103: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
 104: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:132:13
 105: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1754:13
 106: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1738:40
 107: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1727:22
 108: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1738:9
 109: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1751:9
 110: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query
             at ./compiler/rustc_query_impl/src/plumbing.rs:121:9
 111: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:493:22
 112: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:699:5
 113: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:832:9
 114: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
             at ./compiler/rustc_query_impl/src/plumbing.rs:552:17
 115: rustc_middle::ty::query::TyCtxtAt::eval_to_allocation_raw
             at ./compiler/rustc_middle/src/ty/query/mod.rs:206:17
 116: rustc_mir::interpret::eval_context::InterpCx<M>::eval_to_allocation
             at ./compiler/rustc_mir/src/interpret/eval_context.rs:919:19
 117: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::const_to_op
             at ./compiler/rustc_mir/src/interpret/operand.rs:543:20
 118: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::mir_const_to_op
             at ./compiler/rustc_mir/src/interpret/operand.rs:558:42
 119: rustc_mir::transform::const_prop::ConstPropagator::eval_constant
             at ./compiler/rustc_mir/src/transform/const_prop.rs:475:15
 120: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_constant
             at ./compiler/rustc_mir/src/transform/const_prop.rs:1089:9
 121: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_operand
             at ./compiler/rustc_mir/src/transform/const_prop.rs:1077:9
 122: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_statement
             at ./compiler/rustc_mir/src/transform/const_prop.rs:1176:9
 123: rustc_middle::mir::visit::MutVisitor::super_basic_block_data
             at ./compiler/rustc_middle/src/mir/visit.rs:312:21
 124: rustc_middle::mir::visit::MutVisitor::visit_basic_block_data
             at ./compiler/rustc_middle/src/mir/visit.rs:84:17
 125: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_body
             at ./compiler/rustc_mir/src/transform/const_prop.rs:1072:13
 126: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
             at ./compiler/rustc_mir/src/transform/const_prop.rs:152:9
 127: rustc_mir::transform::run_passes::{{closure}}
             at ./compiler/rustc_mir/src/transform/mod.rs:193:9
 128: rustc_mir::transform::run_passes
             at ./compiler/rustc_mir/src/transform/mod.rs:209:13
 129: rustc_mir::transform::run_optimization_passes
             at ./compiler/rustc_mir/src/transform/mod.rs:550:5
 130: rustc_mir::transform::inner_optimized_mir
             at ./compiler/rustc_mir/src/transform/mod.rs:587:5
 131: rustc_mir::transform::optimized_mir
             at ./compiler/rustc_mir/src/transform/mod.rs:565:21
 132: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::{{closure}}
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:243:62
 133: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:76:46
 134: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
 135: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
 136: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
 137: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:76:13
 138: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1738:40
 139: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1727:22
 140: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1738:9
 141: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:73:9
 142: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:243:26
 143: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
 144: stacker::maybe_grow
             at /home/ellen-nyan/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
 145: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
 146: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:133:17
 147: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
 148: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
 149: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
 150: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:132:13
 151: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1754:13
 152: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1738:40
 153: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1727:22
 154: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1738:9
 155: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1751:9
 156: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query
             at ./compiler/rustc_query_impl/src/plumbing.rs:121:9
 157: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:652:9
 158: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:337:18
 159: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:651:51
 160: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:514:36
 161: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:699:5
 162: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:832:9
 163: rustc_middle::ty::query::TyCtxtAt::optimized_mir
             at ./compiler/rustc_middle/src/ty/query/mod.rs:206:17
 164: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::optimized_mir
             at ./compiler/rustc_middle/src/ty/query/mod.rs:187:17
 165: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir
             at ./compiler/rustc_middle/src/ty/mod.rs:1794:21
 166: rustc_mir::monomorphize::collector::collect_neighbours
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:1376:16
 167: rustc_mir::monomorphize::collector::collect_items_rec::{{closure}}
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:417:17
 168: stacker::maybe_grow
             at /home/ellen-nyan/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
 169: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
 170: rustc_mir::monomorphize::collector::collect_items_rec
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:416:13
 171: rustc_mir::monomorphize::collector::collect_items_rec
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:458:9
 172: rustc_mir::monomorphize::collector::collect_items_rec
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:458:9
 173: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}::{{closure}}
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:305:17
 174: core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
             at ./library/core/src/iter/traits/iterator.rs:733:29
 175: core::iter::traits::iterator::Iterator::fold
             at ./library/core/src/iter/traits/iterator.rs:2112:21
 176: core::iter::traits::iterator::Iterator::for_each
             at ./library/core/src/iter/traits/iterator.rs:736:9
 177: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:303:13
 178: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:573:9
 179: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:16:9
 180: rustc_mir::monomorphize::collector::collect_crate_mono_items
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:302:9
 181: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
             at ./compiler/rustc_mir/src/monomorphize/partitioning/mod.rs:344:33
 182: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::collect_and_partition_mono_items>::compute
             at ./compiler/rustc_query_impl/src/plumbing.rs:363:17
 183: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::{{closure}}
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:243:62
 184: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:76:46
 185: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
 186: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
 187: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
 188: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:76:13
 189: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1738:40
 190: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1727:22
 191: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1738:9
 192: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:73:9
 193: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:243:26
 194: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
 195: stacker::maybe_grow
             at /home/ellen-nyan/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
 196: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
 197: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:133:17
 198: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
 199: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
 200: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
 201: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:132:13
 202: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1754:13
 203: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1738:40
 204: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1727:22
 205: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1738:9
 206: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1751:9
 207: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query
             at ./compiler/rustc_query_impl/src/plumbing.rs:121:9
 208: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:652:9
 209: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:337:18
 210: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:651:51
 211: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:514:36
 212: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:699:5
 213: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:832:9
 214: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
             at ./compiler/rustc_query_impl/src/plumbing.rs:552:17
 215: rustc_middle::ty::query::TyCtxtAt::collect_and_partition_mono_items
             at ./compiler/rustc_middle/src/ty/query/mod.rs:206:17
 216: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::collect_and_partition_mono_items
             at ./compiler/rustc_middle/src/ty/query/mod.rs:187:17
 217: rustc_codegen_ssa::base::codegen_crate
             at ./compiler/rustc_codegen_ssa/src/base.rs:490:25
 218: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             at ./compiler/rustc_codegen_llvm/src/lib.rs:257:18
 219: rustc_interface::passes::start_codegen::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:1060:9
 220: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:573:9
 221: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:16:9
 222: rustc_interface::passes::start_codegen
             at ./compiler/rustc_interface/src/passes.rs:1059:19
 223: rustc_interface::queries::Queries::ongoing_codegen::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/queries.rs:295:20
 224: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:799:42
 225: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1710:50
 226: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1694:9
 227: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1710:9
 228: rustc_interface::passes::QueryContext::enter
             at ./compiler/rustc_interface/src/passes.rs:799:9
 229: rustc_interface::queries::Queries::ongoing_codegen::{{closure}}
             at ./compiler/rustc_interface/src/queries.rs:286:13
 230: rustc_interface::queries::Query<T>::compute
             at ./compiler/rustc_interface/src/queries.rs:40:28
 231: rustc_interface::queries::Queries::ongoing_codegen
             at ./compiler/rustc_interface/src/queries.rs:284:9
 232: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:414:13
 233: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:431:19
 234: rustc_driver::run_compiler::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:313:22
 235: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:208:13
 236: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:873:5
 237: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:202:5
 238: rustc_interface::interface::run_compiler::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:224:12
 239: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:158:13
 240: scoped_tls::ScopedKey<T>::set
             at /home/ellen-nyan/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 241: rustc_span::with_session_globals
             at ./compiler/rustc_span/src/lib.rs:104:5
 242: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:156:9
 243: rustc_interface::util::scoped_thread::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:131:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
#0 [mir_borrowck] borrow-checking `lit_test::lit_test`
#1 [eval_to_allocation_raw] const-evaluating + checking `lit_test::lit_test::promoted[1]`
#2 [eval_to_allocation_raw] const-evaluating + checking `lit_test::lit_test::promoted[1]`
#3 [optimized_mir] optimizing MIR for `lit_test::lit_test`
#4 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack

------------------------------------------
