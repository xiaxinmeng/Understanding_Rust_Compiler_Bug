
thread 'rustc' panicked at 'assertion failed: `(left == right)` (left: `2`, right: `1`)', ../src/librustc/middle/check_match.rs:1051
stack backtrace:
   1:        0x107ebe3a8 - sys::backtrace::tracing::imp::write::h8c3fbdc55f1abdcag4u
   2:        0x107ec6d15 - panicking::default_handler::_$u7b$$u7b$closure$u7d$$u7d$::closure.44007
   3:        0x107ec6817 - panicking::default_handler::h6775aa517c5d030aFvz
   4:        0x107e89fc6 - sys_common::unwind::begin_unwind_inner::h9a1b0d9dd0c062e122t
   5:        0x107e8aabe - sys_common::unwind::begin_unwind_fmt::h99d6622b969d640e81t
   6:        0x104d7731e - middle::check_match::check_irrefutable::h037ce85fe9e0719fP4j
   7:        0x104d46440 - middle::check_match::check_local::h57d1a825afa7d07fJ2j
   8:        0x104d466a7 - middle::check_match::check_fn::h02fdb8e29648c14az3j
   9:        0x104d46e8e - middle::check_match::check_crate::h5c6040bd71f7b985jSi
  10:        0x103e8090f - driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::closure.28271
  11:        0x103e7d869 - middle::ty::context::ctxt<'tcx>::create_and_enter::h8855598638456051733
  12:        0x103e7a2e2 - driver::phase_3_run_analysis_passes::h7337521525707930622
  13:        0x103e4e0bd - driver::compile_input::h467e81e458addce4Hca
  14:        0x103e3d788 - run_compiler::h971a7f9ccb1ca35e6Oc
  15:        0x103e3ade5 - sys_common::unwind::try::try_fn::h8709370279840092323
  16:        0x107ebbd7b - __rust_try
  17:        0x107eb3fa3 - sys_common::unwind::inner_try::h8630ed5c10186d0a4Zt
  18:        0x103e3b699 - boxed::F.FnBox<A>::call_box::h4993598626702227198
  19:        0x107ec5d0c - sys::thread::Thread::new::thread_start::h2721df3f64ee52b36Jy
  20:     0x7fff81f6f059 - _pthread_body
  21:     0x7fff81f6efd6 - _pthread_start
