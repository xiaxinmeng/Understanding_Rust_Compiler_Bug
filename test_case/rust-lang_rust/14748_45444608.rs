
[mvdnes@vm-arch rust-kernel (git:master)]$ RUST_BACKTRACE=1 make
rustc -L. -O --target i386-unknown-linux --crate-type lib -o main.o --emit obj main_x86.rs
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'assertion failed: start <= end', /build/rust-git/src/rust/src/libcore/slice.rs:540

stack backtrace:
   1:     0x7f6258c4b560 - rt::backtrace::imp::write::hf7f0193b62c10642jft::v0.11.0.pre
   2:     0x7f6258c52c90 - <unknown>
   3:     0x7f625affaf40 - unwind::begin_unwind_inner::hb191b1c2cc278613x2c::v0.11.0.pre
   4:     0x7f625affa9d0 - unwind::begin_unwind_fmt::hd9fbea452e71602eZZc::v0.11.0.pre
   5:     0x7f625affa990 - rust_begin_unwind
   6:     0x7f625b042750 - failure::begin_unwind::h4ff9994309231abcSZv::v0.11.0.pre
   7:     0x7f625b99d850 - middle::trans::callee::trans_fn_ref_with_vtables::hd8b242ee1b42ae55qze::v0.11.0.pre
   8:     0x7f625b998fc0 - middle::trans::callee::trans_fn_ref::h39af25c07898892b7se::v0.11.0.pre
   9:     0x7f625b9a8470 - middle::trans::meth::trans_method_callee::h658977e3db07169cfsw::v0.11.0.pre
  10:     0x7f625b9a8400 - <unknown>
  11:     0x7f625b9a65c0 - middle::trans::callee::trans_call_inner::h16ea36ba3bed6c30aYe::v0.11.0.pre
  12:     0x7f625b9a7d80 - middle::trans::callee::trans_method_call::hb940a5b8c40323edpTe::v0.11.0.pre
  13:     0x7f625b9b11f0 - <unknown>
  14:     0x7f625b9729b0 - middle::trans::expr::trans_into::h9f32507e10bb94f45zf::v0.11.0.pre
  15:     0x7f625b972f50 - middle::trans::controlflow::trans_block::h91c630cfb7b105c2Elc::v0.11.0.pre
  16:     0x7f625ba30600 - middle::trans::base::trans_closure::hfe923026fff94ef6tlq::v0.11.0.pre
  17:     0x7f625b942a00 - middle::trans::base::trans_fn::hd03f25ae32818d32wtq::v0.11.0.pre
  18:     0x7f625b92c220 - middle::trans::inline::maybe_instantiate_inline::h4d0dfbbb4addff01zub::v0.11.0.pre
  19:     0x7f625b99d850 - middle::trans::callee::trans_fn_ref_with_vtables::hd8b242ee1b42ae55qze::v0.11.0.pre
  20:     0x7f625b998fc0 - middle::trans::callee::trans_fn_ref::h39af25c07898892b7se::v0.11.0.pre
  21:     0x7f625b9a8470 - middle::trans::meth::trans_method_callee::h658977e3db07169cfsw::v0.11.0.pre
  22:     0x7f625b9a8400 - <unknown>
  23:     0x7f625b9a65c0 - middle::trans::callee::trans_call_inner::h16ea36ba3bed6c30aYe::v0.11.0.pre
  24:     0x7f625b9a7d80 - middle::trans::callee::trans_method_call::hb940a5b8c40323edpTe::v0.11.0.pre
  25:     0x7f625b9b11f0 - <unknown>
  26:     0x7f625b9b0010 - <unknown>
  27:     0x7f625b974c80 - middle::trans::expr::trans::h3db050fd3914c9d01Df::v0.11.0.pre
  28:     0x7f625b972850 - middle::trans::expr::trans_to_lvalue::h0def187e2f6a431beWf::v0.11.0.pre
  29:     0x7f625b9bd070 - <unknown>
  30:     0x7f625b9b0010 - <unknown>
  31:     0x7f625b974c80 - middle::trans::expr::trans::h3db050fd3914c9d01Df::v0.11.0.pre
  32:     0x7f625b972850 - middle::trans::expr::trans_to_lvalue::h0def187e2f6a431beWf::v0.11.0.pre
  33:     0x7f625b9c3e00 - middle::trans::_match::trans_match::hed8b35749bd94b9bgmu::v0.11.0.pre
  34:     0x7f625b9b11f0 - <unknown>
  35:     0x7f625b9729b0 - middle::trans::expr::trans_into::h9f32507e10bb94f45zf::v0.11.0.pre
  36:     0x7f625b971c50 - middle::trans::controlflow::trans_stmt_semi::hde0b281f73b8ef7eLkc::v0.11.0.pre
  37:     0x7f625b971180 - middle::trans::controlflow::trans_stmt::hb5cb0c042b3019b1xgc::v0.11.0.pre
  38:     0x7f625b972f50 - middle::trans::controlflow::trans_block::h91c630cfb7b105c2Elc::v0.11.0.pre
  39:     0x7f625ba30600 - middle::trans::base::trans_closure::hfe923026fff94ef6tlq::v0.11.0.pre
  40:     0x7f625b942a00 - middle::trans::base::trans_fn::hd03f25ae32818d32wtq::v0.11.0.pre
  41:     0x7f625ba35e50 - middle::trans::meth::trans_impl::hf8b54cf6a401fbfftnw::v0.11.0.pre
  42:     0x7f625b93c1c0 - middle::trans::base::trans_item::h15a7d15d5a1d399aWJq::v0.11.0.pre
  43:     0x7f625b93c1c0 - middle::trans::base::trans_item::h15a7d15d5a1d399aWJq::v0.11.0.pre
  44:     0x7f625b93c1c0 - middle::trans::base::trans_item::h15a7d15d5a1d399aWJq::v0.11.0.pre
  45:     0x7f625ba407e0 - middle::trans::base::trans_crate::h85cec661243570133Cr::v0.11.0.pre
  46:     0x7f625c1ff5b0 - driver::driver::phase_4_translate_to_llvm::h1f0b6f1f8a7ca49eUao::v0.11.0.pre
  47:     0x7f625c1f4bd0 - driver::driver::compile_input::ha940156ba16b48ebvSn::v0.11.0.pre
  48:     0x7f625c2bcef0 - <unknown>
  49:     0x7f625c2bce10 - <unknown>
  50:     0x7f625c2d75d0 - <unknown>
  51:     0x7f625c2d2870 - <unknown>
  52:     0x7f625b342800 - <unknown>
  53:     0x7f625aff84f0 - <unknown>
  54:     0x7f625b05ced0 - rust_try
  55:     0x7f625affa530 - unwind::try::h5e1236a7876941deTQc::v0.11.0.pre
  56:     0x7f625aff8370 - task::Task::run::h10ae6b6004d94ab7qxc::v0.11.0.pre
  57:     0x7f625b3425a0 - <unknown>
  58:     0x7f6258c4a9b0 - <unknown>
  59:     0x7f6258981060 - start_thread
  60:     0x7f625acd1489 - __clone
  61:                0x0 - <unknown>

Makefile:25: recipe for target 'main.o' failed
make: *** [main.o] Error 101
