
  Compiling env_logger v0.2.2
   Compiling docopt v0.6.44
   Compiling toml v0.1.18
/Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libstd/sync/mutex.rs:177:37: 180:2 error: internal compiler error: debuginfo: Could not find scope info for node NodeExpr(Expr { id: 8164, node: ExprStruct(Path { span: Span { lo: BytePos(4566645), hi: BytePos(4566656), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: StaticMutex#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }, [Field { ident: Spanned { node: lock#0, span: Span { lo: BytePos(1770102), hi: BytePos(1770106), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 8165, node: ExprPath(None, Path { span: Span { lo: BytePos(4566669), hi: BytePos(4566684), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: sys#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: MUTEX_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(4566669), hi: BytePos(4566684), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(4566663), hi: BytePos(4566684), expn_id: ExpnId(4294967295) } }, Field { ident: Spanned { node: poison#0, span: Span { lo: BytePos(1770129), hi: BytePos(1770135), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 8166, node: ExprPath(None, Path { span: Span { lo: BytePos(4566698), hi: BytePos(4566715), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: poison#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: FLAG_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(4566698), hi: BytePos(4566715), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(4566690), hi: BytePos(4566715), expn_id: ExpnId(4294967295) } }], None), span: Span { lo: BytePos(4566645), hi: BytePos(4566718), expn_id: ExpnId(4294967295) } })
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:        0x110798b52 - sys::backtrace::write::h6b0889bc971c1ad3IDA
   2:        0x1107c7844 - panicking::on_panic::h0fcd1d74630df38dKsJ
   3:        0x1106e5c67 - rt::unwind::begin_unwind_inner::h56b969a14fc61916gbJ
   4:        0x10fe7095e - rt::unwind::begin_unwind::h7097931347514503270
   5:        0x10fe7090b - diagnostic::SpanHandler::span_bug::hd809ae68b3d05ee6h0D
   6:        0x10d639d7c - session::Session::span_bug::h5bc4aa3cff230a98ISp
   7:        0x10d35b134 - trans::debuginfo::scope_metadata::hbc8249b53299cdbdSjE
   8:        0x10d265fb9 - trans::debuginfo::set_source_location::h27a07f90a8aeec578MD
   9:        0x10d217925 - trans::expr::trans_into::h4a29f680db4b3526znh
  10:        0x10d217c45 - trans::expr::trans_into::h4a29f680db4b3526znh
  11:        0x10d27e2c0 - trans::expr::trans_uniq_expr::h9953b7ba5686a321ukj
  12:        0x10d27f053 - trans::expr::trans_unary::h112ae451f6aff215Jgj
  13:        0x10d2686fc - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  14:        0x10d217f79 - trans::expr::trans_into::h4a29f680db4b3526znh
  15:        0x10d29d220 - trans::expr::trans_adt::h73a11f7497464a42O6i
  16:        0x10d29fd52 - trans::expr::trans_struct::closure.42052
  17:        0x10d2893cb - trans::expr::trans_struct::h4294120fb3439252K2i
  18:        0x10d26a6be - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  19:        0x10d217f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  20:        0x10d218e29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  21:        0x10d2f7da9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  22:        0x10d200645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  23:        0x10d2021c3 - trans::monomorphize::monomorphic_fn::h8c4abbed98c102adusd
  24:        0x10d24a58b - trans::callee::trans_fn_ref_with_substs::heb22ab8904f67bf64kg
  25:        0x10d248b0e - trans::callee::trans_fn_ref::hb2be9d8aec49d9c4E9f
  26:        0x10d2461d0 - trans::callee::trans::ha12ccf10898e9b03VYf
  27:        0x10d25d24d - trans::callee::trans_call_inner::h11221320451677754653
  28:        0x10d26abec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  29:        0x10d217f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  30:        0x10d29d220 - trans::expr::trans_adt::h73a11f7497464a42O6i
  31:        0x10d29fd52 - trans::expr::trans_struct::closure.42052
  32:        0x10d2893cb - trans::expr::trans_struct::h4294120fb3439252K2i
  33:        0x10d26a6be - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  34:        0x10d26851c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  35:        0x10d21987b - trans::expr::trans::h60a968a47a19defeHth
  36:        0x10d2587e1 - trans::callee::trans_args::h43fd254006ea96e2m1g
  37:        0x10d25e34e - trans::callee::trans_call_inner::h11221320451677754653
  38:        0x10d26abec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  39:        0x10d26851c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  40:        0x10d21987b - trans::expr::trans::h60a968a47a19defeHth
  41:        0x10d2179ca - trans::expr::trans_into::h4a29f680db4b3526znh
  42:        0x10d218e29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  43:        0x10d26a050 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  44:        0x10d217f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  45:        0x10d218e29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  46:        0x10d2f7da9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  47:        0x10d29dcc3 - trans::closure::trans_closure_expr::hc7e51f7f71e8adb0flx
  48:        0x10d26ac42 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  49:        0x10d268647 - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  50:        0x10d21987b - trans::expr::trans::h60a968a47a19defeHth
  51:        0x10d2587e1 - trans::callee::trans_args::h43fd254006ea96e2m1g
  52:        0x10d25e34e - trans::callee::trans_call_inner::h11221320451677754653
  53:        0x10d26abec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  54:        0x10d217f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  55:        0x10d218e29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  56:        0x10d2f7da9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  57:        0x10d200645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  58:        0x10d1fc0bd - trans::base::trans_item::h8ce277cc13b040a3vTt
  59:        0x10d300ecb - trans::base::trans_crate::hb69c466e59630a14GPu
  60:        0x10cc38ec7 - driver::phase_4_translate_to_llvm::he4e045cdc3897b4crNa
  61:        0x10cc147f2 - driver::compile_input::h25d96a14ec26932cIba
  62:        0x10cce672e - run_compiler::h15c3ea085a111a6fH5b
  63:        0x10cce3ac7 - thunk::F.Invoke<A, R>::invoke::h11905936354613679413
  64:        0x10cce260f - rt::unwind::try::try_fn::h4025213131938170097
  65:        0x110844628 - rust_try_inner
  66:        0x110844615 - rust_try
  67:        0x10cce2e48 - thunk::F.Invoke<A, R>::invoke::h13725635537516470714
  68:        0x1107b00a2 - sys::thread::thread_start::h1120d6d2e105321012E
  69:     0x7fff8d0682fb - _pthread_body
  70:     0x7fff8d068278 - _pthread_start
