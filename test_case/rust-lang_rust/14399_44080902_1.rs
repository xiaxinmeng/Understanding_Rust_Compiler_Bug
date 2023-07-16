
reduce.rs:9:1: 11:2 warning: code is never used: `Quat`, #[warn(dead_code)] on by default
reduce.rs:9 struct Quat {
reduce.rs:10     v: Box<Vec3>,
reduce.rs:11 }
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'could not find method while translating', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libstd/option.rs:167
stack backtrace:
   1:     0x7fbe8942aed0 - rt::backtrace::imp::write::h829de4821815e280PUy::v0.11.0.pre
   2:     0x7fbe893a7c50 - rt::unwind::begin_unwind_inner::h3c95a01106a60df1Xuy::v0.11.0.pre
   3:     0x7fbe89e9b250 - rt::unwind::begin_unwind::h8363202580871814802::v0.11.0.pre
   4:     0x7fbe89fe8790 - middle::trans::meth::method_with_name::h27fe9715defde429xnv::v0.11.0.pre
   5:     0x7fbe89f1c610 - middle::trans::meth::trans_method_callee::he0d18bd58a692f20t8u::v0.11.0.pre
   6:     0x7fbe89f1c5a0 - middle::trans::callee::trans_method_call::closure.58859
   7:     0x7fbe89f1a0e0 - middle::trans::callee::trans_call_inner::h6ec9e97b2f0cf3a9xId::v0.11.0.pre
   8:     0x7fbe89f1bef0 - middle::trans::callee::trans_method_call::hb058dc63facf7471NDd::v0.11.0.pre
   9:     0x7fbe89f24c30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  10:     0x7fbe89ee5430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  11:     0x7fbe89ee59e0 - middle::trans::controlflow::trans_block::h7d3f8bec55eca940I7a::v0.11.0.pre
  12:     0x7fbe89f24c30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  13:     0x7fbe89ee5430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  14:     0x7fbe89f31a40 - middle::trans::expr::trans_uniq_expr::hbc1817203acccbdah3f::v0.11.0.pre
  15:     0x7fbe89f325b0 - middle::trans::expr::trans_unary::hca6bb9d2119494eeLXf::v0.11.0.pre
  16:     0x7fbe89f23b00 - middle::trans::expr::trans_unadjusted::ha9f4b115887b34acmPe::v0.11.0.pre
  17:     0x7fbe89ee5430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  18:     0x7fbe89ee59e0 - middle::trans::controlflow::trans_block::h7d3f8bec55eca940I7a::v0.11.0.pre
  19:     0x7fbe89fa2a70 - middle::trans::base::trans_closure::h247c5303be3eb095Php::v0.11.0.pre
  20:     0x7fbe89eae960 - middle::trans::base::trans_fn::hac7f31459ea86cc45pp::v0.11.0.pre
  21:     0x7fbe89eaee60 - middle::trans::monomorphize::monomorphic_fn::h29e7539c7e7490a1Lka::v0.11.0.pre
  22:     0x7fbe89f0fb10 - middle::trans::callee::trans_fn_ref_with_vtables::h739a9d33fff1d6b8Kjd::v0.11.0.pre
  23:     0x7fbe89f0b2f0 - middle::trans::callee::trans_fn_ref::hf912bb2e1b25d34aScd::v0.11.0.pre
  24:     0x7fbe89f1c610 - middle::trans::meth::trans_method_callee::he0d18bd58a692f20t8u::v0.11.0.pre
  25:     0x7fbe89f1c5a0 - middle::trans::callee::trans_method_call::closure.58859
  26:     0x7fbe89f1a0e0 - middle::trans::callee::trans_call_inner::h6ec9e97b2f0cf3a9xId::v0.11.0.pre
  27:     0x7fbe89f1bef0 - middle::trans::callee::trans_method_call::hb058dc63facf7471NDd::v0.11.0.pre
  28:     0x7fbe89f24c30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  29:     0x7fbe89f23b00 - middle::trans::expr::trans_unadjusted::ha9f4b115887b34acmPe::v0.11.0.pre
  30:     0x7fbe89ee7720 - middle::trans::expr::trans::h31b1d37217a231c6cwe::v0.11.0.pre
  31:     0x7fbe89f1deb0 - middle::trans::callee::trans_args::h79811d38aa6e67c3N2d::v0.11.0.pre
  32:     0x7fbe89f1a0e0 - middle::trans::callee::trans_call_inner::h6ec9e97b2f0cf3a9xId::v0.11.0.pre
  33:     0x7fbe89f19eb0 - middle::trans::callee::trans_call::h16885ad67fa12e03FCd::v0.11.0.pre
  34:     0x7fbe89f24c30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  35:     0x7fbe89ee5430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  36:     0x7fbe89ee59e0 - middle::trans::controlflow::trans_block::h7d3f8bec55eca940I7a::v0.11.0.pre
  37:     0x7fbe89fa2a70 - middle::trans::base::trans_closure::h247c5303be3eb095Php::v0.11.0.pre
  38:     0x7fbe89eae960 - middle::trans::base::trans_fn::hac7f31459ea86cc45pp::v0.11.0.pre
  39:     0x7fbe89fa6c40 - middle::trans::meth::trans_impl::hcad0b0d6fab6bc8cX3u::v0.11.0.pre
  40:     0x7fbe89ea91a0 - middle::trans::base::trans_item::h62a895b39c0cb8ccCBp::v0.11.0.pre
  41:     0x7fbe89fb11f0 - middle::trans::base::trans_crate::h3026657c475a7720poq::v0.11.0.pre
  42:     0x7fbe8a76cd30 - driver::driver::phase_4_translate_to_llvm::h639e7dc77f10a518H1j::v0.11.0.pre
  43:     0x7fbe8a762000 - driver::driver::compile_input::hc7e76cb3959dc3aeiJj::v0.11.0.pre
  44:     0x7fbe8a82d820 - driver::run_compiler::ha4687608fe641c15bmm::v0.11.0.pre
  45:     0x7fbe8a82d740 - driver::main_args::closure.93513
  46:     0x7fbe8a842a20 - driver::monitor::closure.94585
  47:     0x7fbe8a83dd10 - task::TaskBuilder::try::closure.94348
  48:     0x7fbe899495a0 - task::spawn_opts::closure.7809
  49:     0x7fbe89425050 - rt::task::Task::run::closure.25264
  50:     0x7fbe89489b10 - rust_try
  51:     0x7fbe89424fa0 - rt::task::Task::run::h82b2a3f233cfc4a9Ajw::v0.11.0.pre
  52:     0x7fbe89949350 - task::spawn_opts::closure.7782
  53:     0x7fbe894292d0 - rt::thread::thread_start::he85cc53b3abeddd182w::v0.11.0.pre
  54:     0x7fbe86ecdfa0 - start_thread
  55:     0x7fbe89070a09 - __clone
  56:                0x0 - <unknown>

