
error: internal compiler error: librustc/traits/codegen/mod.rs:68: Encountered error `Unimplemented` selecting `Binder(<std::marker::PhantomData<Foo> as PostProcess<Foo>>)` during codegen

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:578:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::session::opt_span_bug_fmt
  13: rustc::session::bug_fmt
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
  28: rustc::util::common::time
  29: rustc_mir::monomorphize::collector::collect_crate_mono_items
  30: rustc::util::common::time
  31: rustc_codegen_llvm::base::collect_and_partition_mono_items
  32: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::collect_and_partition_mono_items<'tcx>>::compute
  33: rustc::dep_graph::graph::DepGraph::with_task_impl
  34: rustc::ty::context::tls::with_related_context
  35: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  37: rustc_codegen_llvm::back::symbol_export::exported_symbols_provider_local
  38: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::exported_symbols<'tcx>>::compute
  39: rustc::dep_graph::graph::DepGraph::with_task_impl
  40: rustc::ty::context::tls::with_related_context
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  42: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  43: rustc_metadata::encoder::encode_metadata
  44: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  45: rustc::ty::context::TyCtxt::encode_metadata
  46: rustc_codegen_llvm::base::write_metadata
  47: rustc::util::common::time
  48: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  49: rustc::util::common::time
  50: rustc_driver::driver::phase_4_codegen
  51: rustc_driver::driver::compile_input::{{closure}}
  52: rustc::ty::context::tls::enter_context
  53: <std::thread::local::LocalKey<T>>::with
  54: rustc::ty::context::TyCtxt::create_and_enter
  55: rustc_driver::driver::compile_input
  56: rustc_driver::run_compiler_with_pool
  57: <scoped_tls::ScopedKey<T>>::set
  58: <scoped_tls::ScopedKey<T>>::set
  59: syntax::with_globals
  60: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  61: __rust_maybe_catch_panic
  62: rustc_driver::run
  63: rustc_driver::main
  64: std::rt::lang_start::{{closure}}
  65: std::panicking::try::do_call
  66: __rust_maybe_catch_panic
  67: std::rt::lang_start_internal
  68: main
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `PostProcess` fulfills its obligations
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
#2 [exported_symbols] exported_symbols
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.29.0-nightly (97085f9fb 2018-08-01) running on x86_64-apple-darwin

note: compiler flags: --crate-type rlib
