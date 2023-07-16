
error: internal compiler error: ../src/librustc/infer/mod.rs:624: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<() as std::convert::From<i32>>)),depth=1),Unimplemented)]` resolving bounds after type-checking

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:383
stack backtrace:
   1:     0x7f91a71b237a - std::sys::imp::backtrace::tracing::imp::write::h944c02ac40aee2d7
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f91a71c122f - std::panicking::default_hook::{{closure}}::h6875a2976258b020
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:247
   3:     0x7f91a71c0dcd - std::panicking::default_hook::h88ffbc5922643264
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:257
   4:     0x7f91a71c16d7 - std::panicking::rust_panic_with_hook::ha5aed1dfc0e220e3
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:451
   5:     0x7f91a67fae4b - std::panicking::begin_panic::h264cdc75d51b518b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:413
   6:     0x7f91a6817d93 - rustc::session::opt_span_bug_fmt::{{closure}}::h4a9b70c3df8b4b3a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/<panic macros>:3
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/session/mod.rs:778
   7:     0x7f91a6817bb6 - rustc::session::span_bug_fmt::h0c5d2225aa96fe42
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1046
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1035
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/local.rs:245
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1031
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1046
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/session/mod.rs:775
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/session/mod.rs:768
   8:     0x7f91a688fc77 - rustc_trans::common::fulfill_obligation::{{closure}}::{{closure}}::hf13a4d09f7cbb18e
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/macros.rs:59
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/common.rs:989
   9:     0x7f91a688c0dc - rustc_trans::common::fulfill_obligation::h6bfd7b7f15728b60
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/infer/mod.rs:440
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1019
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/local.rs:245
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1016
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:853
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/infer/mod.rs:440
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/common.rs:949
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/dep_graph/dep_tracking_map.rs:145
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/common.rs:943
  10:     0x7f91a6887278 - rustc_trans::collector::do_static_dispatch::h9456c98c4201c352
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/collector.rs:864
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/collector.rs:829
  11:     0x7f91a68868aa - <rustc_trans::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_operand::h7428e8287ff72240
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/collector.rs:575
  12:     0x7f91a6886fe6 - <rustc_trans::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind::h8233aed64e4688ea
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/mir/visit.rs:424
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/collector.rs:658
  13:     0x7f91a6813387 - rustc::mir::visit::Visitor::visit_mir::hc23b408d887d8c09
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/mir/visit.rs:354
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/mir/visit.rs:123
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/mir/visit.rs:290
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/mir/visit.rs:96
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/mir/visit.rs:256
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/mir/visit.rs:90
  14:     0x7f91a688524c - rustc_trans::collector::collect_items_rec::h9535e33b3068eb29
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/collector.rs:1223
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/collector.rs:376
  15:     0x7f91a68858ff - rustc_trans::collector::collect_items_rec::h9535e33b3068eb29
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/collector.rs:383
  16:     0x7f91a686f200 - rustc_trans::base::collect_and_partition_translation_items::{{closure}}::h9f2b052ae16b5f0b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/collector.rs:285
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/dep_graph/graph.rs:70
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/collector.rs:276
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/base.rs:2023
  17:     0x7f91a686b59f - rustc_trans::base::collect_and_partition_translation_items::h6a325f48a6efa8c7
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/util/common.rs:34
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/base.rs:2022
  18:     0x7f91a685ddca - rustc_trans::base::trans_crate::h6e0b0bf1b66798ce
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_trans/base.rs:1572
  19:     0x7f91a7571401 - rustc_driver::driver::phase_4_translate_to_llvm::h48ed91c172294403
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:1041
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/util/common.rs:34
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:1039
  20:     0x7f91a7541c36 - rustc_driver::driver::compile_input::{{closure}}::hf13172fc4e5a2f4d
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:207
  21:     0x7f91a755f503 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h97a3a12d948df547
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:995
  22:     0x7f91a7558833 - rustc_driver::driver::phase_3_run_analysis_passes::hb0ad9de18d423e67
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1019
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/local.rs:245
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1016
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1003
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/local.rs:245
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1000
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:789
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:870
  23:     0x7f91a753fb77 - rustc_driver::driver::compile_input::h8e119234b60571d5
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:174
  24:     0x7f91a7585cd8 - rustc_driver::run_compiler::hbdfc4f84e2e0f4b9
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:221
  25:     0x7f91a74a1158 - std::panicking::try::do_call::hf679f17bf3b43b0b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:1116
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:137
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:1050
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panic.rs:295
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:356
  26:     0x7f91a71cbc1a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libpanic_unwind/lib.rs:97
  27:     0x7f91a74c31b8 - <F as alloc::boxed::FnBox<A>>::call_box::h506fb5d7b8891cd4
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:332
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panic.rs:351
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/mod.rs:287
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/liballoc/boxed.rs:595
  28:     0x7f91a71c0094 - std::sys::imp::thread::Thread::new::thread_start::h8084b1107992ae5b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/liballoc/boxed.rs:605
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys/unix/thread.rs:84
  29:     0x7f919f3f5709 - start_thread
  30:     0x7f91a6e7a82c - clone
  31:                0x0 - <unknown>
