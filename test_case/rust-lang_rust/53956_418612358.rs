rust
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::continue_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::panicking::panic_bounds_check
  10: <rustc::ty::query::on_disk_cache::CacheDecoder<'a, 'tcx, 'x> as serialize::serialize::SpecializedDecoder<syntax_pos::span_encoding::Span>>::specialized_decode
  11: serialize::serialize::Decoder::read_struct
  12: serialize::serialize::Decoder::read_seq
  13: serialize::serialize::Decoder::read_struct
  14: serialize::serialize::Decoder::read_seq
  15: <rustc::mir::Mir<'tcx> as serialize::serialize::Decodable>::decode::{{closure}}
  16: rustc::ty::query::on_disk_cache::OnDiskCache::try_load_query_result
  17: <rustc::ty::query::queries::optimized_mir<'tcx> as rustc::ty::query::config::QueryDescription<'tcx>>::try_load_from_disk
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_query
  19: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::instance_mir
  20: rustc_mir::monomorphize::collector::collect_items_rec
  21: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  22: rustc_mir::monomorphize::collector::collect_crate_mono_items
  23: rustc::util::common::time
  24: rustc_codegen_llvm::base::collect_and_partition_mono_items
  25: rustc::ty::context::tls::with_context
  26: rustc::ty::context::tls::with_related_context
  27: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  28: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  29: rustc::util::common::time
  30: rustc_driver::driver::phase_4_codegen
  31: rustc_driver::driver::compile_input::{{closure}}
  32: rustc::ty::context::tls::enter_context
  33: <std::thread::local::LocalKey<T>>::with
  34: rustc::ty::context::TyCtxt::create_and_enter
  35: rustc_driver::driver::compile_input
  36: rustc_driver::run_compiler_with_pool
  37: <scoped_tls::ScopedKey<T>>::set
  38: rustc_driver::run_compiler
  39: <scoped_tls::ScopedKey<T>>::set
  40: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  41: __rust_maybe_catch_panic
  42: rustc_driver::run
  43: rustc_driver::main
  44: std::rt::lang_start::{{closure}}
  45: std::panicking::try::do_call
  46: __rust_maybe_catch_panic
  47: std::rt::lang_start_internal
  48: main
  49: __libc_start_main
  50: <unknown>
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
