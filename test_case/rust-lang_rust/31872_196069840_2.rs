
thread 'rustc' panicked at 'assertion failed: `(left == right)` (left: `2`, right: `1`)', ../src/librustc/middle/check_match.rs:1051
stack backtrace:
   1:        0x113bc43a8 - sys::backtrace::tracing::imp::write::h7add7fdda97786a1s4u
   2:        0x113bccd15 - panicking::default_handler::_$u7b$$u7b$closure$u7d$$u7d$::closure.44030
   3:        0x113bcc817 - panicking::default_handler::h72a4f6df9637bc80Rvz
   4:        0x113b8ffc6 - sys_common::unwind::begin_unwind_inner::hf798769ad97c097fe3t
   5:        0x113b90abe - sys_common::unwind::begin_unwind_fmt::hb2ee19b4aa59ba8ak2t
   6:        0x1105cd7be - middle::check_match::check_irrefutable::hb3dbd95dda6de55c92j
   7:        0x11059c980 - middle::check_match::check_local::h61b952193c791ff530j
   8:        0x11059cbe7 - middle::check_match::check_fn::hab6c47340d646ea4T1j
   9:        0x11059d3ce - middle::check_match::check_crate::h4de6707235db8a8dDQi
  10:        0x10f6dea1f - driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::closure.28283
  11:        0x10f6db979 - middle::ty::context::ctxt<'tcx>::create_and_enter::h17676359018619823038
  12:        0x10f6d83f2 - driver::phase_3_run_analysis_passes::h9214455933593547357
  13:        0x10f6ac19d - driver::compile_input::h26b63276678a06cfHca
  14:        0x10f699ce8 - run_compiler::hd54b7109a035c12b6Oc
  15:        0x10f697345 - sys_common::unwind::try::try_fn::h18306268176533686141
  16:        0x113bc1d7b - __rust_try
  17:        0x113bb9fa3 - sys_common::unwind::inner_try::hdaa7ae415a983f2ag0t
  18:        0x10f697bf9 - boxed::F.FnBox<A>::call_box::h14172178064022666043
  19:        0x113bcbd0c - sys::thread::Thread::new::thread_start::h45441f586afbe2dciKy
  20:     0x7fff81f6f059 - _pthread_body
  21:     0x7fff81f6efd6 - _pthread_start
