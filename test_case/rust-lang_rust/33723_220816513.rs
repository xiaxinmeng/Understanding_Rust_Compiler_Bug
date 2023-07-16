
error: internal compiler error: src/librustc/infer/mod.rs:642: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<toxcore::dht::PackedNode as std::cmp::PartialOrd>)),depth=1),Unimplemented)]` resolving bounds after type-checking
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', src/libsyntax/errors/mod.rs:545
stack backtrace:
   1:     0x7f3cbb5d17f4 - std::sys::backtrace::tracing::imp::write::h82353b94d9957b5a
                        at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:41
   2:     0x7f3cbb607fd5 - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hd08c9fd28a21a931
                        at src/libstd/panicking.rs:210
   3:     0x7f3cbb607583 - std::panicking::default_hook::h9f5ce461fe95b992
                        at src/libstd/panicking.rs:219
   4:     0x7f3cbb54b914 - std::panicking::rust_panic_with_hook::hfb9137c9fa89bbb0
                        at src/libstd/panicking.rs:369
   5:     0x7f3cb7ed03b6 - std::panicking::begin_panic::hfa2b023a2705aff0
                        at src/libstd/panicking.rs:327
   6:     0x7f3cb7ed0340 - syntax::errors::Handler::span_bug::hd5565a44979bfb51
                        at /run/media/xenon/arkiv0/kodez/rust/<std macros>:3
   7:     0x7f3cb7ed01f4 - rustc::session::opt_span_bug_fmt::_$u7b$$u7b$closure$u7d$$u7d$::hdce47981d13cfedb
   8:     0x7f3cb7ecfcba - rustc::ty::context::tls::with_opt::_$u7b$$u7b$closure$u7d$$u7d$::h7689487aa4411aa9
   9:     0x7f3cb7ecfb99 - rustc::ty::context::tls::with::_$u7b$$u7b$closure$u7d$$u7d$::h2673b13fde52bc54
  10:     0x7f3cb7ecfaa3 - _<std..thread..LocalKey<T>>::with::hb12eaca4e42bb0cf
                        at src/libstd/thread/local.rs:211
  11:     0x7f3cb7ecf965 - rustc::ty::context::tls::with::h54a0ff11b76451e9
                        at src/librustc/ty/context.rs:918
  12:     0x7f3cb7ecf34d - rustc::ty::context::tls::with_opt::h41d702407e5f0624
                        at src/librustc/ty/context.rs:933
  13:     0x7f3cb7ecf27f - rustc::session::opt_span_bug_fmt::h50c3c22cda036de1
                        at src/librustc/session/mod.rs:634
  14:     0x7f3cb7ecf1b8 - rustc::session::span_bug_fmt::h802dfa3bf4574d12
                        at src/librustc/session/mod.rs:627
  15:     0x7f3cb80d1fca - rustc::infer::InferCtxt::drain_fulfillment_cx_or_panic::hd9729be1aa461bc4
                        at src/librustc/macros.rs:59
  16:     0x7f3cb80cb9c8 - rustc_trans::common::fulfill_obligation::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h38f536cd68233fd5
                        at src/librustc_trans/common.rs:1106
  17:     0x7f3cb80cb025 - rustc::infer::InferCtxtBuilder::enter::_$u7b$$u7b$closure$u7d$$u7d$::h89e7632291ca064a
  18:     0x7f3cb80caba4 - rustc::ty::context::tls::enter::_$u7b$$u7b$closure$u7d$$u7d$::h7b0ebd8b4f199416
  19:     0x7f3cb80ca9bf - _<std..thread..LocalKey<T>>::with::h1b4c6ce8dfe9b82b
                        at src/libstd/thread/local.rs:211
  20:     0x7f3cb80ca77d - rustc::ty::context::tls::enter::h4b21adc0faef8b81
                        at src/librustc/ty/context.rs:903
  21:     0x7f3cb80ca5ac - rustc::ty::context::GlobalCtxt::enter_local::hc12392e9d457337f
                        at src/librustc/ty/context.rs:747
  22:     0x7f3cb80ca28f - rustc::infer::InferCtxtBuilder::enter::h71d9437808bdb1e4
                        at src/librustc/infer/mod.rs:504
  23:     0x7f3cb80c544c - rustc_trans::common::fulfill_obligation::_$u7b$$u7b$closure$u7d$$u7d$::h8e7825cc08816d91
                        at src/librustc_trans/common.rs:1069
  24:     0x7f3cb80c1b79 - rustc::dep_graph::dep_tracking_map::_<impl rustc..util..common..MemoizationMap for std..cell..RefCell<rustc..dep_graph..DepTrackingMap<M>>>::memoize::hf7814e6c99519092
                        at src/librustc/dep_graph/dep_tracking_map.rs:124
  25:     0x7f3cb7efb484 - rustc_trans::common::fulfill_obligation::ha2cfa626ef3e06cf
                        at src/librustc_trans/common.rs:1063
  26:     0x7f3cb808e96a - rustc_trans::collector::do_static_trait_method_dispatch::hf8c3b0cd9cb60938
                        at src/librustc_trans/collector.rs:862
  27:     0x7f3cb808b862 - rustc_trans::collector::do_static_dispatch::hebd0040db731af8f
                        at src/librustc_trans/collector.rs:816
  28:     0x7f3cb8089f86 - _<collector..MirNeighborCollector<'a, 'tcx> as rustc..mir..visit..Visitor<'tcx>>::visit_operand::hea412d96a6bc35e8
                        at src/librustc_trans/collector.rs:555
  29:     0x7f3cb808c87d - rustc::mir::visit::Visitor::super_terminator_kind::h433fe2dbf56da278
                        at src/librustc/mir/visit.rs:409
  30:     0x7f3cb807c481 - _<collector..MirNeighborCollector<'a, 'tcx> as rustc..mir..visit..Visitor<'tcx>>::visit_terminator_kind::hffa9cf2349bca7cb
                        at src/librustc_trans/collector.rs:638
  31:     0x7f3cb807c119 - rustc::mir::visit::Visitor::super_terminator::h991cf31a745841e9
                        at src/librustc/mir/visit.rs:351
  32:     0x7f3cb807c0b8 - rustc::mir::visit::Visitor::visit_terminator::h7a99b1523723a10a
                        at src/librustc/mir/visit.rs:121
  33:     0x7f3cb807af16 - rustc::mir::visit::Visitor::super_basic_block_data::hf4c044472f884c03
                        at src/librustc/mir/visit.rs:296
  34:     0x7f3cb807add8 - rustc::mir::visit::Visitor::visit_basic_block_data::hd4841841ec751044
                        at src/librustc/mir/visit.rs:97
  35:     0x7f3cb807a525 - rustc::mir::visit::Visitor::super_mir::hdc0b160692fdc967
                        at src/librustc/mir/visit.rs:258
  36:     0x7f3cb807a2fc - rustc::mir::visit::Visitor::visit_mir::h369013c3c0aae0b6
                        at src/librustc/mir/visit.rs:91
  37:     0x7f3cb8076ae5 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:382
  38:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  39:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  40:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  41:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  42:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  43:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  44:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  45:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  46:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  47:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  48:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  49:     0x7f3cb8076de7 - rustc_trans::collector::collect_items_rec::h71c8aaf61ac7337a
                        at src/librustc_trans/collector.rs:392
  50:     0x7f3cb8074cce - rustc_trans::collector::collect_crate_translation_items::_$u7b$$u7b$closure$u7d$$u7d$::hcb88d08a1e091e91
                        at src/librustc_trans/collector.rs:285
  51:     0x7f3cb80747c2 - rustc::dep_graph::graph::DepGraph::with_ignore::he0f3197641caf72c
                        at src/librustc/dep_graph/graph.rs:54
  52:     0x7f3cb7fcf095 - rustc_trans::collector::collect_crate_translation_items::h07afceaa75719f40
                        at src/librustc_trans/collector.rs:276
  53:     0x7f3cb7fcd737 - rustc_trans::base::collect_and_partition_translation_items::_$u7b$$u7b$closure$u7d$$u7d$::h1b40dc07c5c84201
                        at src/librustc_trans/base.rs:2958
  54:     0x7f3cb7fccfb5 - rustc::util::common::time::h0b94b110eb16932e
                        at src/librustc/util/common.rs:38
  55:     0x7f3cb7f9021b - rustc_trans::base::collect_and_partition_translation_items::h682f54dfd52ee7ad
                        at src/librustc_trans/base.rs:2957
  56:     0x7f3cb7f8a662 - rustc_trans::base::trans_crate::h0d2f52cf9150d332
                        at src/librustc_trans/base.rs:2736
  57:     0x7f3cbbeaa95c - rustc_driver::driver::phase_4_translate_to_llvm::_$u7b$$u7b$closure$u7d$$u7d$::h8418984c0bc44f78
                        at src/librustc_driver/driver.rs:1045
  58:     0x7f3cbbeaa0bd - rustc::util::common::time::h156771d47d5299d9
                        at src/librustc/util/common.rs:38
  59:     0x7f3cbbc999bb - rustc_driver::driver::phase_4_translate_to_llvm::h5a4797181b85c786
                        at src/librustc_driver/driver.rs:1043
  60:     0x7f3cbbc89541 - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::hd93daa5abe26f190
                        at src/librustc_driver/driver.rs:251
  61:     0x7f3cbbc5f532 - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h2cec98064a41824e
                        at src/librustc_driver/driver.rs:1016
  62:     0x7f3cbbc5e1c5 - rustc::ty::context::tls::enter::_$u7b$$u7b$closure$u7d$$u7d$::h6fce914bb21dee1e
  63:     0x7f3cbbc5dfcf - _<std..thread..LocalKey<T>>::with::hdc9917441d7db9f6
                        at src/libstd/thread/local.rs:211
  64:     0x7f3cbbc5dd80 - rustc::ty::context::tls::enter::h93b004407fbb986a
                        at src/librustc/ty/context.rs:903
  65:     0x7f3cbbc5d973 - rustc::ty::context::tls::enter_global::_$u7b$$u7b$closure$u7d$$u7d$::hb42c74a37f4ac28c
  66:     0x7f3cbbc5d7d5 - _<std..thread..LocalKey<T>>::with::h8a262921ee70aec1
                        at src/libstd/thread/local.rs:211
  67:     0x7f3cbbc5d57b - rustc::ty::context::tls::enter_global::he4f96cc154feef14
                        at src/librustc/ty/context.rs:887
  68:     0x7f3cbbc56bd2 - rustc::ty::context::TyCtxt::create_and_enter::h093266a1f6aebc90
                        at src/librustc/ty/context.rs:685
  69:     0x7f3cbbc4e09a - rustc_driver::driver::phase_3_run_analysis_passes::h6a2fb4c38a1b9849
                        at src/librustc_driver/driver.rs:900
  70:     0x7f3cbbbdd699 - rustc_driver::driver::compile_input::hcb4c0b80738a045b
                        at src/librustc_driver/driver.rs:217
  71:     0x7f3cbbbd15c8 - rustc_driver::run_compiler_with_file_loader::hfc6bc9f2d223895f
                        at src/librustc_driver/lib.rs:217
  72:     0x7f3cbbbb3738 - rustc_driver::run_compiler::ha9dd39a77b6f9883
                        at src/librustc_driver/lib.rs:154
  73:     0x7f3cbbbae918 - rustc_driver::run::_$u7b$$u7b$closure$u7d$$u7d$::h19ce45a29dafca86
                        at src/librustc_driver/lib.rs:132
  74:     0x7f3cbbbac712 - rustc_driver::monitor::_$u7b$$u7b$closure$u7d$$u7d$::h82ece5bf391bc5fd
                        at src/librustc_driver/lib.rs:1044
  75:     0x7f3cbbbac5e9 - _<std..panic..AssertUnwindSafe<F> as std..ops..FnOnce<()>>::call_once::h1c0c98080f4fb76f
                        at src/libstd/panic.rs:284
  76:     0x7f3cbbbac585 - std::panicking::try::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hecc2784e4826f882
  77:     0x7f3cbbbacb04 - std::panicking::try::call::hee2e41b19f6057ba
                        at src/libstd/panicking.rs:271
  78:     0x7f3cbb646997 - __rust_try
  79:     0x7f3cbb646924 - __rust_maybe_catch_panic
                        at src/libpanic_unwind/lib.rs:91
  80:     0x7f3cbbbac21d - std::panicking::try::_$u7b$$u7b$closure$u7d$$u7d$::hc325053e2cf446d3
  81:     0x7f3cbbbac0ef - _<std..thread..LocalKey<T>>::with::h228035e732e3cc97
                        at src/libstd/thread/local.rs:211
  82:     0x7f3cbbbabf67 - std::panicking::try::h701b93ee68739e1e
                        at src/libstd/panicking.rs:234
  83:     0x7f3cbbbabea0 - std::panic::catch_unwind::h84c941df2d3b2678
                        at src/libstd/panic.rs:387
  84:     0x7f3cbbbabd53 - std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::h2ff690d78f0e2740
  85:     0x7f3cbbbace4c - _<F as std..boxed..FnBox<A>>::call_box::h2a211d0ec3984fcf
                        at src/liballoc/boxed.rs:543
  86:     0x7f3cbb5b8c1b - _<Box<alloc..boxed..FnBox<A, Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..FnOnce$LT$A$GT$$GT$::call_once::hcf14a84f5e7594ee
                        at src/liballoc/boxed.rs:553
  87:     0x7f3cbb5c60de - std::sys_common::thread::start_thread::h18419c8184cd096f
                        at src/libstd/sys/common/thread.rs:23
  88:     0x7f3cbb603384 - std::sys::thread::Thread::new::thread_start::h99197ac7442ad7fa
                        at src/libstd/sys/unix/thread.rs:74
  89:     0x7f3cb13e5433 - start_thread
  90:     0x7f3cbb1be28c - clone
  91:                0x0 - <unknown>
