
compiler_bug2.rs:6:9: 6:12 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `core::kinds::Sized` for the type `NoData<T>`
compiler_bug2.rs:6     let val = NoData::<T>;
                           ^~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:123

stack backtrace:
   1:     0x7f467f67e950 - sys::backtrace::write::h895a7595e35ffe42gnt
   2:     0x7f467f6a3660 - failure::on_fail::hb3298435971cff34kEz
   3:     0x7f467f60a940 - rt::unwind::begin_unwind_inner::h6dee24d5dc335a63oiz
   4:     0x7f467a7d3cc0 - rt::unwind::begin_unwind::h12623432956124006653
   5:     0x7f467a7d3c50 - diagnostic::SpanHandler::span_bug::hb1e8fbfda78f91d3aPF
   6:     0x7f467da0df80 - middle::traits::error_reporting::report_fulfillment_error::hc3d51bb26b08511dQWO
   7:     0x7f467d8e70b0 - middle::traits::error_reporting::report_fulfillment_errors::h416175d5a65156fclWO
   8:     0x7f467ec45f50 - check::vtable::select_all_fcx_obligations_or_error::hab001ea061dfaa499mb
   9:     0x7f467ed092c0 - check::check_bare_fn::h24d3cb693ab9ca75Qzj
  10:     0x7f467ed00950 - check::check_item::hb991f99d47aa82ceSSj
  11:     0x7f467eea1030 - check_crate::unboxed_closure.39833
  12:     0x7f467ee9bdd0 - check_crate::hb802e7bb2dee1cd6NLx
  13:     0x7f467fbc7860 - driver::phase_3_run_analysis_passes::h83e2236a47a83223qva
  14:     0x7f467fbb60a0 - driver::compile_input::h2bdea93b5d9f3dbfwba
  15:     0x7f467fd01970 - thunk::F.Invoke<A, R>::invoke::h11261394929911795771
  16:     0x7f467fd00720 - rt::unwind::try::try_fn::h18390836921836612068
  17:     0x7f467f7092d0 - rust_try_inner
  18:     0x7f467f7092c0 - rust_try
  19:     0x7f467fd00a70 - thunk::F.Invoke<A, R>::invoke::h16439250889300796363
  20:     0x7f467f690300 - sys::thread::thread_start::haeaaf0233fbdd5c3bgw
  21:     0x7f4679ff40c0 - start_thread
  22:     0x7f467f2b0ec9 - __clone
  23:                0x0 - <unknown>
