
stack backtrace:
   1:     0x7f290f12bee9 - sys::backtrace::write::h9d474f8e9af32ff4eas
   2:     0x7f290f133a29 - panicking::on_panic::h780bc016e25d560eHOw
   3:     0x7f290f0f4a52 - rt::unwind::begin_unwind_inner::he10b5af763481bfaRtw
   4:     0x7f290c4e999d - rt::unwind::begin_unwind::h10007532221998759728
   5:     0x7f290c4e9932 - diagnostic::SpanHandler::span_bug::hdbc9b25081b2fe3dKWB
   6:     0x7f290d10f83d - session::Session::abort_if_errors::ha2c34f711c028133dJq
   7:     0x7f290e9d6f54 - check_crate::closure.38688
   8:     0x7f290e9d0180 - check_crate::h08a7388cd8cd25829BC
   9:     0x7f290f678bb8 - driver::phase_3_run_analysis_passes::h876ca40ef22f652btGa
  10:     0x7f290f659fec - driver::compile_input::h19dbb81f9d079a10Qba
  11:     0x7f290f712dc1 - run_compiler::h6fdcc5ae4819da2c65b
  12:     0x7f290f710612 - boxed::F.FnBox<A>::call_box::h15867338134671801927
  13:     0x7f290f70fbd9 - rt::unwind::try::try_fn::h10571205104630549427
  14:     0x7f290f1a72f8 - rust_try_inner
  15:     0x7f290f1a72e5 - rust_try
  16:     0x7f290f70fe74 - boxed::F.FnBox<A>::call_box::h11752498478748955554
  17:     0x7f290f1327c1 - sys::thread::Thread::new::thread_start::h9f92b31902592327dAv
  18:     0x7f29091e5373 - start_thread
  19:     0x7f290ed8827c - clone
  20:                0x0 - <unknown>
