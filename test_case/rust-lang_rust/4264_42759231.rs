
error: internal compiler error: node_id_to_type: no type for node `expr 1u (id=19)`
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/erickt/rust/rust-master/src/libsyntax/diagnostic.rs:155
stack backtrace:
   1:        0x10bcc75b5 - rt::backtrace::imp::write::hae9e0c8649bd0651E1F::v0.11.pre
   2:        0x10bc346ae - rt::unwind::begin_unwind_inner::hd5684ca19e9518eaIBF::v0.11.pre
   3:        0x10b361af5 - rt::unwind::begin_unwind::h6760631091188135926::v0.11.pre
   4:        0x10b3623ab - diagnostic::Handler::bug::h31b3a7870496b98ae0b::v0.11.pre
   5:        0x108f261d8 - driver::session::Session::bug::hd05ef4561dcdac8a04j::v0.11.pre
   6:        0x108f89788 - middle::ty::node_id_to_type::h99b941bc6d3c3a6eDON::v0.11.pre
   7:        0x109736bc4 - driver::driver::TypedAnnotation.pprust..PpAnn::post::h2a84cad08dcbcd71BMi::v0.11.pre
   8:        0x10b4496dd - print::pprust::State<'a>::print_expr::h60c4507008911968DIO::v0.11.pre
   9:        0x10b433d13 - print::pprust::State<'a>::print_type::he7151521714d862e2nN::v0.11.pre
  10:        0x10b44f053 - print::pprust::State<'a>::print_item::hbcedf05303f0115dkDN::v0.11.pre
  11:        0x10b46753f - print::pprust::State<'a>::print_decl::h5ab70339d2092c31fDP::v0.11.pre
  12:        0x10b44ad11 - print::pprust::State<'a>::print_stmt::hc78e10d0e5992b99XrO::v0.11.pre
  13:        0x10b468fca - print::pprust::State<'a>::print_block_maybe_unclosed::ha0e6c0106ccaf1d5KwO::v0.11.pre
  14:        0x10b4459f4 - print::pprust::State<'a>::print_expr::h60c4507008911968DIO::v0.11.pre
  15:        0x10b44b309 - print::pprust::State<'a>::print_stmt::hc78e10d0e5992b99XrO::v0.11.pre
  16:        0x10b468fca - print::pprust::State<'a>::print_block_maybe_unclosed::ha0e6c0106ccaf1d5KwO::v0.11.pre
  17:        0x10b44de7d - print::pprust::State<'a>::print_item::hbcedf05303f0115dkDN::v0.11.pre
  18:        0x10b42fea8 - print::pprust::State<'a>::print_mod::h641f136b20a8e075MjN::v0.11.pre
  19:        0x10b42f744 - print::pprust::print_crate::h9326c22da146057bAQM::v0.11.pre
  20:        0x109737bdb - driver::driver::pretty_print_input::h0542caccd914e61bmOi::v0.11.pre
  21:        0x1097597c9 - run_compiler::h6db406e36b756e80saq::v0.11.pre
  22:        0x109771d8d - main_args::closure.93972
  23:        0x109770502 - monitor::closure.93850
  24:        0x10976b05b - task::TaskBuilder::try::closure.93616
  25:        0x10b302b6c - task::spawn_opts::closure.7397
  26:        0x10bcbec18 - rt::task::Task::run::closure.28413
  27:        0x10bcdfb7c - rust_try
  28:        0x10bcbea97 - rt::task::Task::run::hf99ca2c1546bbfb0XqD::v0.11.pre
  29:        0x10b3029ef - task::spawn_opts::closure.7369
  30:        0x10bcc5f16 - rt::thread::thread_start::h61d07f070734480188D::v0.11.pre
  31:     0x7fff90436899 - _pthread_body
  32:     0x7fff9043672a - _pthread_struct_init
