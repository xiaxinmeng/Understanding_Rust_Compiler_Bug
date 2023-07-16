rust
error: internal compiler error: librustc/traits/codegen/mod.rs:68: Encountered error `Unimplemented` selecting `Binder(<dyn std::string::ToString as std::default::Default>)` during codegen

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:600:9
stack backtrace:
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
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:480
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc::ty::context::tls::with_related_context
  15: rustc::infer::InferCtxtBuilder::enter
  16: rustc::traits::codegen::codegen_fulfill_obligation
  17: rustc::ty::query::__query_compute::codegen_fulfill_obligation
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::codegen_fulfill_obligation<'tcx>>::compute
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc::ty::context::tls::with_related_context
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  23: rustc::ty::instance::Instance::resolve
  24: rustc_mir::monomorphize::collector::visit_fn_use
  25: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
  26: rustc_mir::monomorphize::collector::collect_items_rec
  27: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  28: rustc_mir::monomorphize::collector::collect_crate_mono_items
  29: rustc::util::common::time
  30: rustc_codegen_llvm::base::collect_and_partition_mono_items
  31: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::collect_and_partition_mono_items<'tcx>>::compute
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::context::tls::with_related_context
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  35: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  36: rustc_codegen_llvm::base::codegen_crate
  37: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  38: rustc::util::common::time
  39: rustc_driver::driver::phase_4_codegen
  40: rustc_driver::driver::compile_input::{{closure}}
  41: rustc::ty::context::tls::enter_context
  42: <std::thread::local::LocalKey<T>>::with
  43: rustc::ty::context::TyCtxt::create_and_enter
  44: rustc_driver::driver::compile_input
  45: rustc_driver::run_compiler_with_pool
  46: rustc_driver::driver::spawn_thread_pool
  47: rustc_driver::run_compiler
  48: <scoped_tls::ScopedKey<T>>::set
  49: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  50: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  51: rustc_driver::run
  52: rustc_driver::main
  53: std::rt::lang_start::{{closure}}
  54: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  55: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  56: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  57: main
  58: __libc_start_main
  59: <unknown>
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::default::Default` fulfills its obligations
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.31.0-nightly (f99911a4a 2018-10-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin
