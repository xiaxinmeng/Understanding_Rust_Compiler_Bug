
ice.rs:1:1: 1:64 warning: code is never used: `BLOCK_ENUM_CONSTRUCTOR`, #[warn(dead_code)] on by default
ice.rs:1 static BLOCK_ENUM_CONSTRUCTOR: fn(uint) -> Option<uint> = Some;
         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: internal compiler error: adt::represent_type called on non-ADT type
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/erickt/rust/rust-master/src/libsyntax/diagnostic.rs:155
stack backtrace:
   1:        0x110b685b5 - rt::backtrace::imp::write::hae9e0c8649bd0651E1F::v0.11.pre
   2:        0x110ad56ae - rt::unwind::begin_unwind_inner::hd5684ca19e9518eaIBF::v0.11.pre
   3:        0x11020faf5 - rt::unwind::begin_unwind::h6760631091188135926::v0.11.pre
   4:        0x1102103ab - diagnostic::Handler::bug::h31b3a7870496b98ae0b::v0.11.pre
   5:        0x10ddd21d8 - driver::session::Session::bug::hd05ef4561dcdac8a04j::v0.11.pre
   6:        0x10de1bcae - middle::trans::adt::represent_type::he7d194c42dced15dwKC::v0.11.pre
   7:        0x10de815fa - middle::trans::consts::const_expr_unadjusted::h08b31cc93b7529213Uj::v0.11.pre
   8:        0x10de7decd - middle::trans::consts::const_expr::h450bc707188aded1dFj::v0.11.pre
   9:        0x10ddcfedf - middle::trans::base::get_item_val::h037ac9bd804c5f8ckQp::v0.11.pre
  10:        0x10de7fd05 - middle::trans::consts::trans_const::h2f0e3312daf9b7baPsk::v0.11.pre
  11:        0x10ddce415 - middle::trans::base::trans_item::hae404c2fd6d891036vp::v0.11.pre
  12:        0x10deba7c1 - middle::trans::base::trans_crate::hf95e38f62840fed6Vhq::v0.11.pre
  13:        0x10e5dc534 - driver::driver::phase_4_translate_to_llvm::hdd1377f79c12d83e8li::v0.11.pre
  14:        0x10e5e01e5 - driver::driver::compile_input::h2c3dbe74f799efa1gEi::v0.11.pre
  15:        0x10e606211 - run_compiler::h6db406e36b756e80saq::v0.11.pre
  16:        0x10e61dd8d - main_args::closure.93972
  17:        0x10e61c502 - monitor::closure.93850
  18:        0x10e61705b - task::TaskBuilder::try::closure.93616
  19:        0x1101aeb6c - task::spawn_opts::closure.7397
  20:        0x110b5fc18 - rt::task::Task::run::closure.28413
  21:        0x110b80b7c - rust_try
  22:        0x110b5fa97 - rt::task::Task::run::hf99ca2c1546bbfb0XqD::v0.11.pre
  23:        0x1101ae9ef - task::spawn_opts::closure.7369
  24:        0x110b66f16 - rt::thread::thread_start::h61d07f070734480188D::v0.11.pre
  25:     0x7fff90436899 - _pthread_body
  26:     0x7fff9043672a - _pthread_struct_init
