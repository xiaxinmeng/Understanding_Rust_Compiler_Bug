
error: internal compiler error: Impl DefId { krate: 0, node: 12 }:f32.Num was matchable against Obligation(trait_ref=<_ : Num>,depth=0) but now is not
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:175

stack backtrace:
   1:     0x7f915ce993e0 - rt::backtrace::imp::write::h50b71c7573aa44d5Bgt
   2:     0x7f915ce9c460 - failure::on_fail::hc49a9332645beb08cCt
   3:     0x7f915d650c60 - unwind::begin_unwind_inner::hfe5348fa0e613e77E9c
   4:     0x7f915bdefa40 - unwind::begin_unwind::h2013521801281920279
   5:     0x7f915bdf01e0 - diagnostic::Handler::bug::h5ffb209701b06176C5F
   6:     0x7f915dd81300 - middle::traits::select::SelectionContext<'cx, 'tcx>::rematch_impl::h2e796c80b6b7b2d1RbZ
   7:     0x7f915dd6e380 - middle::traits::select::SelectionContext<'cx, 'tcx>::confirm_candidate::hcd9eb474da2b9af3PRY
   8:     0x7f915dd652c0 - middle::traits::select::SelectionContext<'cx, 'tcx>::select::hc07798a8a63189c7LnX
   9:     0x7f915dd63f30 - middle::traits::fulfill::FulfillmentContext::select::h38f1482e9e3d7175sUW
  10:     0x7f915deec9b0 - middle::typeck::check::vtable::select_new_fcx_obligations::h14c7bd91cbe892c8elO
  11:     0x7f915df81820 - middle::typeck::check::check_argument_types::h6b677199ffbdfaa2w4Y
  12:     0x7f915df83230 - middle::typeck::check::check_expr_with_unifier::h3fbf149ddd449935CDZ
  13:     0x7f915dfdcd30 - middle::typeck::check::check_stmt::h697d8a2a54580689zK1
  14:     0x7f915df47e50 - middle::typeck::check::check_block_with_expected::h4705ff96e614d629IO1
  15:     0x7f915df448e0 - middle::typeck::check::check_fn::he598dd4226ca32f92fW
  16:     0x7f915df44620 - middle::typeck::check::check_bare_fn::ha7a99d35a2a226a0h5V
  17:     0x7f915df40730 - middle::typeck::check::check_item::h6dc4c4f79498f002qpW
  18:     0x7f915df443e0 - middle::typeck::check::check_item_types::hae2bcc56566542e1r4V
  19:     0x7f915da3a4d0 - util::common::time::h9758243045951098006
  20:     0x7f915e2682d0 - middle::typeck::check_crate::he3aeb73f13b2d5c7lrp
  21:     0x7f915e2d0940 - driver::driver::phase_3_run_analysis_passes::h2dd06c2b56df9fc3M9B
  22:     0x7f915e2cb7d0 - driver::driver::compile_input::h0eb7d13e493860ffxQB
  23:     0x7f915e352fd0 - driver::run_compiler::hfef7578eee798d05OGF
  24:     0x7f915e352ec0 - driver::run::closure.146242
  25:     0x7f915da53d70 - task::TaskBuilder<S>::try_future::closure.104747
  26:     0x7f915da53b60 - task::TaskBuilder<S>::spawn_internal::closure.104718
  27:     0x7f915ebacb00 - task::NativeSpawner.Spawner::spawn::closure.2456
  28:     0x7f915d6a5bc0 - rust_try_inner
  29:     0x7f915d6a5bb0 - rust_try
  30:     0x7f915d64e5e0 - unwind::try::he984010f47153659mYc
  31:     0x7f915d64e470 - task::Task::run::had29636eca7af1a3u4b
  32:     0x7f915ebac840 - task::NativeSpawner.Spawner::spawn::closure.2382
  33:     0x7f915d64fc80 - thread::thread_start::h1833ce4dca7b1a7erpc
  34:     0x7f915868f250 - start_thread
  35:     0x7f915d32a3b9 - clone
  36:                0x0 - <unknown>
