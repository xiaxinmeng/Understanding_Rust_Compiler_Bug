
rror: internal compiler error: adt::represent_type called on non-ADT type: !
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:175

stack backtrace:
   1:     0x7f4ffcb17bf0 - rt::backtrace::imp::write::h37a3af22082b7094FKq
   2:     0x7f4ffcb1ad20 - failure::on_fail::hf01d9f1e1bdb6e5425q
   3:     0x7f5001083b30 - unwind::begin_unwind_inner::h2ec64a9d433f6b868yd
   4:     0x7f4ffd890610 - unwind::begin_unwind::h11468189666407495110
   5:     0x7f4ffd890dc0 - diagnostic::Handler::bug::h10aa9d98e5527d0113F
   6:     0x7f5001a99cf0 - middle::trans::adt::represent_type::hf0465ce97f58ee09Kqu
   7:     0x7f5001ade0b0 - middle::trans::expr::trans_adt::h26ddc6e798cc0d53Hr6
   8:     0x7f5001acaa20 - middle::trans::expr::trans_rvalue_dps_unadjusted::h59ca517824b4c63aET5
   9:     0x7f5001a8e380 - middle::trans::expr::trans_into::hc8db703a1280c662cv4
  10:     0x7f5001a8e7a0 - middle::trans::controlflow::trans_block::h26f0fa752a438c4fDL0
  11:     0x7f5001b361d0 - middle::trans::base::trans_closure::haec1e74530eb7cefQIg
  12:     0x7f5001a80da0 - middle::trans::base::trans_fn::h778cc3315cb016b0eUg
  13:     0x7f5001a7e400 - middle::trans::base::trans_item::h8e6f687fd31b992bxdh
  14:     0x7f5001b41030 - middle::trans::base::trans_crate::h1e2bed8778c95c8dkbi
  15:     0x7f5001f9a3b0 - driver::driver::phase_4_translate_to_llvm::hbdeb13ca358e159eD9z
  16:     0x7f5001f91450 - driver::driver::compile_input::h8a818e073bdb900cyGz
  17:     0x7f500201c950 - driver::run_compiler::h7e400358677f3066StD
  18:     0x7f500201c800 - driver::run::closure.145782
  19:     0x7f5001701b70 - task::TaskBuilder<S>::try_future::closure.103164
  20:     0x7f5001701950 - task::TaskBuilder<S>::spawn_internal::closure.103135
  21:     0x7f50013cea30 - task::spawn_opts::closure.8464
  22:     0x7f50010ddaa0 - rust_try_inner
  23:     0x7f50010dda90 - rust_try
  24:     0x7f50010813f0 - unwind::try::h0bc5cc5739b8d3c3Qnd
  25:     0x7f5001081270 - task::Task::run::h517e4360aedfbf45uDc
  26:     0x7f50013ce770 - task::spawn_opts::closure.8404
  27:     0x7f5001082b40 - thread::thread_start::h43c1915a7e23212cFXc
  28:     0x7f4ffbec3fe0 - start_thread
  29:     0x7f5000d53bf9 - __clone
  30:                0x0 - <unknown>
