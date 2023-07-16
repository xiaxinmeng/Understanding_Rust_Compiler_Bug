
% RUST_BACKTRACE=1 x86_64-unknown-linux-gnu/stage1/bin/rustc src/test/run-pass/byte-literals.rs           
error: internal compiler error: only scalars and strings supported in compare_values
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/simon/projects/rust/src/libsyntax/diagnostic.rs:162
stack backtrace:
   1:     0x7fc48053d990 - rt::backtrace::imp::write::h90057ac788624522Zxx::v0.11.0.pre
   2:     0x7fc48053c810 - rt::unwind::print_failure::hacc32f6935eb51fahax::v0.11.0.pre
   3:     0x7fc4804c5c90 - rt::unwind::begin_unwind_inner::h14c630382c4f99faA7w::v0.11.0.pre
   4:     0x7fc47ee9a180 - rt::unwind::begin_unwind::h17887864392954574263::v0.11.0.pre
   5:     0x7fc47ee9abc0 - diagnostic::Handler::bug::h501233fb2e27e129Q4b::v0.11.0.pre
   6:     0x7fc48106ce50 - driver::session::Session::bug::hc9bec8c11d27abd5lBn::v0.11.0.pre
   7:     0x7fc48118a0a0 - middle::trans::_match::compile_submatch_continue::hb1b763876f7b6a0fDIs::v0.11.0.pre
   8:     0x7fc481185cb0 - middle::trans::_match::compile_submatch::hdd5760e69afb2da7VAs::v0.11.0.pre
   9:     0x7fc4810f79b0 - middle::trans::_match::trans_match::hab25f232b8c96245P8s::v0.11.0.pre
  10:     0x7fc4810e2a00 - middle::trans::expr::trans_rvalue_dps_unadjusted::hc1f78cf79e9af092Oaf::v0.11.0.pre
  11:     0x7fc4810a4870 - middle::trans::expr::trans_into::hca951bd886811312Wke::v0.11.0.pre
  12:     0x7fc4810a4e10 - middle::trans::controlflow::trans_block::hbb0d00fa8b95087f45a::v0.11.0.pre
  13:     0x7fc481162c00 - middle::trans::base::trans_closure::h76636cac11c7ffe7c8o::v0.11.0.pre
  14:     0x7fc48106de30 - middle::trans::base::trans_fn::h03088bac44d6887fzgp::v0.11.0.pre
  15:     0x7fc481067620 - middle::trans::base::trans_item::h06e2c19a7712c6f6nxp::v0.11.0.pre
  16:     0x7fc4811727a0 - middle::trans::base::trans_crate::ha0e861637bbdf4b3oqq::v0.11.0.pre
  17:     0x7fc481926990 - driver::driver::phase_4_translate_to_llvm::h08f08681cb45579dILm::v0.11.0.pre
  18:     0x7fc48191c0b0 - driver::driver::compile_input::hf69199098e698a1fjtm::v0.11.0.pre
  19:     0x7fc4819e9120 - driver::run_compiler::he195e2917873521527o::v0.11.0.pre
  20:     0x7fc4819e9040 - driver::main_args::closure.94877
  21:     0x7fc4819fe360 - driver::monitor::closure.95948
  22:     0x7fc4819f9740 - task::TaskBuilder::try::closure.95711
  23:     0x7fc480a7a2c0 - task::spawn_opts::closure.7843
  24:     0x7fc480536b30 - rt::task::Task::run::closure.25244
  25:     0x7fc4805a9b50 - rust_try
  26:     0x7fc48053c2e0 - rt::unwind::try::he8bf8bbb372515cbxXw::v0.11.0.pre
  27:     0x7fc4805369b0 - rt::task::Task::run::h15804127363e4efcoVu::v0.11.0.pre
  28:     0x7fc480a7a070 - task::spawn_opts::closure.7816
  29:     0x7fc48053abc0 - rt::thread::thread_start::h9c27125171789b173Ev::v0.11.0.pre
  30:     0x7fc47dfcb060 - start_thread
  31:     0x7fc480182489 - __clone
  32:                0x0 - <unknown>
