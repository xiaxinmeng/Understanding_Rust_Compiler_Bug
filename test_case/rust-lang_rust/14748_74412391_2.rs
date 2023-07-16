
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Expected("Expected label \"ty\" but found \"predicates\"")', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/result.rs:743

stack backtrace:
   1:     0x7fc30de91490 - sys::backtrace::write::h49b06c4e5fa1765dZTA
   2:     0x7fc30deb6120 - failure::on_fail::h128a61ca90111ec2JFJ
   3:     0x7fc30de0d350 - rt::unwind::begin_unwind_inner::h142cc6242e7988934jJ
   4:     0x7fc30de0de60 - rt::unwind::begin_unwind_fmt::h9f6d3c7c640c0421AiJ
   5:     0x7fc30deb5f80 - rust_begin_unwind
   6:     0x7fc30df00540 - panicking::panic_fmt::h21faf6f60b06c77cSvw
   7:     0x7fc30be50950 - middle::astencode::reader..Decoder<'a>.rbml_decoder_decoder_helpers<'tcx>::read_type_scheme::hfbdcf75b538c3875okc
   8:     0x7fc30be709e0 - middle::astencode::decode_side_tables::closure.75211
   9:     0x7fc30bccd4f0 - middle::astencode::decode_inlined_item::h1be258f0a7eb2ea8Xqa
  10:     0x7fc30caf6b20 - trans::inline::instantiate_inline::closure.40385
  11:     0x7fc30c0ed9d0 - metadata::decoder::maybe_get_item_ast::h727a4975347bffd4JMk
  12:     0x7fc30bf10bd0 - metadata::csearch::maybe_get_item_ast::h1ce89de91301f293Egn
  13:     0x7fc30caf50b0 - trans::inline::instantiate_inline::h2ef566630855f9886nd
  14:     0x7fc30cb41030 - trans::callee::trans_fn_ref_with_substs::h5afdf505fb7c2ad0DDg
  15:     0x7fc30cb3f970 - trans::callee::trans_fn_ref::h779769ec72c3a506Mrg
  16:     0x7fc30cb3cb40 - trans::callee::trans::haf8f9c5fe9575300rgg
  17:     0x7fc30cb535a0 - trans::callee::trans_call_inner::h4488088470999471523
  18:     0x7fc30cb5bde0 - trans::expr::trans_rvalue_dps_unadjusted::h8e987a8cff8d69eeYZi
  19:     0x7fc30cb5b200 - trans::expr::trans_unadjusted::h5a418a30ac3a6b40tri
  20:     0x7fc30cb12ea0 - trans::expr::trans::hf1cde9ec5fb37335sJh
  21:     0x7fc30cb4d030 - trans::callee::trans_args::hce224e809afff6ae7ih
  22:     0x7fc30cb535a0 - trans::callee::trans_call_inner::h4488088470999471523
  23:     0x7fc30cb5bde0 - trans::expr::trans_rvalue_dps_unadjusted::h8e987a8cff8d69eeYZi
  24:     0x7fc30cb11d10 - trans::expr::trans_into::h30d9a8cb700b6c4cYFh
  25:     0x7fc30cb111b0 - trans::controlflow::trans_stmt_semi::h3cef538536ab6888Zde
  26:     0x7fc30cb12320 - trans::controlflow::trans_block::h710fedd9b6aa6013Qee
  27:     0x7fc30cbdfa30 - trans::base::trans_closure::he63c8fb49d6fa9e5ofu
  28:     0x7fc30cafd280 - trans::base::trans_fn::h16e5caa2531e2efeEqu
  29:     0x7fc30caf8420 - trans::base::trans_item::h04c6251e8dd304b9tPu
  30:     0x7fc30cbe7100 - trans::base::trans_crate::he0f846e22b2c2665NMv
  31:     0x7fc30e4c4040 - driver::phase_4_translate_to_llvm::he7c9038b456ca0d5wPa
  32:     0x7fc30e49cf60 - driver::compile_input::ha2540a3e586cc4ecEba
  33:     0x7fc30e570d60 - run_compiler::h15054257f14a11945bc
  34:     0x7fc30e56f3c0 - thunk::F.Invoke<A, R>::invoke::h12452595225713592450
  35:     0x7fc30e56e2b0 - rt::unwind::try::try_fn::h15446000090453491186
  36:     0x7fc30df22810 - rust_try_inner
  37:     0x7fc30df22800 - rust_try
  38:     0x7fc30e56e560 - thunk::F.Invoke<A, R>::invoke::h7454813734411192152
  39:     0x7fc30dea1d90 - sys::thread::thread_start::h76415ad3898ce7eaaOE
  40:     0x7fc307e562b0 - start_thread
  41:     0x7fc30da95249 - __clone
  42:                0x0 - <unknown>
