
error: internal compiler error: adt::represent_type called on non-ADT type
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/erickt/rust/rust-master/src/libsyntax/diagnostic.rs:155
stack backtrace:
   1:        0x1076985b5 - rt::backtrace::imp::write::hae9e0c8649bd0651E1F::v0.11.pre
   2:        0x1076056ae - rt::unwind::begin_unwind_inner::hd5684ca19e9518eaIBF::v0.11.pre
   3:        0x106d3baf5 - rt::unwind::begin_unwind::h6760631091188135926::v0.11.pre
   4:        0x106d3c3ab - diagnostic::Handler::bug::h31b3a7870496b98ae0b::v0.11.pre
   5:        0x10490d1d8 - driver::session::Session::bug::hd05ef4561dcdac8a04j::v0.11.pre
   6:        0x104956cae - middle::trans::adt::represent_type::he7d194c42dced15dwKC::v0.11.pre
   7:        0x104977b6d - middle::trans::expr::trans_rvalue_dps_unadjusted::h235b52d8b1db4a08Iff::v0.11.pre
   8:        0x10493fe06 - middle::trans::expr::trans_into::h6f7f93834e172236Tpe::v0.11.pre
   9:        0x104940161 - middle::trans::controlflow::trans_block::hf9b1b309c77796b8R7a::v0.11.pre
  10:        0x1049e92be - middle::trans::base::trans_closure::ha6b3cdecc6ad6371kcp::v0.11.pre
  11:        0x10490e336 - middle::trans::base::trans_fn::h424a69e2f61f5a12Akp::v0.11.pre
  12:        0x1049099a9 - middle::trans::base::trans_item::hae404c2fd6d891036vp::v0.11.pre
  13:        0x1049f57c1 - middle::trans::base::trans_crate::hf95e38f62840fed6Vhq::v0.11.pre
  14:        0x105117534 - driver::driver::phase_4_translate_to_llvm::hdd1377f79c12d83e8li::v0.11.pre
  15:        0x10511b1e5 - driver::driver::compile_input::h2c3dbe74f799efa1gEi::v0.11.pre
  16:        0x105141211 - run_compiler::h6db406e36b756e80saq::v0.11.pre
  17:        0x105158d8d - main_args::closure.93972
  18:        0x105157502 - monitor::closure.93850
  19:        0x10515205b - task::TaskBuilder::try::closure.93616
  20:        0x106cdcb6c - task::spawn_opts::closure.7397
  21:        0x10768fc18 - rt::task::Task::run::closure.28413
  22:        0x1076b0b7c - rust_try
  23:        0x10768fa97 - rt::task::Task::run::hf99ca2c1546bbfb0XqD::v0.11.pre
  24:        0x106cdc9ef - task::spawn_opts::closure.7369
  25:        0x107696f16 - rt::thread::thread_start::h61d07f070734480188D::v0.11.pre
  26:     0x7fff90436899 - _pthread_body
  27:     0x7fff9043672a - _pthread_struct_init
