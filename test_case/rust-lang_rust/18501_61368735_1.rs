
$ RUST_BACKTRACE=1 rustc main.rs
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'assertion failed: `(left == right) && (right == left)` (left: `3`, right: `0`)', /Users/seb/local/src/rust/src/librustc/middle/trans/callee.rs:528

stack backtrace:
   1:        0x10de67369 - rt::backtrace::imp::write::h5cb95caddc8561camlq
   2:        0x10de6a647 - failure::on_fail::h234c06cac3945e8bVBq
   3:        0x10e0cc735 - unwind::begin_unwind_inner::h9105011a78466b9eDud
   4:        0x10e0cc3fc - unwind::begin_unwind_fmt::h07316d219088e3025rd
   5:        0x10aebfc05 - middle::trans::callee::trans_fn_ref_with_substs::h0b4e9d2ee3b6558aAC3
   6:        0x10aebd0d1 - middle::trans::callee::trans_fn_ref::h55761fa4eda1110bqq3
   7:        0x10aedb131 - middle::trans::expr::trans_def::h48c606f79e7dd407VW5
   8:        0x10aed325a - middle::trans::expr::trans_unadjusted::h47409160f1eb6f86tt5
   9:        0x10ae98bc2 - middle::trans::expr::trans::hd44136491cd72162pM4
  10:        0x10aecbf44 - middle::trans::callee::trans_args::h1f6cf1efb791e624oj4
  11:        0x10aea0439 - middle::trans::callee::trans_call_inner::h55cf4dfb2f10e1b9kY3
  12:        0x10aec50d7 - middle::trans::callee::trans_call::h23ec6f0e40508d0fES3
  13:        0x10aed4c01 - middle::trans::expr::trans_rvalue_dps_unadjusted::h5b575c9c069c361eX65
  14:        0x10ae97729 - middle::trans::expr::trans_into::h80c999353edbf98cvI4
  15:        0x10ae97a7e - middle::trans::controlflow::trans_block::h997234ba9aac93d2LY0
  16:        0x10af3eafe - middle::trans::base::trans_closure::hef52fdc374efb614M4g
  17:        0x10ae8a75a - middle::trans::base::trans_fn::h427ae11941f8ecb7agh
  18:        0x10ae87e95 - middle::trans::base::trans_item::h601883efebea6853tzh
  19:        0x10af482c8 - middle::trans::base::trans_crate::hf62c9be45574a5b3kxi
  20:        0x10b382355 - driver::driver::phase_4_translate_to_llvm::hc75cae4187302a30GyA
  21:        0x10b37afe6 - driver::driver::compile_input::h85978a2ffa5320eaA5z
  22:        0x10b3ff60f - driver::run_compiler::h61c720c34640da73LSD
  23:        0x10b3fd796 - driver::run::closure.144840
  24:        0x10ab2d79b - task::TaskBuilder<S>::try_future::closure.103258
  25:        0x10ab2d683 - task::TaskBuilder<S>::spawn_internal::closure.103229
  26:        0x10d4718fd - task::NativeSpawner.Spawner::spawn::closure.8522
  27:        0x10e136bfc - rust_try_inner
  28:        0x10e136be6 - rust_try
  29:        0x10e0c9da7 - unwind::try::h25431bb1593bc97bljd
  30:        0x10e0c9c2c - task::Task::run::h56e3cfd2f55b234ewzc
  31:        0x10d471722 - task::NativeSpawner.Spawner::spawn::closure.8461
  32:        0x10e0cb5ea - thread::thread_start::h023119a990ef057fHTc
  33:     0x7fff86be82fc - _pthread_body
  34:     0x7fff86be8279 - _pthread_body
