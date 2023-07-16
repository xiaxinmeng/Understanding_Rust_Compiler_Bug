
$ RUST_BACKTRACE=1 cargo tarpaulin
Building project
warning: unused `core::result::Result` that must be used
   --> src/test_runner/failure_persistence/file.rs:308:5
    |
308 |     write!(buf, "{}", format_basic_seed_line(seed));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: #[warn(unused_must_use)] on by default
    = note: this `Result` may be an `Err` variant, which should be handled
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: internal compiler error: librustc/traits/codegen/mod.rs:68: Encountered error `Unimplemented` selecting `Binder(<core::char::DecodeUtf16<<std::vec::Vec<u16> as core::iter::IntoIterator>::IntoIter> as arbitrary::traits::Arbitrary>)` during codegen

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
  19: rustc::ty::context::tls::with_context
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  24: rustc::ty::instance::Instance::resolve
  25: <rustc_mir::monomorphize::collector::RootCollector<'b, 'a, 'v> as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  26: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  27: rustc_mir::monomorphize::collector::collect_crate_mono_items
  28: rustc::util::common::time
  29: rustc_codegen_llvm::base::collect_and_partition_mono_items
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::collect_and_partition_mono_items<'tcx>>::compute
  31: rustc::ty::context::tls::with_context
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  35: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  36: rustc_codegen_llvm::back::symbol_export::exported_symbols_provider_local
  37: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::exported_symbols<'tcx>>::compute
  38: rustc::ty::context::tls::with_context
  39: rustc::dep_graph::graph::DepGraph::with_task_impl
  40: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  42: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  43: rustc_metadata::encoder::encode_metadata
  44: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  45: rustc::ty::context::TyCtxt::encode_metadata
  46: rustc_codegen_llvm::base::write_metadata
  47: rustc::util::common::time
  48: rustc_codegen_llvm::base::codegen_crate
  49: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  50: rustc::util::common::time
  51: rustc_driver::driver::phase_4_codegen
  52: rustc_driver::driver::compile_input::{{closure}}
  53: rustc::ty::context::tls::enter_context
  54: <std::thread::local::LocalKey<T>>::with
  55: rustc::ty::context::TyCtxt::create_and_enter
  56: rustc_driver::driver::compile_input
  57: rustc_driver::run_compiler_with_pool
  58: rustc_driver::driver::spawn_thread_pool
  59: rustc_driver::run_compiler
  60: <scoped_tls::ScopedKey<T>>::set
  61: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  62: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  63: rustc_driver::run
  64: rustc_driver::main
  65: std::rt::lang_start::{{closure}}
  66: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  67: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  68: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  69: main
  70: __libc_start_main
  71: <unknown>
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `arbitrary::traits::Arbitrary` fulfills its obligations
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
#2 [exported_symbols] exported_symbols
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.31.0 (abe02cefd 2018-12-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental -C relocation-model=dynamic-no-pic -C link-dead-code -C opt-level=0 --crate-type lib

note: some of the compiler flags provided by cargo are hidden
