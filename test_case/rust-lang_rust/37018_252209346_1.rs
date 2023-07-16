
error: internal compiler error: ../src/librustc_passes/consts.rs:483: no kind for cast
 --> regice.rs:7:39
  |
7 | pub static BAR: *const libc::c_void = baz as *const libc::c_void;
  |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:602
stack backtrace:
   1:        0x111422288 - std::sys::backtrace::tracing::imp::write::h15f4dcb9fc5cc76b
   2:        0x11142f9ff - std::panicking::default_hook::{{closure}}::h122a1c9dfacef96f
   3:        0x11142cefd - std::panicking::default_hook::h9cf83dcedc626223
   4:        0x11142d596 - std::panicking::rust_panic_with_hook::hc99436623a03228e
   5:        0x10e2cbefb - std::panicking::begin_panic::h17785a79ba05422e
   6:        0x10e2e8473 - rustc::session::opt_span_bug_fmt::{{closure}}::h213520e1e2445968
   7:        0x10e2d7789 - rustc::session::span_bug_fmt::h887f1af1528af01b
   8:        0x10e2dfff6 - <rustc_passes::consts::CheckCrateVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'v>>::visit_expr::h0be89430a25e80da
   9:        0x10e2ea346 - rustc_passes::consts::CheckCrateVisitor::global_expr::{{closure}}::h76e0a472abd5b020
  10:        0x10e2dde77 - rustc_passes::consts::CheckCrateVisitor::global_expr::h170e19160d2d6eb6
  11:        0x10e2e0f61 - rustc_passes::consts::check_crate::h91cf5a98b4344a74
  12:        0x10d87c814 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h5dabeea7050cf7c2
  13:        0x10d8530b6 - rustc_driver::driver::phase_3_run_analysis_passes::h79f20d1127b1dfba
  14:        0x10d840eca - rustc_driver::driver::compile_input::h8ef818979445c42c
  15:        0x10d868fa8 - rustc_driver::run_compiler::ha981cef7568b5eb8
  16:        0x10d7a9700 - std::panicking::try::do_call::h4a286c81408a0d92
  17:        0x11142ffba - __rust_maybe_catch_panic
  18:        0x10d7c91c4 - <F as alloc::boxed::FnBox<A>>::call_box::h5de1378b9690a33a
  19:        0x11142c1b4 - std::sys::thread::Thread::new::thread_start::h2eacc7b7061d5d8c
  20:     0x7fff9cd4b99c - _pthread_body
  21:     0x7fff9cd4b919 - _pthread_start
