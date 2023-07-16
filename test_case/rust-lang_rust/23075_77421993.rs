
/Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libstd/sync/mutex.rs:177:37: 180:2 error: internal compiler error: debuginfo: Could not find scope info for node NodeExpr(Expr { id: 99160, node: ExprStruct(Path { span: Span { lo: BytePos(4745586), hi: BytePos(4745597), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: StaticMutex#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }, [Field { ident: Spanned { node: lock#0, span: Span { lo: BytePos(1770102), hi: BytePos(1770106), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 99161, node: ExprPath(None, Path { span: Span { lo: BytePos(4745610), hi: BytePos(4745625), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: sys#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: MUTEX_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(4745610), hi: BytePos(4745625), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(4745604), hi: BytePos(4745625), expn_id: ExpnId(4294967295) } }, Field { ident: Spanned { node: poison#0, span: Span { lo: BytePos(1770129), hi: BytePos(1770135), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 99162, node: ExprPath(None, Path { span: Span { lo: BytePos(4745639), hi: BytePos(4745656), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: poison#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: FLAG_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(4745639), hi: BytePos(4745656), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(4745631), hi: BytePos(4745656), expn_id: ExpnId(4294967295) } }], None), span: Span { lo: BytePos(4745586), hi: BytePos(4745659), expn_id: ExpnId(4294967295) } })
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:        0x108261b52 - sys::backtrace::write::h6b0889bc971c1ad3IDA
   2:        0x108290844 - panicking::on_panic::h0fcd1d74630df38dKsJ
   3:        0x1081aec67 - rt::unwind::begin_unwind_inner::h56b969a14fc61916gbJ
   4:        0x10793b95e - rt::unwind::begin_unwind::h7097931347514503270
   5:        0x10793b90b - diagnostic::SpanHandler::span_bug::hd809ae68b3d05ee6h0D
   6:        0x105110d7c - session::Session::span_bug::h5bc4aa3cff230a98ISp
   7:        0x104e34134 - trans::debuginfo::scope_metadata::hbc8249b53299cdbdSjE
   8:        0x104d3efb9 - trans::debuginfo::set_source_location::h27a07f90a8aeec578MD
   9:        0x104cf0925 - trans::expr::trans_into::h4a29f680db4b3526znh
  10:        0x104cf0c45 - trans::expr::trans_into::h4a29f680db4b3526znh
  11:        0x104d572c0 - trans::expr::trans_uniq_expr::h9953b7ba5686a321ukj
  12:        0x104d58053 - trans::expr::trans_unary::h112ae451f6aff215Jgj
  13:        0x104d416fc - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  14:        0x104cf0f79 - trans::expr::trans_into::h4a29f680db4b3526znh
  15:        0x104d76220 - trans::expr::trans_adt::h73a11f7497464a42O6i
  16:        0x104d78d52 - trans::expr::trans_struct::closure.42052
  17:        0x104d623cb - trans::expr::trans_struct::h4294120fb3439252K2i
  18:        0x104d436be - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  19:        0x104cf0f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  20:        0x104cf1e29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  21:        0x104dd0da9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  22:        0x104cd9645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  23:        0x104cdb1c3 - trans::monomorphize::monomorphic_fn::h8c4abbed98c102adusd
  24:        0x104d2358b - trans::callee::trans_fn_ref_with_substs::heb22ab8904f67bf64kg
  25:        0x104d21b0e - trans::callee::trans_fn_ref::hb2be9d8aec49d9c4E9f
  26:        0x104d1f1d0 - trans::callee::trans::ha12ccf10898e9b03VYf
  27:        0x104d3624d - trans::callee::trans_call_inner::h11221320451677754653
  28:        0x104d43bec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  29:        0x104d4151c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  30:        0x104cf287b - trans::expr::trans::h60a968a47a19defeHth
  31:        0x104d317e1 - trans::callee::trans_args::h43fd254006ea96e2m1g
  32:        0x104d3734e - trans::callee::trans_call_inner::h11221320451677754653
  33:        0x104d43bec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  34:        0x104cf0f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  35:        0x104d76220 - trans::expr::trans_adt::h73a11f7497464a42O6i
  36:        0x104d78d52 - trans::expr::trans_struct::closure.42052
  37:        0x104d623cb - trans::expr::trans_struct::h4294120fb3439252K2i
  38:        0x104d436be - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  39:        0x104cf0f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  40:        0x104d76220 - trans::expr::trans_adt::h73a11f7497464a42O6i
  41:        0x104d30545 - trans::base::trans_named_tuple_constructor::h1ca92235818072b2czt
  42:        0x104d366d1 - trans::callee::trans_call_inner::h11221320451677754653
  43:        0x104d43bec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  44:        0x104cf0f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  45:        0x104cf1e29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  46:        0x104dd0da9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  47:        0x104cd9645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  48:        0x104cd5aa6 - trans::base::trans_item::h8ce277cc13b040a3vTt
  49:        0x104cd58a7 - trans::base::trans_item::h8ce277cc13b040a3vTt
  50:        0x104cd58a7 - trans::base::trans_item::h8ce277cc13b040a3vTt
  51:        0x104dd9ecb - trans::base::trans_crate::hb69c466e59630a14GPu
  52:        0x10470eec7 - driver::phase_4_translate_to_llvm::he4e045cdc3897b4crNa
  53:        0x1046ea7f2 - driver::compile_input::h25d96a14ec26932cIba
  54:        0x1047bc72e - run_compiler::h15c3ea085a111a6fH5b
  55:        0x1047b9ac7 - thunk::F.Invoke<A, R>::invoke::h11905936354613679413
  56:        0x1047b860f - rt::unwind::try::try_fn::h4025213131938170097
  57:        0x10830d628 - rust_try_inner
  58:        0x10830d615 - rust_try
  59:        0x1047b8e48 - thunk::F.Invoke<A, R>::invoke::h13725635537516470714
  60:        0x1082790a2 - sys::thread::thread_start::h1120d6d2e105321012E
  61:     0x7fff8ab44267 - _pthread_body
  62:     0x7fff8ab441e4 - _pthread_start

Could not compile `mysql`.
