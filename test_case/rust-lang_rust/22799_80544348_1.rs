
lib.rs:1:1: 1:1 error: internal compiler error: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(core::marker::Sized)),depth=1),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(Wrapper)),depth=1),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(IsA<_>)),depth=1),Ambiguity), FulfillmentError(Obligation(predicate=Binder(ProjectionPredicate(<_ as Wrapper>::Inner, i8)),depth=2),Ambiguity)]` fulfilling during trans
lib.rs:1 #![feature(core)]
         ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:     0x7f76c6609e2f - sys::backtrace::write::h947ec7660f7e4c8cSBC
   2:     0x7f76c662ff82 - panicking::on_panic::h23ca8edeaea6c81cLMI
   3:     0x7f76c6564e8a - rt::unwind::begin_unwind_inner::h47bdb490e575cf7eRsI
   4:     0x7f76c39db94d - rt::unwind::begin_unwind::h18314895764715619195
   5:     0x7f76c39db8f3 - diagnostic::SpanHandler::span_bug::h8ef69e7b1a01e686EaB
   6:     0x7f76c4294263 - session::Session::span_bug::h45f93f11f39024fbNsn
   7:     0x7f76c5cf5cc7 - trans::common::fulfill_obligation::h1573f2a041dd4f61ykl
   8:     0x7f76c5c9175b - trans::meth::trans_static_method_callee::hd5b7480b401c3e4aA7x
   9:     0x7f76c5c8e200 - trans::callee::trans::hb832566848979f51eYf
  10:     0x7f76c5ca4b7b - trans::callee::trans_call_inner::h13086090426327833152
  11:     0x7f76c5cb1820 - trans::expr::trans_rvalue_dps_unadjusted::h8c6df01d48428517xzi
  12:     0x7f76c5c61cf0 - trans::expr::trans_into::h9649503c823db95eSmh
  13:     0x7f76c5c60fe4 - trans::controlflow::trans_stmt_semi::hc954ae953b072232G5d
  14:     0x7f76c5c62ab0 - trans::controlflow::trans_block::h4c660619c3bd7816t6d
  15:     0x7f76c5d3a611 - trans::base::trans_closure::hcfabd7fd269607bekjt
  16:     0x7f76c5c4b238 - trans::base::trans_fn::hc85106bcc218fcb4dut
  17:     0x7f76c5c4c90b - trans::monomorphize::monomorphic_fn::h22691e7b0d2d168c2td
  18:     0x7f76c5c92a0e - trans::callee::trans_fn_ref_with_substs::h46ce93a241a2e351nkg
  19:     0x7f76c5caaa75 - trans::meth::trans_method_callee::h2c272c3ddb1fcd8cr2x
  20:     0x7f76c5ca64a9 - trans::callee::trans_call_inner::h320915813152599062
  21:     0x7f76c5cb05c9 - trans::expr::trans_rvalue_dps_unadjusted::h8c6df01d48428517xzi
  22:     0x7f76c5c61cf0 - trans::expr::trans_into::h9649503c823db95eSmh
  23:     0x7f76c5c60fe4 - trans::controlflow::trans_stmt_semi::hc954ae953b072232G5d
  24:     0x7f76c5c62ab0 - trans::controlflow::trans_block::h4c660619c3bd7816t6d
  25:     0x7f76c5d3a611 - trans::base::trans_closure::hcfabd7fd269607bekjt
  26:     0x7f76c5c4b238 - trans::base::trans_fn::hc85106bcc218fcb4dut
  27:     0x7f76c5c46ef1 - trans::base::trans_item::h167dcb48771cb88c5Rt
  28:     0x7f76c5d4373c - trans::base::trans_crate::h947378b1d60dae25wOu
  29:     0x7f76c6c3c6e3 - driver::phase_4_translate_to_llvm::h8e4d85cc4e2e15a5gOa
  30:     0x7f76c6c182d3 - driver::compile_input::hf1ccb6f79c82b5e3Nba
  31:     0x7f76c6cd3827 - run_compiler::h5744096088d771cbu2b
  32:     0x7f76c6cd1459 - thunk::F.Invoke<A, R>::invoke::h2151708636338439058
  33:     0x7f76c6cd0110 - rt::unwind::try::try_fn::h17480117432322948657
  34:     0x7f76c66a19c8 - rust_try_inner
  35:     0x7f76c66a19b5 - rust_try
  36:     0x7f76c6cd088a - thunk::F.Invoke<A, R>::invoke::h4030886730675850734
  37:     0x7f76c661e9a5 - sys::thread::thread_start::he7ae8e81d7f8a1f4x4G
  38:     0x7f76c04bbee4 - start_thread
  39:     0x7f76c61d7d1c - clone
  40:                0x0 - <unknown>
