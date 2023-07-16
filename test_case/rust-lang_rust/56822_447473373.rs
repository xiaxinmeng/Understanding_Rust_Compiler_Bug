rust
error: internal compiler error: unresolved inference variable in outlives: _#1t

thread '<unnamed>' panicked at 'encountered error with `-Z treat_err_as_bug', src/librustc_errors/lib.rs:500:13
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:210
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:224
   4: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:491
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::emit_db
   7: rustc_errors::Handler::emit
   8: rustc_errors::Handler::span_bug
   9: rustc_errors::Handler::delay_span_bug
  10: <rustc::infer::outlives::obligations::TypeOutlives<'cx, 'gcx, 'tcx, D>>::components_must_outlive
  11: <rustc::infer::outlives::obligations::TypeOutlives<'cx, 'gcx, 'tcx, D>>::type_must_outlive
  12: rustc::infer::outlives::obligations::<impl rustc::infer::InferCtxt<'cx, 'gcx, 'tcx>>::process_registered_region_obligations
  13: rustc::ty::context::tls::with_context::{{closure}}
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/traits/auto_trait.rs:240
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/infer/mod.rs:526
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:1635
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2003
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:1938
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2002
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:1634
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2103
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2087
  14: rustc::ty::context::GlobalCtxt::enter_local
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2078
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2087
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2098
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:1627
  15: rustc::traits::auto_trait::AutoTraitFinder::find_auto_trait_generics
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/infer/mod.rs:525
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/traits/auto_trait.rs:133
  16: rustdoc::clean::auto_trait::AutoTraitFinder::get_auto_trait_impl_for
             at src/librustdoc/clean/auto_trait.rs:200
             at src/librustdoc/clean/auto_trait.rs:123
  17: rustdoc::clean::auto_trait::AutoTraitFinder::get_auto_trait_impls
             at src/librustdoc/clean/auto_trait.rs:76
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/option.rs:632
             at src/librustdoc/clean/auto_trait.rs:73
  18: rustdoc::clean::def_ctor::get_def_from_node_id
             at src/librustdoc/clean/auto_trait.rs:41
             at src/librustdoc/clean/def_ctor.rs:59
  19: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at src/librustdoc/clean/auto_trait.rs:39
             at src/librustdoc/clean/mod.rs:3362
             at src/librustdoc/passes/collect_trait_impls.rs:170
  20: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at src/librustdoc/fold.rs:65
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/iter/mod.rs:1589
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1908
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1805
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1800
  21: rustdoc::fold::DocFolder::fold_inner_recur
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1700
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/iter/iterator.rs:1477
             at src/librustdoc/fold.rs:110
             at src/librustdoc/fold.rs:37
  22: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at src/librustdoc/fold.rs:100
             at src/librustdoc/passes/collect_trait_impls.rs:178
  23: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at src/librustdoc/fold.rs:65
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/iter/mod.rs:1589
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1908
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1805
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1800
  24: rustdoc::fold::DocFolder::fold_inner_recur
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1700
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/iter/iterator.rs:1477
             at src/librustdoc/fold.rs:110
             at src/librustdoc/fold.rs:37
  25: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at src/librustdoc/fold.rs:100
             at src/librustdoc/passes/collect_trait_impls.rs:178
  26: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at src/librustdoc/fold.rs:65
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/iter/mod.rs:1589
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1908
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1805
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1800
  27: rustdoc::fold::DocFolder::fold_inner_recur
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1700
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/iter/iterator.rs:1477
             at src/librustdoc/fold.rs:110
             at src/librustdoc/fold.rs:37
  28: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at src/librustdoc/fold.rs:100
             at src/librustdoc/passes/collect_trait_impls.rs:178
  29: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at src/librustdoc/fold.rs:65
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/iter/mod.rs:1589
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1908
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1805
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1800
  30: rustdoc::fold::DocFolder::fold_inner_recur
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/vec.rs:1700
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/iter/iterator.rs:1477
             at src/librustdoc/fold.rs:110
             at src/librustdoc/fold.rs:37
  31: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at src/librustdoc/fold.rs:100
             at src/librustdoc/passes/collect_trait_impls.rs:178
  32: rustdoc::passes::collect_trait_impls::collect_trait_impls
             at src/librustdoc/fold.rs:115
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/option.rs:632
             at src/librustdoc/fold.rs:115
             at src/librustdoc/passes/collect_trait_impls.rs:26
  33: rustdoc::core::run_core::{{closure}}::{{closure}}
             at src/librustdoc/core.rs:615
  34: rustc::ty::context::tls::enter_global
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc_driver/driver.rs:1353
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2035
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2003
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:1938
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2002
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2034
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:1992
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/thread/local.rs:309
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/thread/local.rs:255
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:1984
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/thread/local.rs:309
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/thread/local.rs:255
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:1976
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:2014
  35: rustc::ty::context::TyCtxt::create_and_enter
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc/ty/context.rs:1216
  36: rustc_driver::driver::phase_3_run_analysis_passes
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc_driver/driver.rs:1261
  37: <scoped_tls::ScopedKey<T>>::set
             at src/librustdoc/core.rs:497
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc_driver/driver.rs:76
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  38: rustdoc::core::run_core
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc_driver/driver.rs:75
             at src/librustdoc/core.rs:404
  39: <scoped_tls::ScopedKey<T>>::set
             at src/librustdoc/lib.rs:425
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libsyntax/lib.rs:123
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  40: syntax::with_globals
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libsyntax/lib.rs:122
  41: std::panicking::try::do_call
             at src/librustdoc/lib.rs:422
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc_driver/lib.rs:1642
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/panic.rs:319
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/panicking.rs:306
  42: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  43: rustc_driver::monitor
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/panicking.rs:285
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/panic.rs:398
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc_driver/lib.rs:1556
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc_driver/lib.rs:1567
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/librustc_driver/lib.rs:1641
  44: rustdoc::main_args
             at src/librustdoc/lib.rs:422
             at src/librustdoc/lib.rs:385
  45: <scoped_tls::ScopedKey<T>>::set
             at src/librustdoc/lib.rs:108
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libcore/option.rs:424
             at src/librustdoc/lib.rs:108
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libsyntax/lib.rs:123
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  46: syntax::with_globals
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libsyntax/lib.rs:122
  47: std::panicking::try::do_call
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/thread/mod.rs:479
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/panic.rs:319
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/panicking.rs:306
  48: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  49: <F as alloc::boxed::FnBox<A>>::call_box
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/panicking.rs:285
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/panic.rs:398
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/libstd/thread/mod.rs:478
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/boxed.rs:673
  50: std::sys::unix::thread::Thread::new::thread_start
             at /rustc/f4a421ee3cf1259f0750ac7fabd19da1d8551e4c/src/liballoc/boxed.rs:683
             at src/libstd/sys_common/thread.rs:24
             at src/libstd/sys/unix/thread.rs:91
  51: start_thread
  52: __clone

error: internal compiler error: unexpected panic

error: Unrecognized option: 'document-private-items'

error: Could not document `style`.
