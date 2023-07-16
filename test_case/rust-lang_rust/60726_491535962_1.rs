
 Documenting the_error v0.1.0 (/home/matias/Documents/eclipse_workspace/tmp/the_error)
thread 'rustc' panicked at 'Unable to fulfill trait DefId(2:1386 ~ core[4c06]::marker[0]::Send[0]) for 'IntoIter<T>': [FulfillmentError(Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { substs: [FooInterface<T>], item_def_id: DefId(0:18 ~ the_error[8787]::InterfaceType[0]::Send[0]) }, True)),depth=2),MismatchedProjectionTypes(Sorts(ExpectedFound { expected: False, found: True })))]', src/librustc/traits/auto_trait.rs:202:17
stack backtrace:
   0: 0xb5cf6d4c - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h88d972f6f6e46c24
                       at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: 0xb5cef0fb - std::sys_common::backtrace::_print::h83aca1574431bbad
                       at src/libstd/sys_common/backtrace.rs:71
   2: 0xb5cf31aa - std::panicking::default_hook::{{closure}}::h54e8f04261c4c0b1
                       at src/libstd/sys_common/backtrace.rs:59
                       at src/libstd/panicking.rs:197
   3: 0xb5cf2ef4 - std::panicking::default_hook::h66422ac1b55ce676
                       at src/libstd/panicking.rs:211
   4: 0xb5cf3936 - std::panicking::rust_panic_with_hook::hbf2738690e9f5989
                       at src/libstd/panicking.rs:474
   5: 0xb5cf3476 - std::panicking::continue_panic_fmt::hcbef7b43feb6b8d9
                       at src/libstd/panicking.rs:381
   6: 0xb5cf33b9 - std::panicking::begin_panic_fmt::hfafa0a0090d76a4e
                       at src/libstd/panicking.rs:336
   7:   0x5ad31f - rustc::traits::auto_trait::AutoTraitFinder::find_auto_trait_generics::{{closure}}::{{closure}}::h3076052a80a2b2be
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e//<::std::macros::panic macros>:8
   8:   0x5acd23 - rustc::ty::context::tls::with_context::{{closure}}::h68c2c2610d54ace6
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/result.rs:766
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/traits/auto_trait.rs:201
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/infer/mod.rs:529
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1726
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1966
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1899
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1965
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1725
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:2071
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:2055
   9:   0x5ab8d2 - rustc::ty::context::GlobalCtxt::enter_local::h90af253d10758aa1
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:2045
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:2055
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:2067
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1717
  10:   0x5cd3f8 - rustc::traits::auto_trait::AutoTraitFinder::find_auto_trait_generics::h42ee2645b594cb59
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/infer/mod.rs:528
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/traits/auto_trait.rs:119
  11:   0x43d725 - core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut::h25ad1904ab0b7bc3
                       at src/librustdoc/clean/auto_trait.rs:50
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/adapters/mod.rs:826
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/ops/function.rs:269
  12:   0x6b213a - <core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold::hb3e3d5b042f42891
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/traits/iterator.rs:1572
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/adapters/chain.rs:88
  13:   0x4c5cb1 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter::ha8605cbc824c8642
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/adapters/mod.rs:826
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/traits/iterator.rs:1609
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/adapters/mod.rs:812
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/liballoc/vec.rs:1809
  14:   0x690bdd - rustdoc::clean::get_auto_trait_and_blanket_impls::h6b2119eea5d33f2b
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/liballoc/vec.rs:1721
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/traits/iterator.rs:1465
                       at src/librustdoc/clean/auto_trait.rs:33
                       at src/librustdoc/clean/mod.rs:3787
  15:   0x6cc5b6 - <rustdoc::passes::collect_trait_impls::SyntheticImplCollector as rustdoc::fold::DocFolder>::fold_item::h01bd6abc37807d9a
                       at src/librustdoc/passes/collect_trait_impls.rs:158
  16:   0x4c2448 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter::hf3f69d672ddf9615
                       at src/librustdoc/fold.rs:100
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/adapters/mod.rs:826
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/traits/iterator.rs:1572
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/adapters/mod.rs:826
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/traits/iterator.rs:1609
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/adapters/mod.rs:812
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/liballoc/vec.rs:1929
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/liballoc/vec.rs:1826
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/liballoc/vec.rs:1821
  17:   0x6c909e - rustdoc::fold::DocFolder::fold_inner_recur::h3b90485174c31fb1
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/liballoc/vec.rs:1721
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/iter/traits/iterator.rs:1465
                       at src/librustdoc/fold.rs:100
                       at src/librustdoc/fold.rs:27
  18:   0x6cc8ad - <rustdoc::passes::collect_trait_impls::SyntheticImplCollector as rustdoc::fold::DocFolder>::fold_item::h01bd6abc37807d9a
                       at src/librustdoc/fold.rs:90
                       at src/librustdoc/passes/collect_trait_impls.rs:166
  19:   0x6c9f8d - rustdoc::passes::collect_trait_impls::collect_trait_impls::h7ddbb0578e71ca0e
                       at src/librustdoc/fold.rs:105
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/option.rs:624
                       at src/librustdoc/fold.rs:105
                       at src/librustdoc/passes/collect_trait_impls.rs:17
  20:   0x5cb622 - rustc::ty::context::tls::enter_global::h0352a121287a69cf
                       at src/librustdoc/core.rs:465
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_interface/passes.rs:807
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1999
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1966
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1899
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1965
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1998
  21:   0x5fc1d6 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h37af13ddc5734fca
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_interface/passes.rs:807
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e//<::rustc_data_structures::box_region::declare_box_region_type macros>:17
  22: 0xb7c1c368 - rustc_interface::passes::create_global_ctxt::{{closure}}::ha0f95b1f2df7a5e5
  23:   0x5fd24a - rustc_interface::interface::run_compiler_in_existing_thread_pool::hb0e8f2e4ea5e07c3
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/liballoc/boxed.rs:910
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_data_structures/box_region.rs:52
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e//<::rustc_data_structures::box_region::declare_box_region_type macros>:19
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_interface/passes.rs:807
                       at src/librustdoc/core.rs:354
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_interface/interface.rs:122
  24:   0x70a3f7 - rustdoc::core::run_core::h828402703fcc17cc
                       at src/librustdoc/core.rs:340
  25:   0x5aeb2e - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h7b9fd07b2b1ea2c8
                       at src/librustdoc/lib.rs:450
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/panic.rs:309
  26:   0x4f3815 - std::panicking::try::do_call::hb936f360df81fcbb
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/panicking.rs:293
  27: 0xb5d03447 - __rust_maybe_catch_panic
                       at src/libpanic_unwind/lib.rs:85
  28:   0x6d02cc - rustc_driver::report_ices_to_stderr_if_any::h7f9579b20d11e238
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/panicking.rs:272
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/panic.rs:388
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_driver/lib.rs:1114
  29:   0x481438 - rustdoc::main_args::ha7dd5c6ee7b5b640
                       at src/librustdoc/lib.rs:447
                       at src/librustdoc/lib.rs:404
  30:   0x6abdda - std::thread::local::LocalKey<T>::with::ha9c7da692a148e8b
                       at src/librustdoc/lib.rs:98
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/option.rs:416
                       at src/librustdoc/lib.rs:98
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_interface/util.rs:186
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1954
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/thread/local.rs:299
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/thread/local.rs:245
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1946
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/thread/local.rs:299
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/thread/local.rs:245
  31:   0x600c42 - std::sys_common::backtrace::__rust_begin_short_backtrace::ha4de48f9125e55bb
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc/ty/context.rs:1938
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_interface/util.rs:186
                       at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_interface/util.rs:182
                       at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libsyntax/lib.rs:101
                       at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libsyntax/lib.rs:100
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_interface/util.rs:181
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/librustc_interface/util.rs:159
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/sys_common/backtrace.rs:136
  32:   0x4f3980 - std::panicking::try::do_call::hcb704b420ba15158
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/thread/mod.rs:469
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/panic.rs:309
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/panicking.rs:293
  33: 0xb5d03447 - __rust_maybe_catch_panic
                       at src/libpanic_unwind/lib.rs:85
  34:   0x6d7f8b - core::ops::function::FnOnce::call_once{{vtable.shim}}::h71980ce26ab01d54
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/panicking.rs:272
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/panic.rs:388
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libstd/thread/mod.rs:468
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/libcore/ops/function.rs:231
  35: 0xb5cd6208 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h2e642aeea9c9581f
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/liballoc/boxed.rs:704
  36: 0xb5d01e50 - std::sys::unix::thread::Thread::new::thread_start::ha576c140cc0cd745
                       at /rustc/d595b113584f8f446957469951fd5d31adc2a44e/src/liballoc/boxed.rs:704
                       at src/libstd/sys_common/thread.rs:13
                       at src/libstd/sys/unix/thread.rs:79
  37: 0xb5c4f294 - start_thread
  38: 0xb5b5c0ad - __clone
  39:        0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0-nightly (d595b1135 2019-05-10) running on i686-unknown-linux-gnu

