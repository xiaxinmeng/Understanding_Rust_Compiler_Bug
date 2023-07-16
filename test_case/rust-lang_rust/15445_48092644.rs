 Shell
stack backtrace:
   1:     0x2ba6720b5f50 - rt::backtrace::imp::write::h0a6e09d5ac6a396dQMp::v0.11.0
   2:     0x2ba6720bb890 - failure::on_fail::hb59ace9e326a1a4457p::v0.11.0
   3:     0x2ba671947cc0 - unwind::begin_unwind_inner::hf3d3b2017fc07a23cZd::v0.11.0
   4:     0x2ba673603e90 - unwind::begin_unwind::h5897199229657993564::v0.11.0
   5:     0x2ba673604830 - diagnostic::Handler::bug::h7e6dbd99aeae433dDgc::v0.11.0
   6:     0x2ba66ef57350 - driver::session::Session::bug::h3265e34a151b5575Xlr::v0.11.0
   7:     0x2ba66efd8d90 - middle::trans::expr::trans_imm_cast::he03de5a08ee3e7b1KQh::v0.11.0
   8:     0x2ba66efca840 - middle::trans::expr::trans_unadjusted::hd9a3a8c0c81f5db550f::v0.11.0
   9:     0x2ba66ef894c0 - middle::trans::expr::trans::hc2116d27852bd6d2DHf::v0.11.0
  10:     0x2ba66efc48e0 - middle::trans::callee::trans_args::hd2a2511dfbe94b6abef::v0.11.0
  11:     0x2ba66efbbb20 - middle::trans::callee::trans_call_inner::hdfdb8c30fa124095R1e::v0.11.0
  12:     0x2ba66efc1900 - middle::trans::callee::trans_call::h9df70b3aef437bdcZVe::v0.11.0
  13:     0x2ba66efcb640 - middle::trans::expr::trans_rvalue_dps_unadjusted::hf749359ec6e2a160hvg::v0.11.0
  14:     0x2ba66ef874e0 - middle::trans::expr::trans_into::hf0a37465b961389bIDf::v0.11.0
  15:     0x2ba66f078a50 - middle::trans::_match::trans_match_inner::h21253cb4fd5b92c9uOt::v0.11.0
  16:     0x2ba66efcb640 - middle::trans::expr::trans_rvalue_dps_unadjusted::hf749359ec6e2a160hvg::v0.11.0
  17:     0x2ba66ef874e0 - middle::trans::expr::trans_into::hf0a37465b961389bIDf::v0.11.0
  18:     0x2ba66ef87b10 - middle::trans::controlflow::trans_block::hecc22008fef2a1bd2ic::v0.11.0
  19:     0x2ba66efcb640 - middle::trans::expr::trans_rvalue_dps_unadjusted::hf749359ec6e2a160hvg::v0.11.0
  20:     0x2ba66ef874e0 - middle::trans::expr::trans_into::hf0a37465b961389bIDf::v0.11.0
  21:     0x2ba66ef87b10 - middle::trans::controlflow::trans_block::hecc22008fef2a1bd2ic::v0.11.0
  22:     0x2ba66f04a710 - middle::trans::base::trans_closure::h639d28c7bc97ae139tq::v0.11.0
  23:     0x2ba66ef58530 - middle::trans::base::trans_fn::he5cf57c39474f14caCq::v0.11.0
  24:     0x2ba66f04e620 - middle::trans::foreign::trans_rust_fn_with_foreign_abi::hb01b3dc1f75a84bamvy::v0.11.0
  25:     0x2ba66ef514a0 - middle::trans::base::trans_item::h17204c67e6746031BSq::v0.11.0
  26:     0x2ba66ef514a0 - middle::trans::base::trans_item::h17204c67e6746031BSq::v0.11.0
  27:     0x2ba66f05b300 - middle::trans::base::trans_crate::hfddcc2510b7c9311IMr::v0.11.0
  28:     0x2ba66f77fe80 - driver::driver::phase_4_translate_to_llvm::h6b5f55c3e1863d4cTuq::v0.11.0
  29:     0x2ba66f7773f0 - driver::driver::compile_input::hc5634315c7a145c8n8p::v0.11.0
  30:     0x2ba66f82af60 - driver::run_compiler::hae575e8dc5c8f87dVTs::v0.11.0
  31:     0x2ba66f82ae70 - driver::main_args::closure.117607
  32:     0x2ba66f83f010 - task::TaskBuilder<S>::try_future::closure.118749
  33:     0x2ba66f83ed00 - task::TaskBuilder<S>::spawn_internal::closure.118726
  34:     0x2ba66e9c0f40 - task::spawn_opts::closure.7519
  35:     0x2ba6719a01b0 - rust_try
  36:     0x2ba671944290 - unwind::try::hd5f4c2617b203e57BNd::v0.11.0
  37:     0x2ba671943fd0 - task::Task::run::headae25e22673ff0XYc::v0.11.0
  38:     0x2ba66e9c0d00 - task::spawn_opts::closure.7465
  39:     0x2ba671946af0 - thread::thread_start::hf8076cce1f95d94cvld::v0.11.0
  40:     0x2ba67271f0c0 - start_thread
  41:     0x2ba671d432d9 - __clone
  42:                0x0 - <unknown>

make: *** [rfmod] Error 101
