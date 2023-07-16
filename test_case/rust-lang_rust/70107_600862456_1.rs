
  10: rustc_mir::borrow_check::type_check::TypeChecker::check_user_type_annotations
             at src/librustc_mir/borrow_check/type_check/mod.rs:1038
  11: rustc_mir::borrow_check::type_check::TypeChecker::new
             at src/librustc_mir/borrow_check/type_check/mod.rs:988
  12: rustc_mir::borrow_check::type_check::type_check_internal
             at src/librustc_mir/borrow_check/type_check/mod.rs:205
  13: rustc_mir::borrow_check::type_check::type_check
             at src/librustc_mir/borrow_check/type_check/mod.rs:171
  14: rustc_mir::borrow_check::nll::compute_regions
             at src/librustc_mir/borrow_check/nll.rs:178
  15: rustc_mir::borrow_check::do_mir_borrowck
             at src/librustc_mir/borrow_check/mod.rs:209
  16: rustc_mir::borrow_check::mir_borrowck::{{closure}}
             at src/librustc_mir/borrow_check/mod.rs:102
  17: rustc_infer::infer::InferCtxtBuilder::enter::{{closure}}
             at ./src/librustc_infer/infer/mod.rs:575
  18: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}::{{closure}}
             at ./src/librustc/ty/context.rs:1558
  19: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1726
  20: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1710
  21: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1726
  22: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}
             at ./src/librustc/ty/context.rs:1558
  23: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1814
  24: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1798
  25: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1787
  26: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1798
  27: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1811
  28: rustc::ty::context::GlobalCtxt::enter_local
             at ./src/librustc/ty/context.rs:1550
  29: rustc_infer::infer::InferCtxtBuilder::enter
             at ./src/librustc_infer/infer/mod.rs:574
  30: rustc_mir::borrow_check::mir_borrowck
             at src/librustc_mir/borrow_check/mod.rs:99
  31: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:1008
  32: rustc::ty::query::__query_compute::mir_borrowck
             at ./src/librustc/ty/query/plumbing.rs:960
  33: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
             at ./src/librustc/ty/query/plumbing.rs:1000
  34: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:319
  35: rustc::dep_graph::graph::DepGraph::with_task
             at ./src/librustc/dep_graph/graph.rs:209
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:670
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:324
  38: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1726
  39: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1710
  40: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1726
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:324
  42: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1814
  43: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1798
  44: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1787
  45: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1798
  46: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1811
  47: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:313
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:660
  49: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:248
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:659
  51: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_execute_query
             at ./src/librustc/ty/query/plumbing.rs:493
  52: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:468
  53: <rustc::ty::query::caches::DefaultCache as rustc::ty::query::caches::QueryCache<K,V>>::lookup
             at ./src/librustc/ty/query/caches.rs:86
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_get_cached
             at ./src/librustc/ty/query/plumbing.rs:436
  55: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:462
  56: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
             at ./src/librustc/ty/query/plumbing.rs:714
  57: rustc::ty::query::TyCtxtEnsure::mir_borrowck
             at ./src/librustc/ty/query/plumbing.rs:1036
  58: rustc_interface::passes::analysis::{{closure}}::{{closure}}
             at src/librustc_interface/passes.rs:824
  59: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::{{closure}}
             at ./src/librustc/ty/mod.rs:2853
  60: core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
             at ./src/libcore/iter/traits/iterator.rs:655
  61: core::iter::traits::iterator::Iterator::fold::ok::{{closure}}
             at ./src/libcore/iter/traits/iterator.rs:2001
  62: core::iter::traits::iterator::Iterator::try_fold
             at ./src/libcore/iter/traits/iterator.rs:1877
  63: core::iter::traits::iterator::Iterator::fold
             at ./src/libcore/iter/traits/iterator.rs:2004
  64: core::iter::traits::iterator::Iterator::for_each
             at ./src/libcore/iter/traits/iterator.rs:658
  65: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
             at ./src/librustc/ty/mod.rs:2852
  66: rustc_interface::passes::analysis::{{closure}}
             at src/librustc_interface/passes.rs:824
  67: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./src/librustc_data_structures/profiling.rs:569
  68: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./src/librustc_session/utils.rs:9
  69: rustc_interface::passes::analysis
             at src/librustc_interface/passes.rs:823
  70: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:1008
  71: rustc::ty::query::__query_compute::analysis
             at ./src/librustc/ty/query/plumbing.rs:960
  72: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
             at ./src/librustc/ty/query/plumbing.rs:1000
  73: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:319
  74: rustc::dep_graph::graph::DepGraph::with_eval_always_task
             at ./src/librustc/dep_graph/graph.rs:366
  75: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:662
  76: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:324
  77: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1726
  78: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1710
  79: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1726
  80: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:324
  81: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1814
  82: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1798
  83: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1787
  84: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1798
  85: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1811
  86: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:313
  87: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:660
  88: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:248
  89: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:659
  90: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_execute_query
             at ./src/librustc/ty/query/plumbing.rs:493
  91: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:468
  92: <rustc::ty::query::caches::DefaultCache as rustc::ty::query::caches::QueryCache<K,V>>::lookup
             at ./src/librustc/ty/query/caches.rs:86
  93: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_get_cached
             at ./src/librustc/ty/query/plumbing.rs:436
  94: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:462
  95: rustc::ty::query::TyCtxtAt::analysis
             at ./src/librustc/ty/query/plumbing.rs:1114
  96: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::analysis
             at ./src/librustc/ty/query/plumbing.rs:1077
  97: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:392
  98: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./src/librustc_interface/passes.rs:696
  99: rustc::ty::context::tls::enter_global::{{closure}}
             at ./src/librustc/ty/context.rs:1749
 100: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1726
 101: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1710
 102: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1726
 103: rustc::ty::context::tls::enter_global
             at ./src/librustc/ty/context.rs:1749
 104: rustc_interface::passes::QueryContext::enter
             at ./src/librustc_interface/passes.rs:696
 105: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:392
 106: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./src/librustc_interface/queries.rs:339
 107: rustc_driver::run_compiler::{{closure}}
             at src/librustc_driver/lib.rs:292
 108: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at ./src/librustc_interface/interface.rs:199
 109: rustc_interface::interface::run_compiler::{{closure}}
             at ./src/librustc_interface/interface.rs:213
 110: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:155
 111: scoped_tls::ScopedKey<T>::set
             at /home/programming/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 112: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:151
 113: scoped_tls::ScopedKey<T>::set
             at /home/programming/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 114: rustc_ast::attr::with_globals::{{closure}}
             at ./src/librustc_ast/attr/mod.rs:44
 115: scoped_tls::ScopedKey<T>::set
