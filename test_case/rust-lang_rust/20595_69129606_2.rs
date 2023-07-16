
stack backtrace:
   1:     0x7f7d142e0410 - sys::backtrace::write::hb159f9850e834894l1s
   2:     0x7f7d14305150 - failure::on_fail::hd4fb51e7b66e269fxgz
   3:     0x7f7d1426af20 - rt::unwind::begin_unwind_inner::he95e3015bc1198c6tVy
   4:     0x7f7d0efc1dd0 - rt::unwind::begin_unwind::h1691522525006698826
   5:     0x7f7d0efc1d60 - diagnostic::SpanHandler::span_bug::h419846186c6e05bbkEF
   6:     0x7f7d126f2360 - middle::traits::error_reporting::report_fulfillment_error::h23831a3c82b7aea8LKO
   7:     0x7f7d125cd2f0 - middle::traits::error_reporting::report_fulfillment_errors::h85e3379b4029c0b8gKO
   8:     0x7f7d139be950 - check::vtable::select_all_fcx_obligations_or_error::h019e719f5fe0b73dqkb
   9:     0x7f7d13a600f0 - check::check_bare_fn::h368991e084d6817flzj
  10:     0x7f7d13a663a0 - check::check_method_body::hf3fecb89a7935cbeB1j
  11:     0x7f7d13a57840 - check::check_item::hcbf00dd224aa026f7Rj
  12:     0x7f7d13b4bae0 - check_crate::unboxed_closure.40379
  13:     0x7f7d13b46720 - check_crate::h3f64d3a4a846d0b2Nrx
  14:     0x7f7d14820cd0 - driver::phase_3_run_analysis_passes::had0a81b4c9f8a9cd6va
  15:     0x7f7d1480eb50 - driver::compile_input::h4f21f66f6761bb22vba
  16:     0x7f7d148d7640 - thunk::F.Invoke<A, R>::invoke::h1208443586143045988
  17:     0x7f7d148d63f0 - rt::unwind::try::try_fn::h4735177180287591181
  18:     0x7f7d14369810 - rust_try_inner
  19:     0x7f7d14369800 - rust_try
  20:     0x7f7d148d6740 - thunk::F.Invoke<A, R>::invoke::h6623100047367797814
  21:     0x7f7d142f13d0 - sys::thread::thread_start::h89f54a8cba556c09RSv
  22:     0x7f7d0e7e20c0 - start_thread
  23:     0x7f7d13f12ec9 - __clone
  24:                0x0 - <unknown>
