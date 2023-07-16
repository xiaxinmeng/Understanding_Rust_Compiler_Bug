
$ RUST_BACKTRACE=1 cargo test
   Compiling deque v0.1.8 (file:///home/samuel/deque)
/home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sync/mutex.rs:177:37: 180:2 error: internal compiler error: debuginfo: Could not find scope info for node NodeExpr(Expr { id: 3082, node: ExprStruct(Path { span: Span { lo: BytePos(4557025), hi: BytePos(4557036), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: StaticMutex#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }, [Field { ident: Spanned { node: lock#0, span: Span { lo: BytePos(1770102), hi: BytePos(1770106), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 3083, node: ExprPath(None, Path { span: Span { lo: BytePos(4557049), hi: BytePos(4557064), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: sys#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: MUTEX_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(4557049), hi: BytePos(4557064), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(4557043), hi: BytePos(4557064), expn_id: ExpnId(4294967295) } }, Field { ident: Spanned { node: poison#0, span: Span { lo: BytePos(1770129), hi: BytePos(1770135), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 3084, node: ExprPath(None, Path { span: Span { lo: BytePos(4557078), hi: BytePos(4557095), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: poison#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: FLAG_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(4557078), hi: BytePos(4557095), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(4557070), hi: BytePos(4557095), expn_id: ExpnId(4294967295) } }], None), span: Span { lo: BytePos(4557025), hi: BytePos(4557098), expn_id: ExpnId(4294967295) } })
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:     0x7f10c977cf1f - sys::backtrace::write::hf79a3da4fdecb8a0OBA
   2:     0x7f10c97a7c32 - panicking::on_panic::h9f64f4c69e19f194hHJ
   3:     0x7f10c96dceda - rt::unwind::begin_unwind_inner::h37f4496c980fe936knJ
   4:     0x7f10c6b10abd - rt::unwind::begin_unwind::h8320268356453106285
   5:     0x7f10c6b10a63 - diagnostic::SpanHandler::span_bug::h83c8af232eaba6a9h0D
   6:     0x7f10c73eeca3 - session::Session::span_bug::h857b2c7ae23c9286ISp
   7:     0x7f10c8f2180c - trans::debuginfo::scope_metadata::hac54dfdbdcd04cd9SjE
   8:     0x7f10c8e34408 - trans::debuginfo::set_source_location::h1067a74086ed9dd48MD
   9:     0x7f10c8de8e42 - trans::expr::trans_into::h95c6d2681fdd2548znh
  10:     0x7f10c8de9109 - trans::expr::trans_into::h95c6d2681fdd2548znh
  11:     0x7f10c8e4b67f - trans::expr::trans_uniq_expr::h5f082eea62818f84ukj
  12:     0x7f10c8e4c2ef - trans::expr::trans_unary::h4412379888608420Jgj
  13:     0x7f10c8e3680e - trans::expr::trans_unadjusted::hfd3a5e1b5cbe37d5z4h
  14:     0x7f10c8de9417 - trans::expr::trans_into::h95c6d2681fdd2548znh
  15:     0x7f10c8e696f9 - trans::expr::trans_adt::h1af69b9b4e52152aO6i
  16:     0x7f10c8e6c1af - trans::expr::trans_struct::closure.42069
  17:     0x7f10c8e56016 - trans::expr::trans_struct::hcae8f9103f3460d5K2i
  18:     0x7f10c8e3853d - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  19:     0x7f10c8de93f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  20:     0x7f10c8dea227 - trans::controlflow::trans_block::h3e86dfa8c58560e6b5d
  21:     0x7f10c8ec1821 - trans::base::trans_closure::hab3cc3c679d5ff23Kkt
  22:     0x7f10c8dd2b08 - trans::base::trans_fn::he0569b8eb832adf9Dvt
  23:     0x7f10c8dd45a0 - trans::monomorphize::monomorphic_fn::hc1b7393dd1dc77f1usd
  24:     0x7f10c8e19e4e - trans::callee::trans_fn_ref_with_substs::hd01acb4398310d154kg
  25:     0x7f10c8e1843e - trans::callee::trans_fn_ref::hb48e614c9b6dd9bcE9f
  26:     0x7f10c8e1588d - trans::callee::trans::ha56f4fe94448e6baVYf
  27:     0x7f10c8e2bfbb - trans::callee::trans_call_inner::h9722042290657949952
  28:     0x7f10c8e389e0 - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  29:     0x7f10c8e36668 - trans::expr::trans_unadjusted::hfd3a5e1b5cbe37d5z4h
  30:     0x7f10c8deab58 - trans::expr::trans::h23d7d0dd91a5190fHth
  31:     0x7f10c8e279f3 - trans::callee::trans_args::h29a92a6ed71c85ebm1g
  32:     0x7f10c8e2cea0 - trans::callee::trans_call_inner::h9722042290657949952
  33:     0x7f10c8e389e0 - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  34:     0x7f10c8de93f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  35:     0x7f10c8e696f9 - trans::expr::trans_adt::h1af69b9b4e52152aO6i
  36:     0x7f10c8e6c1af - trans::expr::trans_struct::closure.42069
  37:     0x7f10c8e56016 - trans::expr::trans_struct::hcae8f9103f3460d5K2i
  38:     0x7f10c8e3853d - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  39:     0x7f10c8de93f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  40:     0x7f10c8dea227 - trans::controlflow::trans_block::h3e86dfa8c58560e6b5d
  41:     0x7f10c8ec1821 - trans::base::trans_closure::hab3cc3c679d5ff23Kkt
  42:     0x7f10c8dd2b08 - trans::base::trans_fn::he0569b8eb832adf9Dvt
  43:     0x7f10c8dd45a0 - trans::monomorphize::monomorphic_fn::hc1b7393dd1dc77f1usd
  44:     0x7f10c8e19e4e - trans::callee::trans_fn_ref_with_substs::hd01acb4398310d154kg
  45:     0x7f10c8e1843e - trans::callee::trans_fn_ref::hb48e614c9b6dd9bcE9f
  46:     0x7f10c8e1588d - trans::callee::trans::ha56f4fe94448e6baVYf
  47:     0x7f10c8e2bfbb - trans::callee::trans_call_inner::h9722042290657949952
  48:     0x7f10c8e389e0 - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  49:     0x7f10c8de93f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  50:     0x7f10c8ef9967 - trans::_match::mk_binding_alloca::h13035368140960659810
  51:     0x7f10c8de87fd - trans::base::init_local::h1e7c96bb7077440dczs
  52:     0x7f10c8de9f02 - trans::controlflow::trans_block::h3e86dfa8c58560e6b5d
  53:     0x7f10c8ec1821 - trans::base::trans_closure::hab3cc3c679d5ff23Kkt
  54:     0x7f10c8dd2b08 - trans::base::trans_fn::he0569b8eb832adf9Dvt
  55:     0x7f10c8dce8a1 - trans::base::trans_item::h48fc370b7d259ac7vTt
  56:     0x7f10c8eca7ec - trans::base::trans_crate::hc92be67ede893c70GPu
  57:     0x7f10c9db7e83 - driver::phase_4_translate_to_llvm::h9904f5d5fc3fb761rNa
  58:     0x7f10c9d9383f - driver::compile_input::h3913ff7013f0c056Iba
  59:     0x7f10c9e5bcb7 - run_compiler::h28a4446bae1034e7H5b
  60:     0x7f10c9e59829 - thunk::F.Invoke<A, R>::invoke::h6503055919709693733
  61:     0x7f10c9e584a0 - rt::unwind::try::try_fn::h1384674024000742916
  62:     0x7f10c9817de8 - rust_try_inner
  63:     0x7f10c9817dd5 - rust_try
  64:     0x7f10c9e58c3f - thunk::F.Invoke<A, R>::invoke::h5780663349966142752
  65:     0x7f10c9792965 - sys::thread::thread_start::h4ab695857833a5dar8E
  66:     0x7f10c35d7373 - start_thread
  67:     0x7f10c935827c - __clone
  68:                0x0 - <unknown>
