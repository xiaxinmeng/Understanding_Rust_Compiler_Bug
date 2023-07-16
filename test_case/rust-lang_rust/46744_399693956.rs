
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:207
   3: std::panicking::default_hook
             at libstd/panicking.rs:223
   4: core::ops::function::Fn::call
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:403
   6: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:349
   7: rust_begin_unwind
             at libstd/panicking.rs:325
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:72
   9: core::panicking::panic_bounds_check
             at libcore/panicking.rs:58
  10: <rustc::ty::maps::on_disk_cache::CacheDecoder<'a, 'tcx, 'x> as serialize::serialize::SpecializedDecoder<syntax_pos::span_encoding::Span>>::specialized_decode
  11: serialize::serialize::Decoder::read_struct
  12: rustc::ty::maps::on_disk_cache::OnDiskCache::try_load_query_result
  13: rustc::ty::maps::<impl rustc::ty::maps::queries::generics_of<'tcx>>::try_get
  14: rustc::ty::maps::TyCtxtAt::generics_of
  15: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::generics_of
  16: <rustc_typeck::collect::CollectItemTypesVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_trait_item
  17: rustc::hir::Crate::visit_all_item_likes
  18: rustc::session::Session::track_errors
  19: rustc_typeck::check_crate
  20: <std::thread::local::LocalKey<T>>::with
  21: <std::thread::local::LocalKey<T>>::with
  22: rustc::ty::context::TyCtxt::create_and_enter
  23: rustc_driver::driver::compile_input
  24: rustc_driver::run_compiler_impl
  25: syntax::with_globals

