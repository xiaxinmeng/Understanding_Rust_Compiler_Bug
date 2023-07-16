
error: internal compiler error: unexpected const parent path Expr(Expr { hir_id: HirId { owner: DefId(0:8 ~ ice[227a]::foo[0]), local_id: 6 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: DefId(0:8 ~ ice[227a]::foo[0]), local_id: 2 }, kind: Path(Resolved(None, Path { span: src/lib.rs:11:5: 11:39 (#0), res: Def(Struct, DefId(0:3 ~ ice[227a]::ChannelUsage[0])), segments: [PathSegment { ident: ChannelUsage#0, hir_id: Some(HirId { owner: DefId(0:8 ~ ice[227a]::foo[0]), local_id: 1 }), res: Some(Err), args: None, infer_args: true }] })), span: src/lib.rs:11:5: 11:39 (#0) }, PathSegment { ident: collect_arr#0, hir_id: Some(HirId { owner: DefId(0:8 ~ ice[227a]::foo[0]), local_id: 5 }), res: Some(Err), args: Some(GenericArgs { args: [Const(ConstArg { value: AnonConst { hir_id: HirId { owner: DefId(0:8 ~ ice[227a]::foo[0]), local_id: 3 }, body: BodyId { hir_id: HirId { owner: DefId(0:8 ~ ice[227a]::foo[0]), local_id: 4 } } }, span: src/lib.rs:11:33: 11:38 (#0) })], bindings: [], parenthesized: false }), infer_args: false })), attrs: ThinVec(None), span: src/lib.rs:11:5: 11:39 (#0) })

error: internal compiler error: Const::from_anon_const: couldn't lit_to_const
  --> src/lib.rs:11:33
   |
11 |     ChannelUsage::collect_arr::<16i32>();
   |                                 ^^^^^

error: internal compiler error: `ErrorReported` without an error
  --> src/lib.rs:11:33
   |
11 |     ChannelUsage::collect_arr::<16i32>();
   |                                 ^^^^^

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:10:10
   |
10 |   fn foo() {
   |  __________^
11 | |     ChannelUsage::collect_arr::<16i32>();
12 | | }
   | |_^

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:11:5
   |
11 |     ChannelUsage::collect_arr::<16i32>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: PromoteTemps: MIR had errors
  --> src/lib.rs:10:1
   |
10 | / fn foo() {
11 | |     ChannelUsage::collect_arr::<16i32>();
12 | | }
   | |_^

error: internal compiler error: broken MIR in DefId(0:8 ~ ice[227a]::foo[0]) ("return type"): bad type [type error]
  --> src/lib.rs:10:1
   |
10 | / fn foo() {
11 | |     ChannelUsage::collect_arr::<16i32>();
12 | | }
   | |_^

error: internal compiler error: broken MIR in DefId(0:8 ~ ice[227a]::foo[0]) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: src/lib.rs:10:1: 12:2 (#0), scope: scope[0] } }): bad type [type error]
  --> src/lib.rs:10:1
   |
10 | / fn foo() {
11 | |     ChannelUsage::collect_arr::<16i32>();
12 | | }
   | |_^

error: internal compiler error: mir_const_qualif: MIR had errors
  --> src/lib.rs:11:33
   |
11 |     ChannelUsage::collect_arr::<16i32>();
   |                                 ^^^^^

error: internal compiler error: PromoteTemps: MIR had errors
  --> src/lib.rs:11:33
   |
11 |     ChannelUsage::collect_arr::<16i32>();
   |                                 ^^^^^

error: internal compiler error: broken MIR in DefId(0:9 ~ ice[227a]::foo[0]::{{constant}}[0]) ("return type"): bad type [type error]
  --> src/lib.rs:11:33
   |
11 |     ChannelUsage::collect_arr::<16i32>();
   |                                 ^^^^^

error: internal compiler error: broken MIR in DefId(0:9 ~ ice[227a]::foo[0]::{{constant}}[0]) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: src/lib.rs:11:33: 11:38 (#0), scope: scope[0] } }): bad type [type error]
  --> src/lib.rs:11:33
   |
11 |     ChannelUsage::collect_arr::<16i32>();
   |                                 ^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-nightly (a647c0cd6 2020-06-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden
