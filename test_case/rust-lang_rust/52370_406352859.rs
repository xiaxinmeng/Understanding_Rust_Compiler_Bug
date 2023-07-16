
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
             at librustc/util/common.rs:54
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:479
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:390
   7: rust_begin_unwind
             at libstd/panicking.rs:325
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:77
   9: core::panicking::panic_bounds_check
             at libcore/panicking.rs:59
  10: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::def_path_hash
             at /home/oliver/Projects/rust/rust/src/libcore/slice/mod.rs:2085
             at /home/oliver/Projects/rust/rust/src/libcore/slice/mod.rs:1953
             at /home/oliver/Projects/rust/rust/src/liballoc/vec.rs:1714
             at /home/oliver/Projects/rust/rust/src/librustc/hir/map/definitions.rs:89
             at librustc_metadata/decoder.rs:1086
             at librustc_metadata/cstore_impl.rs:494
  11: rustc::ich::impls_hir::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext<'a>> for rustc::hir::def_id::DefId>::hash_stable
             at librustc/ich/hcx.rs:152
             at librustc/ich/impls_hir.rs:29
  12: <[T] as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at librustc/macros.rs:86
             at librustc/macros.rs:103
             at /home/oliver/Projects/rust/rust/src/librustc_data_structures/stable_hasher.rs:287
  13: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/oliver/Projects/rust/rust/src/librustc_data_structures/stable_hasher.rs:297
             at /home/oliver/Projects/rust/rust/src/librustc_data_structures/stable_hasher.rs:315
             at librustc/dep_graph/graph.rs:286
  14: rustc::dep_graph::graph::DepGraph::with_task
             at librustc/dep_graph/graph.rs:207
  15: rustc::ty::context::tls::set_tlv
             at librustc/ty/query/plumbing.rs:533
             at librustc/ty/query/plumbing.rs:203
             at librustc/ty/context.rs:1836
             at librustc/ty/context.rs:1775
  16: rustc::ty::context::tls::with_context_opt
             at librustc/ty/context.rs:1835
             at librustc/ty/query/plumbing.rs:202
             at librustc/ty/context.rs:1936
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1911
  17: rustc::ty::context::tls::with_related_context
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1931
  18: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
             at librustc/ty/query/plumbing.rs:192
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
             at librustc/ty/query/plumbing.rs:526
  20: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
             at librustc/ty/query/plumbing.rs:412
             at librustc/ty/query/plumbing.rs:602
             at librustc/ty/query/plumbing.rs:613
  21: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::item_children
             at librustc/ty/query/plumbing.rs:830
  22: core::ops::function::FnOnce::call_once
             at librustc_metadata/cstore_impl.rs:402
             at /home/oliver/Projects/rust/rust/src/libcore/ops/function.rs:223
  23: rustc::ty::query::__query_compute::visible_parent_map
             at librustc/ty/query/plumbing.rs:783
             at librustc/ty/query/plumbing.rs:752
  24: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::visible_parent_map<'tcx>>::compute
             at librustc/ty/query/plumbing.rs:781
  25: rustc::ty::context::tls::with_context_opt
             at librustc/dep_graph/graph.rs:274
             at librustc/ty/context.rs:1836
             at librustc/ty/context.rs:1775
             at librustc/ty/context.rs:1835
             at librustc/dep_graph/graph.rs:273
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1911
  26: rustc::ty::context::tls::with_context
             at librustc/ty/context.rs:1920
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
             at librustc/dep_graph/graph.rs:267
  28: rustc::dep_graph::graph::DepGraph::with_task
             at librustc/dep_graph/graph.rs:207
  29: rustc::ty::context::tls::set_tlv
             at librustc/ty/query/plumbing.rs:533
             at librustc/ty/query/plumbing.rs:203
             at librustc/ty/context.rs:1836
             at librustc/ty/context.rs:1775
  30: rustc::ty::context::tls::with_context_opt
             at librustc/ty/context.rs:1835
             at librustc/ty/query/plumbing.rs:202
             at librustc/ty/context.rs:1936
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1911
  31: rustc::ty::context::tls::with_related_context
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1931
  32: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
             at librustc/ty/query/plumbing.rs:192
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
             at librustc/ty/query/plumbing.rs:526
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
             at librustc/ty/query/plumbing.rs:412
             at librustc/ty/query/plumbing.rs:602
             at librustc/ty/query/plumbing.rs:613
  35: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::visible_parent_map
             at librustc/ty/query/plumbing.rs:830
  36: rustc::ty::item_path::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_push_visible_item_path
             at librustc/ty/item_path.rs:132
  37: rustc::ty::item_path::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::push_item_path
             at librustc/ty/item_path.rs:192
  38: rustc::ty::item_path::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::item_path_str
             at librustc/ty/item_path.rs:67
  39: rustc::ty::context::tls::with
             at librustc_mir/interpret/const_eval.rs:165
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1966
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1920
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1911
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1920
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1966
  40: rustc_mir::interpret::const_eval::eval_body_using_ecx
             at librustc_mir/interpret/const_eval.rs:165
  41: rustc_mir::interpret::const_eval::const_eval_provider
             at librustc_mir/interpret/const_eval.rs:131
             at librustc_mir/interpret/const_eval.rs:562
  42: rustc::ty::query::__query_compute::const_eval
             at librustc/ty/query/plumbing.rs:783
             at librustc/ty/query/plumbing.rs:752
  43: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval<'tcx>>::compute
             at librustc/ty/query/plumbing.rs:781
  44: rustc::ty::context::tls::with_context_opt
             at librustc/dep_graph/graph.rs:274
             at librustc/ty/context.rs:1836
             at librustc/ty/context.rs:1775
             at librustc/ty/context.rs:1835
             at librustc/dep_graph/graph.rs:273
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1911
  45: rustc::ty::context::tls::with_context
             at librustc/ty/context.rs:1920
  46: rustc::dep_graph::graph::DepGraph::with_task_impl
             at librustc/dep_graph/graph.rs:267
  47: rustc::dep_graph::graph::DepGraph::with_task
             at librustc/dep_graph/graph.rs:207
  48: rustc::ty::context::tls::set_tlv
             at librustc/ty/query/plumbing.rs:533
             at librustc/ty/query/plumbing.rs:203
             at librustc/ty/context.rs:1836
             at librustc/ty/context.rs:1775
  49: rustc::ty::context::tls::with_context_opt
             at librustc/ty/context.rs:1835
             at librustc/ty/query/plumbing.rs:202
             at librustc/ty/context.rs:1936
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1911
  50: rustc::ty::context::tls::with_related_context
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1931
  51: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
             at librustc/ty/query/plumbing.rs:192
  52: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
             at librustc/ty/query/plumbing.rs:526
  53: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
             at librustc/ty/query/plumbing.rs:412
             at librustc/ty/query/plumbing.rs:602
             at librustc/ty/query/plumbing.rs:613
  54: rustc::ty::query::TyCtxtAt::const_eval
             at librustc/ty/query/plumbing.rs:837
  55: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::const_eval
             at librustc/ty/query/plumbing.rs:830
  56: rustc_mir::monomorphize::collector::collect_const
             at librustc_mir/monomorphize/collector.rs:1261
  57: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_const
             at librustc_mir/monomorphize/collector.rs:607
  58: rustc::mir::visit::Visitor::super_terminator_kind
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:759
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:197
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:751
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:191
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:628
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:144
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:472
  59: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
             at librustc_mir/monomorphize/collector.rs:653
  60: rustc::mir::visit::Visitor::super_mir
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:418
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:119
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:326
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:92
             at /home/oliver/Projects/rust/rust/src/librustc/mir/visit.rs:289
  61: rustc_mir::monomorphize::collector::collect_items_rec
             at librustc_mir/monomorphize/collector.rs:1198
             at librustc_mir/monomorphize/collector.rs:412
  62: rustc_mir::monomorphize::collector::collect_items_rec
             at librustc_mir/monomorphize/collector.rs:425
  63: rustc_mir::monomorphize::collector::collect_items_rec
             at librustc_mir/monomorphize/collector.rs:425
  64: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
             at librustc_mir/monomorphize/collector.rs:318
             at /home/oliver/Projects/rust/rust/src/libcore/iter/iterator.rs:551
             at /home/oliver/Projects/rust/rust/src/libcore/iter/iterator.rs:1632
             at /home/oliver/Projects/rust/rust/src/libcore/iter/iterator.rs:1520
             at /home/oliver/Projects/rust/rust/src/libcore/iter/iterator.rs:1632
             at /home/oliver/Projects/rust/rust/src/libcore/iter/iterator.rs:551
             at librustc_mir/monomorphize/collector.rs:316
  65: rustc::util::common::time_ext
             at /home/oliver/Projects/rust/rust/src/librustc/util/common.rs:166
  66: rustc::util::common::time
             at /home/oliver/Projects/rust/rust/src/librustc/util/common.rs:160
  67: rustc_mir::monomorphize::collector::collect_crate_mono_items
             at librustc_mir/monomorphize/collector.rs:315
  68: rustc::util::common::time_ext
             at librustc_codegen_llvm/base.rs:1000
             at /home/oliver/Projects/rust/rust/src/librustc/util/common.rs:166
  69: rustc::util::common::time
             at /home/oliver/Projects/rust/rust/src/librustc/util/common.rs:160
  70: rustc_codegen_llvm::base::collect_and_partition_mono_items
             at librustc_codegen_llvm/base.rs:999
  71: rustc::ty::query::__query_compute::collect_and_partition_mono_items
             at librustc/ty/query/plumbing.rs:783
             at librustc/ty/query/plumbing.rs:752
  72: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::collect_and_partition_mono_items<'tcx>>::compute
             at librustc/ty/query/plumbing.rs:781
  73: rustc::ty::context::tls::with_context_opt
             at librustc/dep_graph/graph.rs:274
             at librustc/ty/context.rs:1836
             at librustc/ty/context.rs:1775
             at librustc/ty/context.rs:1835
             at librustc/dep_graph/graph.rs:273
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1911
  74: rustc::ty::context::tls::with_context
             at librustc/ty/context.rs:1920
  75: rustc::dep_graph::graph::DepGraph::with_task_impl
             at librustc/dep_graph/graph.rs:267
  76: rustc::dep_graph::graph::DepGraph::with_eval_always_task
             at librustc/dep_graph/graph.rs:391
  77: rustc::ty::context::tls::set_tlv
             at librustc/ty/query/plumbing.rs:528
             at librustc/ty/query/plumbing.rs:203
             at librustc/ty/context.rs:1836
             at librustc/ty/context.rs:1775
  78: rustc::ty::context::tls::with_context_opt
             at librustc/ty/context.rs:1835
             at librustc/ty/query/plumbing.rs:202
             at librustc/ty/context.rs:1936
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1911
  79: rustc::ty::context::tls::with_related_context
             at librustc/ty/context.rs:1920
             at librustc/ty/context.rs:1931
  80: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
             at librustc/ty/query/plumbing.rs:192
  81: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
             at librustc/ty/query/plumbing.rs:526
  82: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
             at librustc/ty/query/plumbing.rs:412
             at librustc/ty/query/plumbing.rs:602
             at librustc/ty/query/plumbing.rs:613
  83: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::collect_and_partition_mono_items
             at librustc/ty/query/plumbing.rs:830
  84: rustc_codegen_llvm::base::codegen_crate
             at librustc_codegen_llvm/base.rs:789
  85: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
             at librustc_codegen_llvm/lib.rs:204
  86: rustc::util::common::time_ext
             at librustc_driver/driver.rs:1335
             at /home/oliver/Projects/rust/rust/src/librustc/util/common.rs:166
  87: rustc::util::common::time
             at /home/oliver/Projects/rust/rust/src/librustc/util/common.rs:160
  88: rustc_driver::driver::phase_4_codegen
             at librustc_driver/driver.rs:1335
  89: rustc_driver::driver::compile_input::{{closure}}
             at librustc_driver/driver.rs:325
  90: rustc::ty::context::tls::set_tlv
             at librustc_driver/driver.rs:1319
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1868
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1836
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1775
  91: rustc::ty::context::tls::enter_context
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1835
  92: <std::thread::local::LocalKey<T>>::try_with
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1867
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1825
             at /home/oliver/Projects/rust/rust/src/libstd/thread/local.rs:294
  93: <std::thread::local::LocalKey<T>>::try_with
             at /home/oliver/Projects/rust/rust/src/libstd/thread/local.rs:248
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1817
             at /home/oliver/Projects/rust/rust/src/libstd/thread/local.rs:294
  94: <std::thread::local::LocalKey<T>>::with
             at /home/oliver/Projects/rust/rust/src/libstd/thread/local.rs:248
  95: rustc::ty::context::tls::enter_global
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1809
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1847
  96: rustc::ty::context::TyCtxt::create_and_enter
             at /home/oliver/Projects/rust/rust/src/librustc/ty/context.rs:1186
  97: rustc_driver::driver::phase_3_run_analysis_passes
             at librustc_driver/driver.rs:1227
  98: rustc_driver::driver::compile_input
             at librustc_driver/driver.rs:284
  99: rustc_driver::run_compiler_with_pool
             at librustc_driver/lib.rs:553
query stack during panic:

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
