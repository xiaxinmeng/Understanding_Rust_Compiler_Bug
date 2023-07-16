
stack backtrace:
   1:     0x555e2d41dd70 - sys::backtrace::tracing::imp::write::hf017a53caeea9181K0t
   2:     0x555e2d422d15 - panicking::log_panic::_<closure>::closure.41445
   3:     0x555e2d4227d7 - panicking::log_panic::h1721f4958b0153faY7x
   4:     0x555e2d4056e3 - sys_common::unwind::begin_unwind_inner::hceb53b2cdeb63b0ajTs
   5:     0x555e2d1797d7 - sys_common::unwind::begin_unwind::begin_unwind::h8702540759513209410
   6:     0x555e2d1799b4 - errors::Handler::bug::h596e91013ba395d3AZc
   7:     0x555e2be38efc - session::Session::bug::h4287060760826713mLr
   8:     0x555e2bf9a116 - middle::traits::coherence::ty_is_local_constructor::hcb9781ef105b4c51deT
   9:     0x555e2bf993ab - middle::traits::coherence::ty_is_local::h44a33c4aa00b2a11XcT
  10:     0x555e2bf979bc - middle::traits::coherence::orphan_check_trait_ref::h0075a05a4454a1f6B3S
  11:     0x555e2bf992ef - middle::traits::coherence::orphan_check::ha7120e3f654aa863iZS
  12:     0x555e2b929540 - coherence::orphan::OrphanChecker<'cx, 'tcx>.intravisit..Visitor<'v>::visit_item::h288fc85d45b3819eK6A
  13:     0x555e2b7e0db5 - coherence::check_coherence::h6a2cd04514b79e01raC
  14:     0x555e2b7d9be7 - check_crate::h3cba37cadfcc2d65dFD
  15:     0x555e2b65d588 - driver::phase_3_run_analysis_passes::_<closure>::closure.25745
  16:     0x555e2b63ffb5 - middle::ty::context::ctxt<'tcx>::create_and_enter::create_and_enter::h5563987744098407412
  17:     0x555e2b63b8d1 - driver::phase_3_run_analysis_passes::h15227196103201629072
  18:     0x555e2b610569 - driver::compile_input::h8b92c3e1d5e7c1b1jca
  19:     0x555e2b60277b - run_compiler::hb2fb2dc8aadcbe345wc
  20:     0x555e2b5ff536 - sys_common::unwind::try::try_fn::try_fn::h12384702373597705490
  21:     0x555e2d41d218 - __rust_try
  22:     0x555e2d41847b - sys_common::unwind::try::inner_try::h95ceacb411385b64RPs
  23:     0x555e2b5ff890 - boxed::F.FnBox<A>::call_box::call_box::h15731941968204426205
  24:     0x555e2d421683 - sys::thread::Thread::new::thread_start::hf71a3c2717797118bax
  25:     0x7f5cb96114a3 - start_thread
  26:     0x7f5cb8a2f13c - clone
  27:                0x0 - <unknown>
