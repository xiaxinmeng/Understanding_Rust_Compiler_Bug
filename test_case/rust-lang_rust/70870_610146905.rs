
error: internal compiler error: src/librustc_privacy/lib.rs:226: unexpected type: FreshTy(0)

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/mark/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /home/mark/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1439
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at ./src/liballoc/boxed.rs:1022
  11: rustc_driver::report_ice
             at src/librustc_driver/lib.rs:1174
  12: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:515
  13: std::panicking::begin_panic
             at ./src/libstd/panicking.rs:438
  14: rustc_errors::HandlerInner::bug
             at src/librustc_errors/lib.rs:880
  15: rustc_errors::Handler::bug
             at src/librustc_errors/lib.rs:651
  16: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
             at src/librustc_middle/util/bug.rs:36
  17: rustc_middle::ty::context::tls::with_opt::{{closure}}
             at src/librustc_middle/ty/context.rs:1804
  18: rustc_middle::ty::context::tls::with_context_opt
             at src/librustc_middle/ty/context.rs:1756
  19: rustc_middle::ty::context::tls::with_opt
             at src/librustc_middle/ty/context.rs:1804
  20: rustc_middle::util::bug::opt_span_bug_fmt
             at src/librustc_middle/util/bug.rs:32
  21: rustc_middle::util::bug::bug_fmt
             at src/librustc_middle/util/bug.rs:12
  22: <rustc_privacy::DefIdVisitorSkeleton<V> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
             at src/librustc_privacy/lib.rs:226
  23: rustc_middle::ty::subst::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::List<rustc_middle::ty::subst::GenericArg>>::super_visit_with::{{closure}}
             at ./src/librustc_middle/ty/subst.rs:414
  24: core::iter::traits::iterator::Iterator::any::check::{{closure}}
             at ./src/libcore/iter/traits/iterator.rs:2145
  25: core::iter::traits::iterator::Iterator::try_fold
             at ./src/libcore/iter/traits/iterator.rs:1878
  26: core::iter::traits::iterator::Iterator::any
             at ./src/libcore/iter/traits/iterator.rs:2149
  27: rustc_middle::ty::subst::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::List<rustc_middle::ty::subst::GenericArg>>::super_visit_with
             at ./src/librustc_middle/ty/subst.rs:414
  28: rustc_middle::ty::fold::TypeFoldable::visit_with
             at ./src/librustc_middle/ty/fold.rs:54
  29: rustc_privacy::DefIdVisitorSkeleton<V>::visit_trait
             at src/librustc_privacy/lib.rs:87
  30: rustc_privacy::DefIdVisitor::visit_trait
             at src/librustc_privacy/lib.rs:64
  31: <rustc_privacy::TypePrivacyVisitor as rustc_hir::intravisit::Visitor>::visit_trait_ref
             at src/librustc_privacy/lib.rs:1250
  32: rustc_hir::intravisit::walk_poly_trait_ref
             at ./src/librustc_hir/intravisit.rs:528
  33: rustc_hir::intravisit::Visitor::visit_poly_trait_ref
             at ./src/librustc_hir/intravisit.rs:393
  34: rustc_hir::intravisit::walk_param_bound
             at ./src/librustc_hir/intravisit.rs:838
  35: rustc_hir::intravisit::Visitor::visit_param_bound
             at ./src/librustc_hir/intravisit.rs:390
  36: rustc_hir::intravisit::walk_where_predicate
             at ./<::rustc_ast::visit::walk_list macros>:2
  37: rustc_hir::intravisit::Visitor::visit_where_predicate
             at ./src/librustc_hir/intravisit.rs:363
  38: rustc_hir::intravisit::walk_generics
             at ./<::rustc_ast::visit::walk_list macros>:2
  39: rustc_hir::intravisit::Visitor::visit_generics
             at ./src/librustc_hir/intravisit.rs:360
  40: rustc_hir::intravisit::walk_impl_item
             at ./src/librustc_hir/intravisit.rs:989
  41: <rustc_privacy::TypePrivacyVisitor as rustc_hir::intravisit::Visitor>::visit_impl_item
             at src/librustc_privacy/lib.rs:1388
  42: rustc_hir::intravisit::Visitor::visit_nested_impl_item
             at ./<::rustc_ast::visit::walk_list macros>:2
  43: rustc_hir::intravisit::walk_impl_item_ref
             at ./src/librustc_hir/intravisit.rs:1019
  44: rustc_hir::intravisit::Visitor::visit_impl_item_ref
             at ./src/librustc_hir/intravisit.rs:384
  45: rustc_hir::intravisit::walk_item
             at ./<::rustc_ast::visit::walk_list macros>:2
  46: <rustc_privacy::TypePrivacyVisitor as rustc_hir::intravisit::Visitor>::visit_item
             at src/librustc_privacy/lib.rs:1372
  47: rustc_hir::intravisit::Visitor::visit_nested_item
             at ./<::rustc_ast::visit::walk_list macros>:2
  48: rustc_hir::intravisit::walk_mod
             at ./src/librustc_hir/intravisit.rs:479
  49: rustc_privacy::check_mod_privacy
             at src/librustc_privacy/lib.rs:2078
  50: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_mod_privacy>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:357
  51: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::{{closure}}
             at ./src/librustc_query_system/dep_graph/graph.rs:249
  52: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}::{{closure}}
             at ./src/librustc_middle/dep_graph/mod.rs:71
  53: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
  54: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
  55: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
  56: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}
             at ./src/librustc_middle/dep_graph/mod.rs:71
  57: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1767
  58: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1756
  59: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1767
  60: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
             at ./src/librustc_middle/dep_graph/mod.rs:68
  61: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:249
  62: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./src/librustc_query_system/dep_graph/graph.rs:200
  63: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:592
  64: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
  65: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
  66: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
  67: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
  68: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
  69: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1783
  70: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1767
  71: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1756
  72: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1767
  73: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1780
  74: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
  75: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:582
  76: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:292
  77: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:581
  78: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:461
  79: rustc_query_system::query::plumbing::get_query::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:626
  80: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:91
  81: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:367
  82: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:618
  83: rustc_query_system::query::plumbing::ensure_query
             at ./src/librustc_query_system/query/plumbing.rs:660
  84: rustc_middle::ty::query::TyCtxtEnsure::check_mod_privacy
             at ./src/librustc_middle/ty/query/plumbing.rs:384
  85: rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_interface/passes.rs:890
  86: core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
             at ./src/libcore/iter/traits/iterator.rs:655
  87: core::iter::traits::iterator::Iterator::fold::ok::{{closure}}
             at ./src/libcore/iter/traits/iterator.rs:2002
  88: core::iter::traits::iterator::Iterator::try_fold
             at ./src/libcore/iter/traits/iterator.rs:1878
  89: core::iter::traits::iterator::Iterator::fold
             at ./src/libcore/iter/traits/iterator.rs:2005
  90: core::iter::traits::iterator::Iterator::for_each
             at ./src/libcore/iter/traits/iterator.rs:658
  91: rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_interface/passes.rs:889
  92: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./src/librustc_data_structures/profiling.rs:568
  93: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./src/librustc_session/utils.rs:9
  94: rustc_interface::passes::analysis::{{closure}}::{{closure}}
             at src/librustc_interface/passes.rs:888
  95: core::ops::function::FnOnce::call_once
             at ./src/libcore/ops/function.rs:232
  96: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./src/libstd/panic.rs:318
  97: std::panicking::try::do_call
             at ./src/libstd/panicking.rs:331
  98: std::panicking::try::do_try
             at src/libstd/panicking.rs:298
  99: std::panicking::try
             at ./src/libstd/panicking.rs:274
 100: std::panic::catch_unwind
             at ./src/libstd/panic.rs:394
 101: rustc_interface::passes::analysis::{{closure}}
             at src/librustc_interface/passes.rs:862
 102: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./src/librustc_data_structures/profiling.rs:568
 103: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./src/librustc_session/utils.rs:9
 104: rustc_interface::passes::analysis
             at src/librustc_interface/passes.rs:861
 105: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:357
 106: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::{{closure}}
             at ./src/librustc_query_system/dep_graph/graph.rs:249
 107: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}::{{closure}}
             at ./src/librustc_middle/dep_graph/mod.rs:71
 108: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
 109: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
 110: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
 111: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::{{closure}}
             at ./src/librustc_middle/dep_graph/mod.rs:71
 112: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1767
 113: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1756
 114: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1767
 115: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
             at ./src/librustc_middle/dep_graph/mod.rs:68
 116: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:249
 117: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             at ./src/librustc_query_system/dep_graph/graph.rs:336
 118: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:584
 119: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 120: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
 121: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
 122: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
 123: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 124: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1783
 125: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1767
 126: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1756
 127: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1767
 128: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1780
 129: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
 130: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:582
 131: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:292
 132: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:581
 133: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:461
 134: rustc_query_system::query::plumbing::get_query::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:626
 135: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:91
 136: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:367
 137: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:618
 138: rustc_middle::ty::query::TyCtxtAt::analysis
             at ./src/librustc_middle/ty/query/plumbing.rs:462
 139: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::analysis
             at ./src/librustc_middle/ty/query/plumbing.rs:425
 140: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:385
 141: rustc_middle::ty::context::tls::enter_global::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1718
 142: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
 143: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
 144: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
 145: rustc_middle::ty::context::tls::enter_global
             at ./src/librustc_middle/ty/context.rs:1718
 146: rustc_interface::passes::QueryContext::enter
             at ./src/librustc_interface/passes.rs:709
 147: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:385
 148: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./src/librustc_interface/queries.rs:385
 149: rustc_driver::run_compiler::{{closure}}
             at src/librustc_driver/lib.rs:285
 150: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at ./src/librustc_interface/interface.rs:199
 151: rustc_interface::interface::run_compiler::{{closure}}
             at ./src/librustc_interface/interface.rs:213
 152: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:152
 153: scoped_tls::ScopedKey<T>::set
             at /home/mark/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 154: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:148
 155: scoped_tls::ScopedKey<T>::set
             at /home/mark/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 156: rustc_ast::attr::with_globals::{{closure}}
             at ./src/librustc_ast/attr/mod.rs:44
 157: scoped_tls::ScopedKey<T>::set
             at /home/mark/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 158: rustc_ast::attr::with_globals
             at ./src/librustc_ast/attr/mod.rs:44
 159: rustc_interface::util::spawn_thread_pool::{{closure}}
             at ./src/librustc_interface/util.rs:147
 160: rustc_interface::util::scoped_thread::{{closure}}
             at ./src/librustc_interface/util.rs:122
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=2 -C incremental -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-im
port-instr-limit=10 -C debug-assertions=y --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_mod_privacy] checking privacy in module `f32`
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
