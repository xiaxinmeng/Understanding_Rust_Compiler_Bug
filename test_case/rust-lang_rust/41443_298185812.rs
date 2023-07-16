
---- src/lib.rs -  (line 40) stdout ----
	thread 'rustc' panicked at 'slice index starts at 2667773953 but ends at 4384', /checkout/src/libcore/slice/mod.rs:678
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:365
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:511
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:495
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:471
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:69
   9: core::slice::slice_index_order_fail
             at /checkout/src/libcore/slice/mod.rs:678
  10: rustc_metadata::index::<impl rustc_metadata::schema::LazySeq<rustc_metadata::index::Index>>::lookup
  11: rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::entry
  12: rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::get_variant
  13: rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::get_adt_def
  14: rustc_metadata::cstore_impl::provide::adt_def
  15: rustc::ty::maps::<impl rustc::ty::maps::queries::adt_def<'tcx>>::try_get
  16: rustc::ty::maps::TyCtxtAt::adt_def
  17: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::adt_def
  18: <rustc_metadata::decoder::DecodeContext<'a, 'tcx> as serialize::serialize::SpecializedDecoder<&'tcx rustc::ty::AdtDef>>::specialized_decode
  19: <rustc_metadata::decoder::DecodeContext<'a, 'tcx> as serialize::serialize::SpecializedDecoder<&'tcx rustc::ty::TyS<'tcx>>>::specialized_decode
  20: serialize::serialize::Decoder::read_enum_variant_arg
  21: <rustc_metadata::decoder::DecodeContext<'a, 'tcx> as serialize::serialize::SpecializedDecoder<&'tcx rustc::ty::TyS<'tcx>>>::specialized_decode
  22: <rustc_metadata::decoder::DecodeContext<'a, 'tcx> as serialize::serialize::SpecializedDecoder<&'tcx rustc::ty::TyS<'tcx>>>::specialized_decode
  23: rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::get_type
  24: rustc_metadata::cstore_impl::provide::type_of
  25: rustc::ty::maps::<impl rustc::ty::maps::queries::type_of<'tcx>>::try_get_with
  26: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::type_of
  27: rustc_typeck::check::FnCtxt::instantiate_value_path
  28: rustc_typeck::check::FnCtxt::check_expr_kind
  29: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  30: rustc_typeck::check::FnCtxt::check_expr_kind
  31: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  32: rustc_typeck::check::FnCtxt::check_decl_initializer
  33: rustc_typeck::check::FnCtxt::check_decl_local
  34: rustc_typeck::check::FnCtxt::check_stmt
  35: rustc_typeck::check::FnCtxt::check_block_with_expected::{{closure}}
  36: rustc_typeck::check::FnCtxt::check_block_with_expected
  37: rustc_typeck::check::FnCtxt::check_expr_kind
  38: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  39: rustc_typeck::check::FnCtxt::check_return_expr
  40: rustc_typeck::check::check_fn
  41: rustc_typeck::check::typeck_tables_of
  42: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  43: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  44: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
  45: rustc_typeck::check::typeck_item_bodies
  46: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  47: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  48: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  49: rustc_typeck::check_crate
  50: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  51: rustc_driver::driver::phase_3_run_analysis_passes
  52: rustc_driver::driver::compile_input
  53: std::panicking::try::do_call
  54: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
  55: rustdoc::test::runtest
  56: std::panicking::try::do_call
  57: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
  58: <F as alloc::boxed::FnBox<A>>::call_box
  59: std::sys::imp::thread::Thread::new::thread_start
             at /checkout/src/liballoc/boxed.rs:658
             at /checkout/src/libstd/sys_common/thread.rs:21
             at /checkout/src/libstd/sys/unix/thread.rs:84
  60: <unknown>
thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:273
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:365
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
   6: rustdoc::test::runtest
   7: std::panicking::try::do_call
   8: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
   9: <F as alloc::boxed::FnBox<A>>::call_box
  10: std::sys::imp::thread::Thread::new::thread_start
             at /checkout/src/liballoc/boxed.rs:658
             at /checkout/src/libstd/sys_common/thread.rs:21
             at /checkout/src/libstd/sys/unix/thread.rs:84
  11: <unknown>

---- src/lib.rs -  (line 55) stdout ----
	thread 'rustc' panicked at 'slice index starts at 2667773953 but ends at 4384', /checkout/src/libcore/slice/mod.rs:678
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:365
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:511
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:495
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:471
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:69
   9: core::slice::slice_index_order_fail
             at /checkout/src/libcore/slice/mod.rs:678
  10: rustc_metadata::index::<impl rustc_metadata::schema::LazySeq<rustc_metadata::index::Index>>::lookup
  11: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::item_children
  12: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::populate_module_if_necessary
  13: rustc_resolve::resolve_imports::<impl rustc_resolve::Resolver<'a>>::resolve_ident_in_module
  14: rustc_resolve::Resolver::resolve_path
  15: rustc_resolve::Resolver::resolve_qpath
  16: rustc_resolve::Resolver::smart_resolve_path_fragment
  17: rustc_resolve::Resolver::smart_resolve_path
  18: syntax::ast::Pat::walk
  19: rustc_resolve::Resolver::resolve_arm
  20: syntax::visit::walk_expr
  21: rustc_resolve::Resolver::resolve_expr
  22: rustc_resolve::Resolver::resolve_block
  23: rustc_resolve::Resolver::resolve_expr
  24: rustc_resolve::Resolver::resolve_block
  25: rustc_resolve::Resolver::resolve_expr
  26: rustc_resolve::Resolver::resolve_block
  27: <rustc_resolve::Resolver<'a> as syntax::visit::Visitor<'tcx>>::visit_fn
  28: syntax::visit::walk_item
  29: rustc_resolve::Resolver::resolve_item
  30: rustc_resolve::Resolver::resolve_crate
  31: rustc_driver::driver::phase_2_configure_and_expand
  32: rustc_driver::driver::compile_input
  33: std::panicking::try::do_call
  34: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
  35: rustdoc::test::runtest
  36: std::panicking::try::do_call
  37: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
  38: <F as alloc::boxed::FnBox<A>>::call_box
  39: std::sys::imp::thread::Thread::new::thread_start
             at /checkout/src/liballoc/boxed.rs:658
             at /checkout/src/libstd/sys_common/thread.rs:21
             at /checkout/src/libstd/sys/unix/thread.rs:84
  40: <unknown>
thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:273
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:365
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
   6: rustdoc::test::runtest
   7: std::panicking::try::do_call
   8: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
   9: <F as alloc::boxed::FnBox<A>>::call_box
  10: std::sys::imp::thread::Thread::new::thread_start
             at /checkout/src/liballoc/boxed.rs:658
             at /checkout/src/libstd/sys_common/thread.rs:21
             at /checkout/src/libstd/sys/unix/thread.rs:84
  11: <unknown>
