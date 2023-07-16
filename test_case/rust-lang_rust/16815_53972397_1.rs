
error: internal compiler error: can't dereference const of type [u8, .. 1]
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /build/rust-git/src/rust/src/libsyntax/ast_util.rs:784

stack backtrace:
   1:     0x7f9fe6a621c0 - rt::backtrace::imp::write::hc06a9af3428b6402OLq
   2:     0x7f9fe6a653b0 - <unknown>
   3:     0x7f9fe7234c60 - unwind::begin_unwind_inner::h52e356952ae73580rje
   4:     0x7f9fe2f24920 - <unknown>
   5:     0x7f9fe2f25290 - diagnostic::Handler::bug::h9a4b58260c6eb84bCdF
   6:     0x7f9fe7620b00 - driver::session::Session::bug::h38b87ec2c07879e1U5C
   7:     0x7f9fe7a59330 - <unknown>
   8:     0x7f9fe7a57c80 - middle::trans::consts::const_expr::hfc3059cbac1ab05aIh8
   9:     0x7f9fe7a5a020 - <unknown>
  10:     0x7f9fe7a57c80 - middle::trans::consts::const_expr::hfc3059cbac1ab05aIh8
  11:     0x7f9fe79c5d20 - middle::trans::base::get_item_val::hfac7a156b8d0ce26Fxf
  12:     0x7f9fe7a59d20 - middle::trans::consts::trans_const::h395a96c97daf2b75C38
  13:     0x7f9fe79c2e40 - middle::trans::base::trans_item::hcb03a26de1f1e4a1n4e
  14:     0x7f9fe7a97e60 - middle::trans::base::trans_crate::h2bfac95542704ddfOYf
  15:     0x7f9fe7ecc6f0 - driver::driver::phase_4_translate_to_llvm::h112e0d5721d56638XHB
  16:     0x7f9fe7ec3f10 - driver::driver::compile_input::hc29a8cb80fd73a2b5jB
  17:     0x7f9fe7f57bb0 - <unknown>
  18:     0x7f9fe7f57ac0 - <unknown>
  19:     0x7f9fe7f6c0e0 - <unknown>
  20:     0x7f9fe7f6bee0 - <unknown>
  21:     0x7f9fe89d3880 - <unknown>
  22:     0x7f9fe7286f10 - <unknown>
  23:     0x7f9fe7286f00 - rust_try
  24:     0x7f9fe7232210 - unwind::try::hd6d9d0484638c05fH7d
  25:     0x7f9fe7231fb0 - task::Task::run::hc0937059e11808beNdd
  26:     0x7f9fe89d35e0 - <unknown>
  27:     0x7f9fe7233e50 - <unknown>
  28:     0x7f9fe64f5060 - start_thread
  29:     0x7f9fe6f01489 - __clone
  30:                0x0 - <unknown>
