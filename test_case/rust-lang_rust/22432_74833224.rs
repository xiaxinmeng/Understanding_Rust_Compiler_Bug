
$ RUST_BACKTRACE=1 cargo build --verbose
       Fresh gcc v0.2.1
       Fresh pkg-config v0.2.1
   Compiling openssl-sys v0.4.1 (file:///home/munksgaard/tmp/rust-openssl/openssl-sys)
     Running `rustc src/lib.rs --crate-name openssl-sys --crate-type lib -g -C metadata=0a516417e4a8262c -C extra-filename=-0a516417e4a8262c --out-dir /home/munksgaard/tmp/rust-openssl/openssl-sys/target --emit=dep-info,link -L dependency=/home/munksgaard/tmp/rust-openssl/openssl-sys/target -L dependency=/home/munksgaard/tmp/rust-openssl/openssl-sys/target/deps -L native=/usr/lib64 -L native=/home/munksgaard/tmp/rust-openssl/openssl-sys/target/build/openssl-sys-0a516417e4a8262c/out -l ssl -l crypto -l static=old_openssl_shim`
src/lib.rs:1:1: 1:1 error: internal compiler error: debuginfo: Could not find scope info for node NodeExpr(Expr { id: 5326, node: ExprStruct(Path { span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: StaticMutex#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }, [Field { ident: Spanned { node: lock#0, span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 5327, node: ExprPath(Path { span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: sys#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: MUTEX_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }, Field { ident: Spanned { node: poison#0, span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 5328, node: ExprPath(Path { span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: poison#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: FLAG_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }], None), span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } })
src/lib.rs:1 #![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
             ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/munksgaard/src/rust/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:     0x7fbcb65cf9f0 - sys::backtrace::write::hc2151c58018175e4EmB
   2:     0x7fbcb65f8360 - panicking::on_panic::h072dffafa85b39e5iPK
   3:     0x7fbcb653f7e0 - rt::unwind::begin_unwind_inner::h6f07616a934ac46bDtK
   4:     0x7fbcb3856cb0 - rt::unwind::begin_unwind::h5219167339450884215
   5:     0x7fbcb3856c40 - diagnostic::SpanHandler::span_bug::hda6c189b692dffbf74E
   6:     0x7fbcb43e14e0 - session::Session::span_bug::he9ae6b6969340694f3s
   7:     0x7fbcb536a300 - trans::debuginfo::scope_metadata::h6731a042098b67f4UFF
   8:     0x7fbcb528e920 - trans::debuginfo::set_source_location::ha2990c29a8fe020987E
   9:     0x7fbcb5245500 - trans::expr::trans_into::h7870cc45782e63a6jGh
  10:     0x7fbcb5245500 - trans::expr::trans_into::h7870cc45782e63a6jGh
  11:     0x7fbcb5246070 - trans::controlflow::trans_block::hf9c60b32ec83f851Xee
  12:     0x7fbcb53123b0 - trans::base::trans_closure::h229f078514fa9f31Fiu
  13:     0x7fbcb52c5150 - trans::closure::trans_closure_expr::h03bc7d9fb9c19401aqy
  14:     0x7fbcb52f4a10 - trans::consts::const_expr_unadjusted::h4d61697462349d4bamo
  15:     0x7fbcb528f060 - trans::consts::get_const_expr_as_global::h9e0aec0cc1dcbe1bd6n
  16:     0x7fbcb5246bb0 - trans::expr::trans::h5ec539888faaf3c5BMh
  17:     0x7fbcb5281400 - trans::callee::trans_args::h6cd23ea01b124b4brjh
  18:     0x7fbcb5288b20 - trans::callee::trans_call_inner::h17700662577811064847
  19:     0x7fbcb5290fb0 - trans::expr::trans_rvalue_dps_unadjusted::h5c8037cc245c29152Wi
  20:     0x7fbcb52903f0 - trans::expr::trans_unadjusted::hf6c8f8f87021a16bmqi
  21:     0x7fbcb5246bb0 - trans::expr::trans::h5ec539888faaf3c5BMh
  22:     0x7fbcb5281400 - trans::callee::trans_args::h6cd23ea01b124b4brjh
  23:     0x7fbcb5288b20 - trans::callee::trans_call_inner::h17700662577811064847
  24:     0x7fbcb5290fb0 - trans::expr::trans_rvalue_dps_unadjusted::h5c8037cc245c29152Wi
  25:     0x7fbcb52903f0 - trans::expr::trans_unadjusted::hf6c8f8f87021a16bmqi
  26:     0x7fbcb5246bb0 - trans::expr::trans::h5ec539888faaf3c5BMh
  27:     0x7fbcb5281400 - trans::callee::trans_args::h6cd23ea01b124b4brjh
  28:     0x7fbcb52874c0 - trans::callee::trans_call_inner::h1829336221582505235
  29:     0x7fbcb5290fb0 - trans::expr::trans_rvalue_dps_unadjusted::h5c8037cc245c29152Wi
  30:     0x7fbcb5245500 - trans::expr::trans_into::h7870cc45782e63a6jGh
  31:     0x7fbcb53480d0 - trans::_match::mk_binding_alloca::h17654241473117154624
  32:     0x7fbcb5244bd0 - trans::base::init_local::h38abc2e7211753e3Sst
  33:     0x7fbcb5246070 - trans::controlflow::trans_block::hf9c60b32ec83f851Xee
  34:     0x7fbcb5290fb0 - trans::expr::trans_rvalue_dps_unadjusted::h5c8037cc245c29152Wi
  35:     0x7fbcb5245500 - trans::expr::trans_into::h7870cc45782e63a6jGh
  36:     0x7fbcb5246070 - trans::controlflow::trans_block::hf9c60b32ec83f851Xee
  37:     0x7fbcb53123b0 - trans::base::trans_closure::h229f078514fa9f31Fiu
  38:     0x7fbcb52c5150 - trans::closure::trans_closure_expr::h03bc7d9fb9c19401aqy
  39:     0x7fbcb5290fb0 - trans::expr::trans_rvalue_dps_unadjusted::h5c8037cc245c29152Wi
  40:     0x7fbcb52903f0 - trans::expr::trans_unadjusted::hf6c8f8f87021a16bmqi
  41:     0x7fbcb5246bb0 - trans::expr::trans::h5ec539888faaf3c5BMh
  42:     0x7fbcb5281400 - trans::callee::trans_args::h6cd23ea01b124b4brjh
  43:     0x7fbcb5288b20 - trans::callee::trans_call_inner::h17700662577811064847
  44:     0x7fbcb5290fb0 - trans::expr::trans_rvalue_dps_unadjusted::h5c8037cc245c29152Wi
  45:     0x7fbcb5245500 - trans::expr::trans_into::h7870cc45782e63a6jGh
  46:     0x7fbcb5246070 - trans::controlflow::trans_block::hf9c60b32ec83f851Xee
  47:     0x7fbcb5290fb0 - trans::expr::trans_rvalue_dps_unadjusted::h5c8037cc245c29152Wi
  48:     0x7fbcb5245500 - trans::expr::trans_into::h7870cc45782e63a6jGh
  49:     0x7fbcb5246070 - trans::controlflow::trans_block::hf9c60b32ec83f851Xee
  50:     0x7fbcb53123b0 - trans::base::trans_closure::h229f078514fa9f31Fiu
  51:     0x7fbcb52319d0 - trans::base::trans_fn::h15a0839090e57fdfWtu
  52:     0x7fbcb522d040 - trans::base::trans_item::hcb68e8106b5a736aPSu
  53:     0x7fbcb5319910 - trans::base::trans_crate::hea1dd27f09b3034efQv
  54:     0x7fbcb6c44de0 - driver::phase_4_translate_to_llvm::hb9632b5b608dc69diPa
  55:     0x7fbcb6c1de10 - driver::compile_input::hb1442f9e5047d907Eba
  56:     0x7fbcb6cee150 - run_compiler::he44f6d934e02b5b6Rbc
  57:     0x7fbcb6cec6b0 - thunk::F.Invoke<A, R>::invoke::h2990667340858899670
  58:     0x7fbcb6ceb590 - rt::unwind::try::try_fn::h11448077034486860112
  59:     0x7fbcb666f640 - rust_try_inner
  60:     0x7fbcb666f630 - rust_try
  61:     0x7fbcb6ceb840 - thunk::F.Invoke<A, R>::invoke::h10996923374445134736
  62:     0x7fbcb65e3cf0 - sys::thread::thread_start::h2ad0b44c98dca42cnWF
  63:     0x7fbcb0639200 - <unknown>
  64:     0x7fbcb61d02a9 - __clone
  65:                0x0 - <unknown>

Could not compile `openssl-sys`.

Caused by:
  Process didn't exit successfully: `rustc src/lib.rs --crate-name openssl-sys --crate-type lib -g -C metadata=0a516417e4a8262c -C extra-filename=-0a516417e4a8262c --out-dir /home/munksgaard/tmp/rust-openssl/openssl-sys/target --emit=dep-info,link -L dependency=/home/munksgaard/tmp/rust-openssl/openssl-sys/target -L dependency=/home/munksgaard/tmp/rust-openssl/openssl-sys/target/deps -L native=/usr/lib64 -L native=/home/munksgaard/tmp/rust-openssl/openssl-sys/target/build/openssl-sys-0a516417e4a8262c/out -l ssl -l crypto -l static=old_openssl_shim` (status=101)
