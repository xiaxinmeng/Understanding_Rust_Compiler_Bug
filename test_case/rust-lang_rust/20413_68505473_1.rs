
compiler_bug.rs:7:13: 7:16 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `core::kinds::Sized` for the type `NoData<T>`
compiler_bug.rs:7         let val: NoData<T> = NoData;
                              ^~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:123

stack backtrace:
   1:     0x7f55f88b4950 - sys::backtrace::write::h895a7595e35ffe42gnt
   2:     0x7f55f88d9660 - failure::on_fail::hb3298435971cff34kEz
   3:     0x7f55f8840940 - rt::unwind::begin_unwind_inner::h6dee24d5dc335a63oiz
   4:     0x7f55f3a09cc0 - rt::unwind::begin_unwind::h12623432956124006653
   5:     0x7f55f3a09c50 - diagnostic::SpanHandler::span_bug::hb1e8fbfda78f91d3aPF
   6:     0x7f55f6c43f80 - middle::traits::error_reporting::report_fulfillment_error::hc3d51bb26b08511dQWO
   7:     0x7f55f6b1d0b0 - middle::traits::error_reporting::report_fulfillment_errors::h416175d5a65156fclWO
   8:     0x7f55f7e7bf50 - check::vtable::select_all_fcx_obligations_or_error::hab001ea061dfaa499mb
   9:     0x7f55f7f3f2c0 - check::check_bare_fn::h24d3cb693ab9ca75Qzj
  10:     0x7f55f7f44a70 - check::check_method_body::h18ea7cf32e2032f6w2j
  11:     0x7f55f7f36950 - check::check_item::hb991f99d47aa82ceSSj
  12:     0x7f55f80d7030 - check_crate::unboxed_closure.39833
  13:     0x7f55f80d1dd0 - check_crate::hb802e7bb2dee1cd6NLx
  14:     0x7f55f8dfd860 - driver::phase_3_run_analysis_passes::h83e2236a47a83223qva
  15:     0x7f55f8dec0a0 - driver::compile_input::h2bdea93b5d9f3dbfwba
  16:     0x7f55f8f37970 - thunk::F.Invoke<A, R>::invoke::h11261394929911795771
  17:     0x7f55f8f36720 - rt::unwind::try::try_fn::h18390836921836612068
  18:     0x7f55f893f2d0 - rust_try_inner
  19:     0x7f55f893f2c0 - rust_try
  20:     0x7f55f8f36a70 - thunk::F.Invoke<A, R>::invoke::h16439250889300796363
  21:     0x7f55f88c6300 - sys::thread::thread_start::haeaaf0233fbdd5c3bgw
  22:     0x7f55f322a0c0 - start_thread
  23:     0x7f55f84e6ec9 - __clone
  24:                0x0 - <unknown>
