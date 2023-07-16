
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:     0x7f351f0bdf1f - sys::backtrace::write::hf79a3da4fdecb8a0OBA
   2:     0x7f351f0e8c32 - panicking::on_panic::h9f64f4c69e19f194hHJ
   3:     0x7f351f01deda - rt::unwind::begin_unwind_inner::h37f4496c980fe936knJ
   4:     0x7f351c44eabd - rt::unwind::begin_unwind::h8320268356453106285
   5:     0x7f351c44ea63 - diagnostic::SpanHandler::span_bug::h83c8af232eaba6a9h0D
   6:     0x7f351cd2aca3 - session::Session::span_bug::h857b2c7ae23c9286ISp
   7:     0x7f351e84880c - trans::debuginfo::scope_metadata::hac54dfdbdcd04cd9SjE
   8:     0x7f351e75b408 - trans::debuginfo::set_source_location::h1067a74086ed9dd48MD
   9:     0x7f351e70fe42 - trans::expr::trans_into::h95c6d2681fdd2548znh
  10:     0x7f351e710109 - trans::expr::trans_into::h95c6d2681fdd2548znh
  11:     0x7f351e77267f - trans::expr::trans_uniq_expr::h5f082eea62818f84ukj
  12:     0x7f351e7732ef - trans::expr::trans_unary::h4412379888608420Jgj
  13:     0x7f351e75d80e - trans::expr::trans_unadjusted::hfd3a5e1b5cbe37d5z4h
  14:     0x7f351e710417 - trans::expr::trans_into::h95c6d2681fdd2548znh
  15:     0x7f351e7906f9 - trans::expr::trans_adt::h1af69b9b4e52152aO6i
  16:     0x7f351e7931af - trans::expr::trans_struct::closure.42069
  17:     0x7f351e77d016 - trans::expr::trans_struct::hcae8f9103f3460d5K2i
  18:     0x7f351e75f53d - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  19:     0x7f351e7103f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  20:     0x7f351e711227 - trans::controlflow::trans_block::h3e86dfa8c58560e6b5d
  21:     0x7f351e7e8821 - trans::base::trans_closure::hab3cc3c679d5ff23Kkt
  22:     0x7f351e6f9b08 - trans::base::trans_fn::he0569b8eb832adf9Dvt
  23:     0x7f351e6fb5a0 - trans::monomorphize::monomorphic_fn::hc1b7393dd1dc77f1usd
  24:     0x7f351e740e4e - trans::callee::trans_fn_ref_with_substs::hd01acb4398310d154kg
  25:     0x7f351e73f43e - trans::callee::trans_fn_ref::hb48e614c9b6dd9bcE9f
  26:     0x7f351e73c88d - trans::callee::trans::ha56f4fe94448e6baVYf
  27:     0x7f351e752fbb - trans::callee::trans_call_inner::h9722042290657949952
  28:     0x7f351e75f9e0 - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  29:     0x7f351e7103f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  30:     0x7f351e7906f9 - trans::expr::trans_adt::h1af69b9b4e52152aO6i
  31:     0x7f351e7931af - trans::expr::trans_struct::closure.42069
  32:     0x7f351e77d016 - trans::expr::trans_struct::hcae8f9103f3460d5K2i
  33:     0x7f351e75f53d - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  34:     0x7f351e7103f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  35:     0x7f351e820967 - trans::_match::mk_binding_alloca::h13035368140960659810
  36:     0x7f351e70f7fd - trans::base::init_local::h1e7c96bb7077440dczs
  37:     0x7f351e710f02 - trans::controlflow::trans_block::h3e86dfa8c58560e6b5d
  38:     0x7f351e7e8821 - trans::base::trans_closure::hab3cc3c679d5ff23Kkt
  39:     0x7f351e6f9b08 - trans::base::trans_fn::he0569b8eb832adf9Dvt
  40:     0x7f351e6f6241 - trans::base::trans_item::h48fc370b7d259ac7vTt
  41:     0x7f351e6f6078 - trans::base::trans_item::h48fc370b7d259ac7vTt
  42:     0x7f351e6f6078 - trans::base::trans_item::h48fc370b7d259ac7vTt
  43:     0x7f351e7f17ec - trans::base::trans_crate::hc92be67ede893c70GPu
  44:     0x7f351f6f6e83 - driver::phase_4_translate_to_llvm::h9904f5d5fc3fb761rNa
  45:     0x7f351f6d283f - driver::compile_input::h3913ff7013f0c056Iba
  46:     0x7f351f79acb7 - run_compiler::h28a4446bae1034e7H5b
  47:     0x7f351f798829 - thunk::F.Invoke<A, R>::invoke::h6503055919709693733
  48:     0x7f351f7974a0 - rt::unwind::try::try_fn::h1384674024000742916
  49:     0x7f351f158de8 - rust_try_inner
  50:     0x7f351f158dd5 - rust_try
  51:     0x7f351f797c3f - thunk::F.Invoke<A, R>::invoke::h5780663349966142752
  52:     0x7f351f0d3965 - sys::thread::thread_start::h4ab695857833a5dar8E
  53:     0x7f3518f2e181 - start_thread
  54:     0x7f351ec8e47c - __clone
  55:                0x0 - <unknown>

