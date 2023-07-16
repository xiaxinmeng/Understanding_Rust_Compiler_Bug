
error[E0326]: implemented const `VALUE` has an incompatible type for trait
 --> foo.rs:8:18
  |
2 |     const VALUE: usize;
  |                  ----- type in trait
...
8 |     const VALUE: i32 = 0;
  |                  ^^^ expected `usize`, found `i32`

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size { raw: 4 }`,
 right: `Size { raw: 8 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:740:21
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/centril/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /home/centril/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1504
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
  13: rust_begin_unwind
             at src/libstd/panicking.rs:419
  14: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:373
  15: rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::write_immediate_no_validate
             at src/librustc_mir/interpret/place.rs:0
  16: rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::copy_op_no_validate
             at src/librustc_mir/interpret/place.rs:886
  17: rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::copy_op
             at src/librustc_mir/interpret/place.rs:849
  18: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
             at src/librustc_mir/interpret/step.rs:149
  19: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::statement
             at src/librustc_mir/interpret/step.rs:90
  20: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step
             at src/librustc_mir/interpret/step.rs:67
  21: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run
             at src/librustc_mir/interpret/step.rs:34
  22: rustc_mir::const_eval::eval_queries::eval_body_using_ecx
             at src/librustc_mir/const_eval/eval_queries.rs:56
  23: rustc_mir::const_eval::eval_queries::const_eval_raw_provider::{{closure}}
             at src/librustc_mir/const_eval/eval_queries.rs:310
  24: core::result::Result<T,E>::and_then
             at ./src/libcore/result.rs:729
  25: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
             at src/librustc_mir/const_eval/eval_queries.rs:310
  26: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_raw>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:357
  27: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
  28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./src/librustc_query_system/dep_graph/graph.rs:200
  29: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:592
  30: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
  31: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
  32: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
  33: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
  34: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
  35: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1783
  36: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1767
  37: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1756
  38: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1767
  39: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1780
  40: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
  41: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:582
  42: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:292
  43: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:581
  44: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:409
  45: rustc_query_system::query::plumbing::get_query::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:626
  46: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:91
  47: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:367
  48: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:618
  49: rustc_middle::ty::query::TyCtxtAt::const_eval_raw
             at ./src/librustc_middle/ty/query/plumbing.rs:462
  50: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_raw
             at ./src/librustc_middle/ty/query/plumbing.rs:425
  51: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
             at src/librustc_mir/const_eval/eval_queries.rs:254
  52: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:357
  53: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
  54: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./src/librustc_query_system/dep_graph/graph.rs:200
  55: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:592
  56: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
  57: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
  58: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
  59: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
  60: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
  61: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1783
  62: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1767
  63: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1756
  64: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1767
  65: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1780
  66: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
  67: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:582
  68: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:292
  69: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:581
  70: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:409
  71: rustc_query_system::query::plumbing::get_query::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:626
  72: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:91
  73: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:367
  74: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:618
  75: rustc_middle::ty::query::TyCtxtAt::const_eval_validated
             at ./src/librustc_middle/ty/query/plumbing.rs:462
  76: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_validated
             at ./src/librustc_middle/ty/query/plumbing.rs:425
  77: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
             at src/librustc_mir/const_eval/eval_queries.rs:231
  78: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
             at src/librustc_middle/ty/query/plumbing.rs:357
  79: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
  80: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./src/librustc_query_system/dep_graph/graph.rs:200
  81: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:592
  82: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:71
  83: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1695
  84: rustc_middle::ty::context::tls::set_tlv
             at src/librustc_middle/ty/context.rs:1679
  85: rustc_middle::ty::context::tls::enter_context
             at src/librustc_middle/ty/context.rs:1695
  86: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:71
  87: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1783
  88: rustc_middle::ty::context::tls::with_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1767
  89: rustc_middle::ty::context::tls::with_context_opt
             at src/librustc_middle/ty/context.rs:1756
  90: rustc_middle::ty::context::tls::with_context
             at src/librustc_middle/ty/context.rs:1767
  91: rustc_middle::ty::context::tls::with_related_context
             at src/librustc_middle/ty/context.rs:1780
  92: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at src/librustc_middle/ty/query/plumbing.rs:60
  93: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:582
  94: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:292
  95: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:581
  96: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:409
  97: rustc_query_system::query::plumbing::get_query::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:626
  98: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:91
  99: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:367
 100: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:618
 101: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id
             at src/librustc_middle/mir/interpret/queries.rs:0
 102: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve
             at src/librustc_middle/mir/interpret/queries.rs:45
 103: rustc_middle::ty::sty::Const::eval::{{closure}}
             at ./src/librustc_middle/ty/sty.rs:2373
 104: rustc_middle::ty::sty::Const::eval
             at ./src/librustc_middle/ty/sty.rs:2394
 105: <rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
             at src/librustc_trait_selection/traits/project.rs:391
 106: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::sty::Const>::fold_with
             at ./src/librustc_middle/ty/structural_impls.rs:1005
 107: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
             at ./src/librustc_middle/ty/structural_impls.rs:874
 108: <rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
             at src/librustc_trait_selection/traits/project.rs:321
 109: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::fold_with
             at ./src/librustc_middle/ty/structural_impls.rs:914
 110: rustc_trait_selection::traits::project::AssocTypeNormalizer::fold
             at ./src/librustc_trait_selection/traits/project.rs:297
 111: rustc_trait_selection::traits::project::normalize_with_depth_to
             at ./src/librustc_trait_selection/traits/project.rs:264
 112: rustc_trait_selection::traits::project::normalize_to
             at ./src/librustc_trait_selection/traits/project.rs:232
 113: rustc_trait_selection::traits::project::normalize
             at ./src/librustc_trait_selection/traits/project.rs:218
 114: <rustc_infer::infer::InferCtxt as rustc_trait_selection::infer::InferCtxtExt>::partially_normalize_associated_types_in
             at ./src/librustc_trait_selection/infer.rs:75
 115: rustc_typeck::check::Inherited::normalize_associated_types_in
             at src/librustc_typeck/check/mod.rs:725
 116: rustc_typeck::check::FnCtxt::normalize_associated_types_in
             at src/librustc_typeck/check/mod.rs:3231
 117: <rustc_typeck::check::FnCtxt as rustc_typeck::astconv::AstConv>::normalize_ty
             at src/librustc_typeck/check/mod.rs:2813
 118: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty
             at src/librustc_typeck/astconv.rs:2776
 119: rustc_typeck::check::FnCtxt::to_ty
             at src/librustc_typeck/check/mod.rs:3296
 120: <rustc_typeck::check::GatherLocalsVisitor as rustc_hir::intravisit::Visitor>::visit_local
             at src/librustc_typeck/check/mod.rs:1195
 121: rustc_hir::intravisit::walk_stmt
             at ./src/librustc_hir/intravisit.rs:1051
 122: rustc_hir::intravisit::Visitor::visit_stmt
             at ./src/librustc_hir/intravisit.rs:339
 123: rustc_hir::intravisit::walk_block
             at ./<::rustc_ast::visit::walk_list macros>:2
 124: rustc_hir::intravisit::walk_expr
             at ./src/librustc_hir/intravisit.rs:0
 125: rustc_hir::intravisit::Visitor::visit_expr
             at ./src/librustc_hir/intravisit.rs:351
 126: rustc_hir::intravisit::walk_body
             at ./src/librustc_hir/intravisit.rs:485
 127: rustc_hir::intravisit::Visitor::visit_body
             at ./src/librustc_hir/intravisit.rs:302
 128: rustc_typeck::check::check_fn
             at src/librustc_typeck/check/mod.rs:1335
 129: rustc_typeck::check::typeck_tables_of_with_fallback::{{closure}}
             at src/librustc_typeck/check/mod.rs:1035
 130: rustc_typeck::check::InheritedBuilder::enter::{{closure}}
             at src/librustc_typeck/check/mod.rs:660
 131: rustc_infer::infer::InferCtxtBuilder::enter::{{closure}}
             at ./src/librustc_infer/infer/mod.rs:595
 132: rustc_middle::ty::context::GlobalCtxt::enter_local::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1527
 133: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
 134: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
 135: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
 136: rustc_middle::ty::context::GlobalCtxt::enter_local::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1527
 137: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1783
 138: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1767
 139: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1756
 140: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1767
 141: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1780
 142: rustc_middle::ty::context::GlobalCtxt::enter_local
             at ./src/librustc_middle/ty/context.rs:1519
 143: rustc_infer::infer::InferCtxtBuilder::enter
             at ./src/librustc_infer/infer/mod.rs:594
 144: rustc_typeck::check::InheritedBuilder::enter
             at src/librustc_typeck/check/mod.rs:660
 145: rustc_typeck::check::typeck_tables_of_with_fallback
             at src/librustc_typeck/check/mod.rs:1005
 146: rustc_typeck::check::typeck_tables_of
             at src/librustc_typeck/check/mod.rs:966
 147: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_tables_of>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:357
 148: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
 149: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./src/librustc_query_system/dep_graph/graph.rs:200
 150: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:592
 151: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 152: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
 153: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
 154: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
 155: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 156: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1783
 157: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1767
 158: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1756
 159: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1767
 160: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1780
 161: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
 162: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:582
 163: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:292
 164: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:581
 165: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:409
 166: rustc_query_system::query::plumbing::get_query::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:626
 167: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:91
 168: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:367
 169: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:618
 170: rustc_query_system::query::plumbing::ensure_query
             at ./src/librustc_query_system/query/plumbing.rs:660
 171: rustc_middle::ty::query::TyCtxtEnsure::typeck_tables_of
             at ./src/librustc_middle/ty/query/plumbing.rs:384
 172: rustc_typeck::check::typeck_item_bodies::{{closure}}
             at src/librustc_typeck/check/mod.rs:754
 173: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners::{{closure}}
             at ./src/librustc_middle/ty/mod.rs:2724
 174: core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
             at ./src/libcore/iter/traits/iterator.rs:655
 175: core::iter::traits::iterator::Iterator::fold::ok::{{closure}}
             at ./src/libcore/iter/traits/iterator.rs:2002
 176: core::iter::traits::iterator::Iterator::try_fold
             at ./src/libcore/iter/traits/iterator.rs:1878
 177: core::iter::traits::iterator::Iterator::fold
             at ./src/libcore/iter/traits/iterator.rs:2005
 178: core::iter::traits::iterator::Iterator::for_each
             at ./src/libcore/iter/traits/iterator.rs:658
 179: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
             at ./src/librustc_middle/ty/mod.rs:2723
 180: rustc_typeck::check::typeck_item_bodies
             at src/librustc_typeck/check/mod.rs:753
 181: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_item_bodies>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:357
 182: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
 183: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./src/librustc_query_system/dep_graph/graph.rs:200
 184: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:592
 185: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 186: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
 187: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
 188: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
 189: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 190: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1783
 191: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1767
 192: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1756
 193: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1767
 194: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1780
 195: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
 196: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:582
 197: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:292
 198: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:581
 199: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:409
 200: rustc_query_system::query::plumbing::get_query::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:626
 201: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:91
 202: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:367
 203: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:618
 204: rustc_middle::ty::query::TyCtxtAt::typeck_item_bodies
             at ./src/librustc_middle/ty/query/plumbing.rs:462
 205: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::typeck_item_bodies
             at ./src/librustc_middle/ty/query/plumbing.rs:425
 206: rustc_typeck::check_crate::{{closure}}
             at src/librustc_typeck/lib.rs:346
 207: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./src/librustc_data_structures/profiling.rs:568
 208: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./src/librustc_session/utils.rs:9
 209: rustc_typeck::check_crate
             at src/librustc_typeck/lib.rs:346
 210: rustc_interface::passes::analysis
             at src/librustc_interface/passes.rs:808
 211: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:357
 212: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
 213: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             at ./src/librustc_query_system/dep_graph/graph.rs:336
 214: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:584
 215: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 216: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
 217: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
 218: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
 219: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 220: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1783
 221: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1767
 222: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1756
 223: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1767
 224: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1780
 225: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
 226: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:582
 227: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:292
 228: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:581
 229: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:409
 230: rustc_query_system::query::plumbing::get_query::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:626
 231: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:91
 232: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:367
 233: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:618
 234: rustc_middle::ty::query::TyCtxtAt::analysis
             at ./src/librustc_middle/ty/query/plumbing.rs:462
 235: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::analysis
             at ./src/librustc_middle/ty/query/plumbing.rs:425
 236: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:385
 237: rustc_middle::ty::context::tls::enter_global::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1718
 238: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1695
 239: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1679
 240: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1695
 241: rustc_middle::ty::context::tls::enter_global
             at ./src/librustc_middle/ty/context.rs:1718
 242: rustc_interface::passes::QueryContext::enter
             at ./src/librustc_interface/passes.rs:709
 243: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:385
 244: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./src/librustc_interface/queries.rs:385
 245: rustc_driver::run_compiler::{{closure}}
             at src/librustc_driver/lib.rs:285
 246: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at ./src/librustc_interface/interface.rs:199
 247: rustc_interface::interface::run_compiler::{{closure}}
             at ./src/librustc_interface/interface.rs:213
 248: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:152
 249: scoped_tls::ScopedKey<T>::set
             at /home/centril/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 250: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:148
 251: scoped_tls::ScopedKey<T>::set
             at /home/centril/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 252: rustc_ast::attr::with_globals::{{closure}}
             at ./src/librustc_ast/attr/mod.rs:44
 253: scoped_tls::ScopedKey<T>::set
             at /home/centril/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 254: rustc_ast::attr::with_globals
             at ./src/librustc_ast/attr/mod.rs:44
 255: rustc_interface::util::spawn_thread_pool::{{closure}}
             at ./src/librustc_interface/util.rs:147
 256: rustc_interface::util::scoped_thread::{{closure}}
             at ./src/librustc_interface/util.rs:122
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [const_eval_raw] const-evaluating `main::{{constant}}#0`
#1 [const_eval_validated] const-evaluating + checking `main::{{constant}}#0`
#2 [const_eval_validated] const-evaluating + checking `main::{{constant}}#0`
#3 [typeck_tables_of] type-checking `main`
#4 [typeck_item_bodies] type-checking all item bodies
#5 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0326`.
