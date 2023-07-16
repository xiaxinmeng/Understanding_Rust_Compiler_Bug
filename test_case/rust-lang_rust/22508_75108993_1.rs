
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Expected("Expected label \"ty\" but found \"predicates\"")', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/result.rs:743

stack backtrace:
   1:     0x7fa2e9c85750 - sys::backtrace::write::h49b06c4e5fa1765dZTA
   2:     0x7fa2e9caa3e0 - failure::on_fail::h128a61ca90111ec2JFJ
   3:     0x7fa2e9c02350 - rt::unwind::begin_unwind_inner::h142cc6242e7988934jJ
   4:     0x7fa2e9c02e60 - rt::unwind::begin_unwind_fmt::h9f6d3c7c640c0421AiJ
   5:     0x7fa2e9caa240 - rust_begin_unwind
   6:     0x7fa2e9cf47a0 - panicking::panic_fmt::h21faf6f60b06c77cSvw
   7:     0x7fa2e7c40160 - middle::astencode::reader..Decoder<'a>.rbml_decoder_decoder_helpers<'tcx>::read_type_scheme::h8e3dba270454c7e2okc
   8:     0x7fa2e7c601f0 - middle::astencode::decode_side_tables::closure.75145
   9:     0x7fa2e7ac04e0 - middle::astencode::decode_inlined_item::h068d532261c60680Xqa
  10:     0x7fa2e88d8df0 - trans::inline::instantiate_inline::closure.40362
  11:     0x7fa2e7ed28c0 - metadata::decoder::maybe_get_item_ast::hbd54e15ceac79354JMk
  12:     0x7fa2e7cfc770 - metadata::csearch::maybe_get_item_ast::h22d6fe7c2dd8b6ffEgn
  13:     0x7fa2e88d7380 - trans::inline::instantiate_inline::h08b37fe16127fdf76nd
  14:     0x7fa2e8922120 - trans::callee::trans_fn_ref_with_substs::h4a9e9231881dd945DDg
  15:     0x7fa2e8936ec0 - trans::meth::trans_method_callee::h96316b4b826569cf9bz
  16:     0x7fa2e8935a10 - trans::callee::trans_call_inner::h10290956782341029294
  17:     0x7fa2e893c920 - trans::expr::trans_rvalue_dps_unadjusted::he612c9f9091d08b2YZi
  18:     0x7fa2e893bd40 - trans::expr::trans_unadjusted::h0a35d1e10b03cd27tri
  19:     0x7fa2e88f4a20 - trans::expr::trans::h353ab30c52c622dbsJh
  20:     0x7fa2e8953470 - trans::expr::trans_unary::haee3f4e6a826b679oIj
  21:     0x7fa2e893bd40 - trans::expr::trans_unadjusted::h0a35d1e10b03cd27tri
  22:     0x7fa2e88f4a20 - trans::expr::trans::h353ab30c52c622dbsJh
  23:     0x7fa2e8942130 - trans::expr::trans_rvalue_stmt_unadjusted::hdc1fcb88faa3f8006Ti
  24:     0x7fa2e88f3890 - trans::expr::trans_into::h9f3ed4622867868fYFh
  25:     0x7fa2e88f2d30 - trans::controlflow::trans_stmt_semi::h18d9e1f6ee095b69Zde
  26:     0x7fa2e88f3ea0 - trans::controlflow::trans_block::h7a9017bd6dd61f5cQee
  27:     0x7fa2e8942130 - trans::expr::trans_rvalue_stmt_unadjusted::hdc1fcb88faa3f8006Ti
  28:     0x7fa2e88f3890 - trans::expr::trans_into::h9f3ed4622867868fYFh
  29:     0x7fa2e88f2d30 - trans::controlflow::trans_stmt_semi::h18d9e1f6ee095b69Zde
  30:     0x7fa2e88f3ea0 - trans::controlflow::trans_block::h7a9017bd6dd61f5cQee
  31:     0x7fa2e89bf180 - trans::base::trans_closure::hbc1e838a851464b4ofu
  32:     0x7fa2e88df550 - trans::base::trans_fn::h89cdd4576220c5b2Equ
  33:     0x7fa2e88da6f0 - trans::base::trans_item::h821a1d8c2f3bd0d2tPu
  34:     0x7fa2e89c3320 - trans::base::trans_mod::h004afd65bfc8913foVu
  35:     0x7fa2e88da6f0 - trans::base::trans_item::h821a1d8c2f3bd0d2tPu
  36:     0x7fa2e89c6730 - trans::base::trans_crate::hbc3532ac87923bc6NMv
  37:     0x7fa2ea2b8280 - driver::phase_4_translate_to_llvm::he2aa9fc36a75551bwPa
  38:     0x7fa2ea291190 - driver::compile_input::h2199f07dd067a038Eba
  39:     0x7fa2ea362360 - run_compiler::h7807f450e003c3f65bc
  40:     0x7fa2ea3609c0 - thunk::F.Invoke<A, R>::invoke::h10965480741658827823
  41:     0x7fa2ea35f8b0 - rt::unwind::try::try_fn::h9927690981525442750
  42:     0x7fa2e9d16a70 - rust_try_inner
  43:     0x7fa2e9d16a60 - rust_try
  44:     0x7fa2ea35fb60 - thunk::F.Invoke<A, R>::invoke::h17448596242061042531
  45:     0x7fa2e9c96050 - sys::thread::thread_start::h76415ad3898ce7eaaOE
  46:     0x7fa2e3c610c0 - start_thread
  47:     0x7fa2e9879fd9 - __clone
  48:                0x0 - <unknown>
