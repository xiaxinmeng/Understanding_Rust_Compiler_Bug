
$ RUST_BACKTRACE=full rustc src/lib.rs --crate-type=lib
error: internal compiler error: /checkout/src/librustc_typeck/check/mod.rs:1710: escaping regions in predicate Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { trait_ref: <_ as std::iter::Iterator>, item_name: Item(88) }, &u32)),depth=0)
 --> src/lib.rs:2:33
  |
2 | fn will_ice(something: &u32) -> impl Iterator<Item = &u32> {
  |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:377
stack backtrace:
   0:     0x7fd7beb20173 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::h0c49f46a3545f908
                               at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7fd7beb1a8a4 - std::sys_common::backtrace::_print::hcef39a9816714c4c
                               at /checkout/src/libstd/sys_common/backtrace.rs:71
   2:     0x7fd7beb2e227 - std::panicking::default_hook::{{closure}}::h7c3c94835e02f846
                               at /checkout/src/libstd/sys_common/backtrace.rs:60
                               at /checkout/src/libstd/panicking.rs:355
   3:     0x7fd7beb2dd4b - std::panicking::default_hook::h0bf7bc3112fb107d
                               at /checkout/src/libstd/panicking.rs:365
   4:     0x7fd7beb2e6fb - std::panicking::rust_panic_with_hook::ha27630c950090fec
                               at /checkout/src/libstd/panicking.rs:549
   5:     0x7fd7bd42ebb8 - std::panicking::begin_panic::h63c8743b74308765
   6:     0x7fd7bd455325 - rustc::session::opt_span_bug_fmt::{{closure}}::h0399ec9f6fdc80f4
   7:     0x7fd7bd45513a - rustc::session::span_bug_fmt::h91d1439893b8a5d4
   8:     0x7fd7bd4e1f1c - rustc_typeck::check::FnCtxt::register_predicate::he6d13886b1ff3937
   9:     0x7fd7bd42a5a8 - <rustc::ty::fold::BottomUpFolder<'a, 'gcx, 'tcx, F> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h16e93d791bf416ce
  10:     0x7fd7bd4d883e - rustc_typeck::check::check_fn::hc87bd2602316e434
  11:     0x7fd7bd4d73f6 - rustc_typeck::check::typeck_tables::h9b4c3ea40474b1b2
  12:     0x7fd7bbbfb30c - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables<'tcx>>::try_get::h50e18962037fb36f
  13:     0x7fd7bbbfb4d1 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables<'tcx>>::get::h0b200a289899fb66
  14:     0x7fd7bbbb1062 - rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::item_tables::ha0531bf6c24dcec8
  15:     0x7fd7bd4d6003 - rustc_typeck::check::check_item_bodies::h4f3e40f875ac7e76
  16:     0x7fd7bd5429e2 - rustc_typeck::check_crate::h6ea76d8b10d9239d
  17:     0x7fd7bef10fd0 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h45ea055410817656
  18:     0x7fd7bee55bf9 - rustc::ty::context::TyCtxt::create_and_enter::hb63d87ce6018d148
  19:     0x7fd7bef02bfd - rustc_driver::driver::phase_3_run_analysis_passes::h303bc0d75d32173b
  20:     0x7fd7beedc275 - rustc_driver::driver::compile_input::h446a5d2d354ecae7
  21:     0x7fd7bef3c046 - rustc_driver::run_compiler::h427b60825e751999
  22:     0x7fd7bee021ab - std::panicking::try::do_call::hc1fa4f624bc07e15
  23:     0x7fd7beb375ea - __rust_maybe_catch_panic
                               at /checkout/src/libpanic_unwind/lib.rs:98
  24:     0x7fd7bee49a31 - <F as alloc::boxed::FnBox<A>>::call_box::h69b187d2a58a3ac6
  25:     0x7fd7beb2cfa4 - std::sys::imp::thread::Thread::new::thread_start::h75b208405df6dcf1
                               at /checkout/src/liballoc/boxed.rs:650
                               at /checkout/src/libstd/sys_common/thread.rs:21
                               at /checkout/src/libstd/sys/unix/thread.rs:84
  26:     0x7fd7b6abf6c9 - start_thread
  27:     0x7fd7be7df0ae - __clone
  28:                0x0 - <unknown>
