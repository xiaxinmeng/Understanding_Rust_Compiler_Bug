
error: internal compiler error: /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/common.rs:473: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@/tmp/bug.rs:12:23: 12:29] as std::ops::FnOnce<(<() as Fun<'a>>::Output,)>>), Binder(<[closure@/tmp/bug.rs:12:23: 12:29] as std::ops::FnOnce<(&str,)>>), Sorts(ExpectedFound { expected: &str, found: <() as Fun<'_>>::Output }))` selecting `Binder(<[closure@/tmp/bug.rs:12:23: 12:29] as std::ops::FnOnce<(&str,)>>)` during trans

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_errors/lib.rs:376
stack backtrace:
   1:     0x7f867e11e52c - std::sys::imp::backtrace::tracing::imp::write::h1d59ca58eb86a1e2
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f867e12cb1e - std::panicking::default_hook::{{closure}}::hc8550e2dc230bf9b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:351
   3:     0x7f867e12c6c3 - std::panicking::default_hook::he85ae9e5c9867198
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:361
   4:     0x7f867e12cfbb - std::panicking::rust_panic_with_hook::h319375f6b98710b0
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:555
   5:     0x7f867ca6fa78 - std::panicking::begin_panic::hcffa9fd2a04ec06f
   6:     0x7f867ca8d595 - rustc::session::opt_span_bug_fmt::{{closure}}::h614c6fe11c4465e9
   7:     0x7f867ca8d3aa - rustc::session::span_bug_fmt::h8b2f72c913194199
   8:     0x7f867cb02fa7 - rustc_trans::common::fulfill_obligation::{{closure}}::{{closure}}::h765609b5f897b16b
   9:     0x7f867caff22c - rustc_trans::common::fulfill_obligation::h5accac95e8a401b5
  10:     0x7f867cafb98b - rustc_trans::collector::do_static_dispatch::hce598ea3ab1ce3a5
  11:     0x7f867cafb00d - <rustc_trans::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_operand::hd92120f4e6379a7e
  12:     0x7f867cafb58a - <rustc_trans::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind::hccb2193f3818a450
  13:     0x7f867ca88286 - rustc::mir::visit::Visitor::visit_mir::hfb8959f7b6b5ba95
  14:     0x7f867cafd449 - rustc_trans::collector::collect_neighbours::he1db279e25d9c10d
  15:     0x7f867caf9522 - rustc_trans::collector::collect_items_rec::hbbb361a730e6b0d3
  16:     0x7f867caf9844 - rustc_trans::collector::collect_items_rec::hbbb361a730e6b0d3
  17:     0x7f867cae64d8 - rustc_trans::base::collect_and_partition_translation_items::{{closure}}::h742429e3883a12cb
  18:     0x7f867cae1bc2 - rustc_trans::base::collect_and_partition_translation_items::hac7e8bb398fe3219
  19:     0x7f867cad4e52 - rustc_trans::base::trans_crate::hf153b22edafcd3f0
  20:     0x7f867e4e0e80 - rustc_driver::driver::phase_4_translate_to_llvm::h6ba7de0cd156758c
  21:     0x7f867e4ad282 - rustc_driver::driver::compile_input::{{closure}}::h40e96e4888acf56e
  22:     0x7f867e4df551 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::hfacc2eff9ff79a87
  23:     0x7f867e4bd516 - rustc_driver::driver::phase_3_run_analysis_passes::h40916decd3c5ea98
  24:     0x7f867e4ab220 - rustc_driver::driver::compile_input::hce31fd38e3368b28
  25:     0x7f867e4f5ba4 - rustc_driver::run_compiler::hf9be2d0376d35b2a
  26:     0x7f867e401f1b - std::panicking::try::do_call::h996393a0ecaa9347
  27:     0x7f867e135dfa - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
  28:     0x7f867e42a292 - <F as alloc::boxed::FnBox<A>>::call_box::h70b5304598a75c7d
  29:     0x7f867e12b974 - std::sys::imp::thread::Thread::new::thread_start::hc16926852e47c008
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/liballoc/boxed.rs:623
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys/unix/thread.rs:84
  30:     0x7f8675f14453 - start_thread
  31:     0x7f867ddee7de - __GI___clone
  32:                0x0 - <unknown>
