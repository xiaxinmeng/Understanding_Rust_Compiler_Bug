 rust
unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'assertion failed: !ty::type_needs_infer(ty)', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/lib.rs:147

stack backtrace:
   1: 0xb732a077 - sys::backtrace::write::h6952996a21c47c07ads
   2: 0xb7333434 - panicking::on_panic::hd35d988ee3a5c2f1ZXw
   3: 0xb72ef773 - rt::unwind::begin_unwind_inner::he8bdefc833f13e3eJDw
   4: 0xb6cfbdc2 - rt::unwind::begin_unwind::h16236413390050183289
   5: 0xb6e1f99f - collect::convert_item::h9d94d3548d25d26ffSx
   6: 0xb6e1c7ef - collect::collect_item_types::h4bbb002ff41bd8d7cZw
   7: 0xb6e70912 - check_crate::h049a1ef5eff22056w8C
   8: 0xb7679d02 - driver::phase_3_run_analysis_passes::h41e2f1d518a20260GGa
   9: 0xb76584f6 - driver::compile_input::hb258e9d05d852669Qba
  10: 0xb77278fd - run_compiler::hb667b3f9ee661164n6b
  11: 0xb7724c6e - boxed::F.FnBox<A>::call_box::h4284501152216545036
  12: 0xb7724441 - rt::unwind::try::try_fn::h3762789488623463148
  13: 0xb73beb8a - rust_try_inner
  14: 0xb73beb63 - rust_try
  15: 0xb731cd7c - rt::unwind::try::inner_try::h778b76ec9ae57243Czw
  16: 0xb7724668 - boxed::F.FnBox<A>::call_box::h8463684128851076905
  17: 0xb7332054 - sys::thread::Thread::new::thread_start::h5f3e9b55c97be469OJv
  18: 0xb31c81c2 - start_thread
  19: 0xb717ee8d - __clone
  20:        0x0 - <unknown>
