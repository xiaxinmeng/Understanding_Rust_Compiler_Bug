
$ RUST_BACKTRACE=full cargo +nightly test
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.20.0-nightly (598eddf4f 2017-07-24) running on x86_64-apple-darwin

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'assertion failed: self.hashes.insert(k, v).is_none()', src/librustc_incremental/calculate_svh/mod.rs:70:8
stack backtrace:
   0:        0x113179153 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::h1c846d30ab9d2835
   1:        0x113186aaa - std::panicking::default_hook::{{closure}}::h5686ff0b2c6e4eb9
   2:        0x11318659b - std::panicking::default_hook::h9f2e51b0fbc325af
   3:        0x113188d82 - std::panicking::rust_panic_with_hook::hd0e3deaa5a3f3a11
   4:        0x1119ee9f7 - std::panicking::begin_panic_new::h9ddc3149addcd00d
   5:        0x111a2de2a - rustc_incremental::calculate_svh::IncrementalHashesMap::insert::h74e01598230aeac9
   6:        0x111a30280 - rustc_incremental::calculate_svh::ComputeItemHashesVisitor::compute_and_store_ich_for_item_like::hf675144d47908776
   7:        0x111a304be - <rustc_incremental::calculate_svh::ComputeItemHashesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item::h742bab9c5eaebc0f
   8:        0x111a30cce - rustc_incremental::calculate_svh::compute_incremental_hashes_map::h73e253fb72747d3c
   9:        0x10d7695bf - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h66b2b9a83b85e06c
  10:        0x10d759482 - rustc_driver::driver::phase_3_run_analysis_passes::hc18545f9a6fc068c
  11:        0x10d73fca5 - rustc_driver::driver::compile_input::haa173b6c5d780534
  12:        0x10d7854e0 - rustc_driver::run_compiler::hbd8c2d0cc1d7195f
  13:        0x10d6a2739 - std::sys_common::backtrace::__rust_begin_short_backtrace::ha89ce0330978fd9d
  14:        0x1131b8dcc - __rust_maybe_catch_panic
  15:        0x10d6db3ff - <F as alloc::boxed::FnBox<A>>::call_box::hd33b69fc474605d5
  16:        0x113185a4b - std::sys::imp::thread::Thread::new::thread_start::h657456c8b9cbda9f
  17:     0x7fff8954d99c - _pthread_body
  18:     0x7fff8954d919 - _pthread_start
