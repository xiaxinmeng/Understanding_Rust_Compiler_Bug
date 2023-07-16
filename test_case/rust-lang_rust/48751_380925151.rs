
error: internal compiler error: librustc_mir/monomorphize/collector.rs:750: Cannot create local mono-item for DefId(44/0:489 ~ libgsh[4be4]::resolver_server[0]::send[0])

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:542:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:206
   3: std::panicking::default_hook
             at libstd/panicking.rs:222
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:401
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::session::opt_span_bug_fmt
  13: rustc::session::bug_fmt
  14: rustc_mir::monomorphize::collector::should_monomorphize_locally
  15: rustc_mir::monomorphize::collector::visit_instance_use
  16: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
  17: rustc_mir::monomorphize::collector::collect_items_rec
  18: rustc_mir::monomorphize::collector::collect_items_rec
  19: rustc_mir::monomorphize::collector::collect_items_rec
  20: rustc_mir::monomorphize::collector::collect_items_rec
  21: rustc_mir::monomorphize::collector::collect_items_rec
  22: rustc_mir::monomorphize::collector::collect_items_rec
  23: rustc_mir::monomorphize::collector::collect_items_rec
  24: rustc_mir::monomorphize::collector::collect_items_rec
  25: rustc_mir::monomorphize::collector::collect_items_rec
  26: rustc_mir::monomorphize::collector::collect_items_rec
  27: rustc_mir::monomorphize::collector::collect_items_rec
  28: rustc_mir::monomorphize::collector::collect_items_rec
  29: rustc_mir::monomorphize::collector::collect_crate_mono_items
  30: rustc::util::common::time
  31: rustc_trans::base::collect_and_partition_translation_items
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::context::tls::with_related_context
  34: rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::force_with_lock
  35: rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::try_get
  36: rustc::ty::maps::TyCtxtAt::collect_and_partition_translation_items
  37: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::collect_and_partition_translation_items
  38: rustc_trans::base::trans_crate
  39: <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate
  40: rustc::util::common::time
  41: rustc_driver::driver::phase_4_translate_to_llvm
  42: rustc_driver::driver::compile_input::{{closure}}
  43: rustc::ty::context::tls::enter_context
  44: <std::thread::local::LocalKey<T>>::with
  45: rustc::ty::context::TyCtxt::create_and_enter
  46: rustc_driver::driver::compile_input
  47: rustc_driver::run_compiler_impl
  48: syntax::with_globals
  49: rustc_driver::run
  50: rustc_driver::main
  51: std::rt::lang_start::{{closure}}
  52: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:305
  53: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:101
  54: std::rt::lang_start_internal
             at libstd/panicking.rs:284
             at libstd/panic.rs:361
             at libstd/rt.rs:58
  55: main
  56: __libc_start_main
  57: <unknown>
query stack during panic:
#0 [collect_and_partition_translation_items] collect_and_partition_translation_items
end of query stack
error: aborting due to previous error
