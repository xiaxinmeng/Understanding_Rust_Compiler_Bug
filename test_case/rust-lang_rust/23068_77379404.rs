
   Compiling type-directed-tdd-rust v0.1.0 (file:///Users/chen/MEGA/rust/type-directed-tdd-rust)
/Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libstd/sync/mutex.rs:177:37: 180:2 error: internal compiler error: debuginfo: Could not find scope info for node NodeExpr(Expr { id: 10933, node: ExprStruct(Path { span: Span { lo: BytePos(4562885), hi: BytePos(4562896), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: StaticMutex#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }, [Field { ident: Spanned { node: lock#0, span: Span { lo: BytePos(1770102), hi: BytePos(1770106), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 10934, node: ExprPath(None, Path { span: Span { lo: BytePos(4562909), hi: BytePos(4562924), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: sys#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: MUTEX_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(4562909), hi: BytePos(4562924), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(4562903), hi: BytePos(4562924), expn_id: ExpnId(4294967295) } }, Field { ident: Spanned { node: poison#0, span: Span { lo: BytePos(1770129), hi: BytePos(1770135), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 10935, node: ExprPath(None, Path { span: Span { lo: BytePos(4562938), hi: BytePos(4562955), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: poison#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: FLAG_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(4562938), hi: BytePos(4562955), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(4562930), hi: BytePos(4562955), expn_id: ExpnId(4294967295) } }], None), span: Span { lo: BytePos(4562885), hi: BytePos(4562958), expn_id: ExpnId(4294967295) } })
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:        0x10f348b52 - sys::backtrace::write::h6b0889bc971c1ad3IDA
   2:        0x10f377844 - panicking::on_panic::h0fcd1d74630df38dKsJ
   3:        0x10f295c67 - rt::unwind::begin_unwind_inner::h56b969a14fc61916gbJ
   4:        0x10ea1c95e - rt::unwind::begin_unwind::h7097931347514503270
   5:        0x10ea1c90b - diagnostic::SpanHandler::span_bug::hd809ae68b3d05ee6h0D
   6:        0x10c1fbd7c - session::Session::span_bug::h5bc4aa3cff230a98ISp
   7:        0x10bf20134 - trans::debuginfo::scope_metadata::hbc8249b53299cdbdSjE
   8:        0x10be2afb9 - trans::debuginfo::set_source_location::h27a07f90a8aeec578MD
   9:        0x10bddc925 - trans::expr::trans_into::h4a29f680db4b3526znh
  10:        0x10bddcc45 - trans::expr::trans_into::h4a29f680db4b3526znh
  11:        0x10be432c0 - trans::expr::trans_uniq_expr::h9953b7ba5686a321ukj
  12:        0x10be44053 - trans::expr::trans_unary::h112ae451f6aff215Jgj
  13:        0x10be2d6fc - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  14:        0x10bddcf79 - trans::expr::trans_into::h4a29f680db4b3526znh
  15:        0x10be62220 - trans::expr::trans_adt::h73a11f7497464a42O6i
  16:        0x10be64d52 - trans::expr::trans_struct::closure.42052
  17:        0x10be4e3cb - trans::expr::trans_struct::h4294120fb3439252K2i
  18:        0x10be2f6be - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  19:        0x10bddcf59 - trans::expr::trans_into::h4a29f680db4b3526znh
  20:        0x10bddde29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  21:        0x10bebcda9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  22:        0x10bdc5645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  23:        0x10bdc71c3 - trans::monomorphize::monomorphic_fn::h8c4abbed98c102adusd
  24:        0x10be0f58b - trans::callee::trans_fn_ref_with_substs::heb22ab8904f67bf64kg
  25:        0x10be0db0e - trans::callee::trans_fn_ref::hb2be9d8aec49d9c4E9f
  26:        0x10be0b1d0 - trans::callee::trans::ha12ccf10898e9b03VYf
  27:        0x10be2224d - trans::callee::trans_call_inner::h11221320451677754653
  28:        0x10be2fbec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  29:        0x10bddcf59 - trans::expr::trans_into::h4a29f680db4b3526znh
  30:        0x10be62220 - trans::expr::trans_adt::h73a11f7497464a42O6i
  31:        0x10be64d52 - trans::expr::trans_struct::closure.42052
  32:        0x10be4e3cb - trans::expr::trans_struct::h4294120fb3439252K2i
  33:        0x10be2f6be - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  34:        0x10bddcf59 - trans::expr::trans_into::h4a29f680db4b3526znh
  35:        0x10bef6d26 - trans::_match::mk_binding_alloca::h3448676421097350985
  36:        0x10bddc280 - trans::base::init_local::h08a9fcff749ad013czs
  37:        0x10bdddb02 - trans::controlflow::trans_block::h26a308528bb95051b5d
  38:        0x10bebcda9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  39:        0x10bdc5645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  40:        0x10bdc71c3 - trans::monomorphize::monomorphic_fn::h8c4abbed98c102adusd
  41:        0x10be0f58b - trans::callee::trans_fn_ref_with_substs::heb22ab8904f67bf64kg
  42:        0x10be0db0e - trans::callee::trans_fn_ref::hb2be9d8aec49d9c4E9f
  43:        0x10be0b1d0 - trans::callee::trans::ha12ccf10898e9b03VYf
  44:        0x10be2224d - trans::callee::trans_call_inner::h11221320451677754653
  45:        0x10be2fbec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  46:        0x10be2d51c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  47:        0x10bdde87b - trans::expr::trans::h60a968a47a19defeHth
  48:        0x10be1d7e1 - trans::callee::trans_args::h43fd254006ea96e2m1g
  49:        0x10be2334e - trans::callee::trans_call_inner::h11221320451677754653
  50:        0x10be2fbec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  51:        0x10be2d51c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  52:        0x10bdde87b - trans::expr::trans::h60a968a47a19defeHth
  53:        0x10be1d7e1 - trans::callee::trans_args::h43fd254006ea96e2m1g
  54:        0x10be2334e - trans::callee::trans_call_inner::h11221320451677754653
  55:        0x10be2fbec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  56:        0x10bddcf59 - trans::expr::trans_into::h4a29f680db4b3526znh
  57:        0x10bef6d26 - trans::_match::mk_binding_alloca::h3448676421097350985
  58:        0x10bddc280 - trans::base::init_local::h08a9fcff749ad013czs
  59:        0x10bdddb02 - trans::controlflow::trans_block::h26a308528bb95051b5d
  60:        0x10be2f050 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  61:        0x10bddcf59 - trans::expr::trans_into::h4a29f680db4b3526znh
  62:        0x10bede4e0 - trans::_match::trans_match_inner::he768a1b2ccde6888CIw
  63:        0x10be2efe3 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  64:        0x10be2d51c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  65:        0x10bdde87b - trans::expr::trans::h60a968a47a19defeHth
  66:        0x10bddc352 - trans::base::init_local::h08a9fcff749ad013czs
  67:        0x10bdddb02 - trans::controlflow::trans_block::h26a308528bb95051b5d
  68:        0x10bebcda9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  69:        0x10bdc5645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  70:        0x10bdc71c3 - trans::monomorphize::monomorphic_fn::h8c4abbed98c102adusd
  71:        0x10be0f58b - trans::callee::trans_fn_ref_with_substs::heb22ab8904f67bf64kg
  72:        0x10be2889f - trans::meth::trans_method_callee::h92a644d0a2dc2a56N6x
  73:        0x10be23dbb - trans::callee::trans_call_inner::h17268543188350918970
  74:        0x10be2e76a - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  75:        0x10be2d51c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  76:        0x10bdde87b - trans::expr::trans::h60a968a47a19defeHth
  77:        0x10be1d7e1 - trans::callee::trans_args::h43fd254006ea96e2m1g
  78:        0x10be2334e - trans::callee::trans_call_inner::h11221320451677754653
  79:        0x10be2fbec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  80:        0x10bddcf59 - trans::expr::trans_into::h4a29f680db4b3526znh
  81:        0x10bef6d26 - trans::_match::mk_binding_alloca::h3448676421097350985
  82:        0x10bddc280 - trans::base::init_local::h08a9fcff749ad013czs
  83:        0x10bdddb02 - trans::controlflow::trans_block::h26a308528bb95051b5d
  84:        0x10bebcda9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  85:        0x10bdc5645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  86:        0x10bdc6d2a - trans::monomorphize::monomorphic_fn::h8c4abbed98c102adusd
  87:        0x10be0f58b - trans::callee::trans_fn_ref_with_substs::heb22ab8904f67bf64kg
  88:        0x10be0db0e - trans::callee::trans_fn_ref::hb2be9d8aec49d9c4E9f
  89:        0x10be0b1d0 - trans::callee::trans::ha12ccf10898e9b03VYf
  90:        0x10be2224d - trans::callee::trans_call_inner::h11221320451677754653
  91:        0x10be2fbec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  92:        0x10be2d51c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  93:        0x10bdde87b - trans::expr::trans::h60a968a47a19defeHth
  94:        0x10be1d7e1 - trans::callee::trans_args::h43fd254006ea96e2m1g
  95:        0x10be24ee7 - trans::callee::trans_call_inner::h17268543188350918970
  96:        0x10be2e76a - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  97:        0x10bddcf59 - trans::expr::trans_into::h4a29f680db4b3526znh
  98:        0x10bddde29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  99:        0x10be2f050 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  100:        0x10bddcf59 - trans::expr::trans_into::h4a29f680db4b3526znh
 ... <frames omitted>

Could not compile `type-directed-tdd-rust`.

To learn more, run the command again with --verbose.
