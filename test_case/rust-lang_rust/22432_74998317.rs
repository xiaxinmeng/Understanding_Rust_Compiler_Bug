
RUST_BACKTRACE=1 cargo build --verbose
   Compiling pkg-config v0.2.1
     Running `rustc /Users/scott/.cargo/registry/src/github.com-1ecc6299db9ec823/pkg-config-0.2.1/src/lib.rs --crate-name pkg-config --crate-type lib -g -C metadata=fa0aaf6e1b26fdfc -C extra-filename=-fa0aaf6e1b26fdfc --out-dir /Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps --emit=dep-info,link -L dependency=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps -L dependency=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps -Awarnings`
   Compiling gcc v0.2.1
     Running `rustc /Users/scott/.cargo/registry/src/github.com-1ecc6299db9ec823/gcc-0.2.1/src/lib.rs --crate-name gcc --crate-type lib -g -C metadata=33c984517cbb3cc2 -C extra-filename=-33c984517cbb3cc2 --out-dir /Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps --emit=dep-info,link -L dependency=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps -L dependency=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps -Awarnings`
   Compiling openssl-sys v0.4.1 (file:///Users/scott/Devel/Rust/fix-log/rust-openssl/openssl)
     Running `rustc /Users/scott/Devel/Rust/fix-log/rust-openssl/openssl-sys/build.rs --crate-name build-script-build --crate-type bin -C prefer-dynamic -g --out-dir /Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/build/openssl-sys-8d0da4c531140a11 --emit=dep-info,link -L dependency=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps -L dependency=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps --extern pkg-config=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps/libpkg-config-fa0aaf6e1b26fdfc.rlib --extern gcc=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps/libgcc-33c984517cbb3cc2.rlib`
     Running `/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/build/openssl-sys-8d0da4c531140a11/build-script-build`
     Running `rustc /Users/scott/Devel/Rust/fix-log/rust-openssl/openssl-sys/src/lib.rs --crate-name openssl-sys --crate-type lib -g -C metadata=8d0da4c531140a11 -C extra-filename=-8d0da4c531140a11 --out-dir /Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps --emit=dep-info,link -L dependency=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps -L dependency=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps -L native=/usr/lib -L native=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/build/openssl-sys-8d0da4c531140a11/out -l ssl -l crypto -l z -l static=old_openssl_shim`
/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl-sys/src/lib.rs:1:1: 1:1 error: internal compiler error: debuginfo: Could not find scope info for node NodeExpr(Expr { id: 5326, node: ExprStruct(Path { span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: StaticMutex#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }, [Field { ident: Spanned { node: lock#0, span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 5327, node: ExprPath(Path { span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: sys#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: MUTEX_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }, Field { ident: Spanned { node: poison#0, span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }, expr: Expr { id: 5328, node: ExprPath(Path { span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }, global: false, segments: [PathSegment { identifier: poison#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }, PathSegment { identifier: FLAG_INIT#0, parameters: AngleBracketedParameters(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] }), span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }, span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } }], None), span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) } })
/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl-sys/src/lib.rs:1 #![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
                                                                      ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:        0x10b20c7c3 - sys::backtrace::write::h758e31f43edeba51YoB
   2:        0x10b23a525 - panicking::on_panic::h64e08c3ac337d99aAGK
   3:        0x10b16bb28 - rt::unwind::begin_unwind_inner::hbe305651b0fb4548QnK
   4:        0x10a84517f - rt::unwind::begin_unwind::h2254625848453715247
   5:        0x10a84512c - diagnostic::SpanHandler::span_bug::h37c90c46b671876c74E
   6:        0x107efae3d - session::Session::span_bug::h62bb1b8f6c18bf99f3s
   7:        0x10782aa25 - trans::debuginfo::scope_metadata::h1fbe6077a5482c041FF
   8:        0x107746898 - trans::debuginfo::set_source_location::he816012c69125c66f8E
   9:        0x1076fa4a6 - trans::expr::trans_into::h985f6b8ade1f0862jGh
  10:        0x1076fa8b3 - trans::expr::trans_into::h985f6b8ade1f0862jGh
  11:        0x1076fb9d6 - trans::controlflow::trans_block::h24750b7c4a4226dcXee
  12:        0x1077d1339 - trans::base::trans_closure::hc0a2577d0b8ff90aFiu
  13:        0x10777e596 - trans::closure::trans_closure_expr::h25418713c3e0fb75aqy
  14:        0x1077b0d95 - trans::consts::const_expr_unadjusted::h6449b3b4f5651270amo
  15:        0x107747249 - trans::consts::get_const_expr_as_global::hcb75b2273a1f8b31d6n
  16:        0x1076fbef7 - trans::expr::trans::h6ed65d3057a56e00BMh
  17:        0x107738f85 - trans::callee::trans_args::hfd5a9e8b98fe7124rjh
  18:        0x10774111f - trans::callee::trans_call_inner::h6331790232619854971
  19:        0x107749b74 - trans::expr::trans_rvalue_dps_unadjusted::h1c13f9f319ca8f992Wi
  20:        0x107748842 - trans::expr::trans_unadjusted::h60994bf53fc1f141mqi
  21:        0x1076fc3b0 - trans::expr::trans::h6ed65d3057a56e00BMh
  22:        0x107738f85 - trans::callee::trans_args::hfd5a9e8b98fe7124rjh
  23:        0x10774111f - trans::callee::trans_call_inner::h6331790232619854971
  24:        0x107749b74 - trans::expr::trans_rvalue_dps_unadjusted::h1c13f9f319ca8f992Wi
  25:        0x107748842 - trans::expr::trans_unadjusted::h60994bf53fc1f141mqi
  26:        0x1076fc3b0 - trans::expr::trans::h6ed65d3057a56e00BMh
  27:        0x107738f85 - trans::callee::trans_args::hfd5a9e8b98fe7124rjh
  28:        0x10773f874 - trans::callee::trans_call_inner::h11469899704773280112
  29:        0x10774afcd - trans::expr::trans_rvalue_dps_unadjusted::h1c13f9f319ca8f992Wi
  30:        0x1076fabcb - trans::expr::trans_into::h985f6b8ade1f0862jGh
  31:        0x1078067af - trans::_match::mk_binding_alloca::h16447597904001643702
  32:        0x1076f9d01 - trans::base::init_local::h3a7fbf969f4e1b35Sst
  33:        0x1076fb6b4 - trans::controlflow::trans_block::h24750b7c4a4226dcXee
  34:        0x10774a421 - trans::expr::trans_rvalue_dps_unadjusted::h1c13f9f319ca8f992Wi
  35:        0x1076fabcb - trans::expr::trans_into::h985f6b8ade1f0862jGh
  36:        0x1076fb9d6 - trans::controlflow::trans_block::h24750b7c4a4226dcXee
  37:        0x1077d1339 - trans::base::trans_closure::hc0a2577d0b8ff90aFiu
  38:        0x10777e596 - trans::closure::trans_closure_expr::h25418713c3e0fb75aqy
  39:        0x10774b18e - trans::expr::trans_rvalue_dps_unadjusted::h1c13f9f319ca8f992Wi
  40:        0x107748930 - trans::expr::trans_unadjusted::h60994bf53fc1f141mqi
  41:        0x1076fc3b0 - trans::expr::trans::h6ed65d3057a56e00BMh
  42:        0x107738f85 - trans::callee::trans_args::hfd5a9e8b98fe7124rjh
  43:        0x10774111f - trans::callee::trans_call_inner::h6331790232619854971
  44:        0x107749b74 - trans::expr::trans_rvalue_dps_unadjusted::h1c13f9f319ca8f992Wi
  45:        0x1076fabcb - trans::expr::trans_into::h985f6b8ade1f0862jGh
  46:        0x1076fb9d6 - trans::controlflow::trans_block::h24750b7c4a4226dcXee
  47:        0x10774a421 - trans::expr::trans_rvalue_dps_unadjusted::h1c13f9f319ca8f992Wi
  48:        0x1076fabcb - trans::expr::trans_into::h985f6b8ade1f0862jGh
  49:        0x1076fb9d6 - trans::controlflow::trans_block::h24750b7c4a4226dcXee
  50:        0x1077d1339 - trans::base::trans_closure::hc0a2577d0b8ff90aFiu
  51:        0x1076e59b6 - trans::base::trans_fn::hdc6e33d20e22c94bWtu
  52:        0x1076e0d1d - trans::base::trans_item::h64db2b88677453d4PSu
  53:        0x1077d7d1c - trans::base::trans_crate::h414eeb210dd461c6fQv
  54:        0x1075669c8 - driver::phase_4_translate_to_llvm::h83cf15bb7798b7baiPa
  55:        0x1075408a6 - driver::compile_input::h5f42c7591ab1e8afEba
  56:        0x10761a284 - run_compiler::h569cf29d11ba11b9Rbc
  57:        0x107617305 - thunk::F.Invoke<A, R>::invoke::h584465160991669811
  58:        0x107615fa0 - rt::unwind::try::try_fn::h36519155281163913
  59:        0x10b2b4329 - rust_try_inner
  60:        0x10b2b4316 - rust_try
  61:        0x107616699 - thunk::F.Invoke<A, R>::invoke::h10026663398983596708
  62:        0x10b222d13 - sys::thread::thread_start::h0953ffc55eda2123CWF
  63:     0x7fff874fb268 - _pthread_body
  64:     0x7fff874fb1e5 - _pthread_body

Could not compile `openssl-sys`.

Caused by:
  Process didn't exit successfully: `rustc /Users/scott/Devel/Rust/fix-log/rust-openssl/openssl-sys/src/lib.rs --crate-name openssl-sys --crate-type lib -g -C metadata=8d0da4c531140a11 -C extra-filename=-8d0da4c531140a11 --out-dir /Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps --emit=dep-info,link -L dependency=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps -L dependency=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/deps -L native=/usr/lib -L native=/Users/scott/Devel/Rust/fix-log/rust-openssl/openssl/target/build/openssl-sys-8d0da4c531140a11/out -l ssl -l crypto -l z -l static=old_openssl_shim` (status=101)
