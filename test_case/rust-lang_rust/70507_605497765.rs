
   Compiling playground v0.0.1 (/playground)
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
 --> src/lib.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default

warning: field is never read: `rem`
  --> src/lib.rs:20:5
   |
20 |     rem: &'a [T]
   |     ^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: function is never used: `main`
  --> src/lib.rs:40:4
   |
40 | fn main() {
   |    ^^^^

error: internal compiler error: unexpected const parent in type_of_def_id(): Expr(Expr { hir_id: HirId { owner: DefId(0:27 ~ playground[7ff2]::main[0]), local_id: 30 }, kind: MethodCall(PathSegment { ident: const_chunks_exact#0, hir_id: Some(HirId { owner: DefId(0:27 ~ playground[7ff2]::main[0]), local_id: 27 }), res: Some(Err), args: Some(GenericArgs { args: [Const(ConstArg { value: AnonConst { hir_id: HirId { owner: DefId(0:27 ~ playground[7ff2]::main[0]), local_id: 23 }, body: BodyId { hir_id: HirId { owner: DefId(0:27 ~ playground[7ff2]::main[0]), local_id: 26 } } }, span: src/lib.rs:43:41: 43:49 })], bindings: [], parenthesized: false }), infer_args: false }, src/lib.rs:43:20: 43:38, [Expr { hir_id: HirId { owner: DefId(0:27 ~ playground[7ff2]::main[0]), local_id: 29 }, kind: Path(Resolved(None, Path { span: src/lib.rs:43:14: 43:19, res: Local(HirId { owner: DefId(0:27 ~ playground[7ff2]::main[0]), local_id: 1 }), segments: [PathSegment { ident: slice#0, hir_id: Some(HirId { owner: DefId(0:27 ~ playground[7ff2]::main[0]), local_id: 28 }), res: Some(Local(HirId { owner: DefId(0:27 ~ playground[7ff2]::main[0]), local_id: 1 })), args: None, infer_args: true }] })), attrs: ThinVec(None), span: src/lib.rs:43:14: 43:19 }]), attrs: ThinVec(None), span: src/lib.rs:43:14: 43:52 })

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:43:41
   |
43 |     for a in slice.const_chunks_exact::<{3usize}>() {
   |                                         ^^^^^^^^

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:40:11
   |
40 |   fn main() {
   |  ___________^
41 | |     let slice = &[1i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];
42 | |     
43 | |     for a in slice.const_chunks_exact::<{3usize}>() {
44 | |         dbg!(a);
45 | |     }
46 | | }
   | |_^

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:43:14
   |
43 |     for a in slice.const_chunks_exact::<{3usize}>() {
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:43:9
   |
43 |     for a in slice.const_chunks_exact::<{3usize}>() {
   |         ^

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:44:9
   |
44 |         dbg!(a);
   |         ^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:44:14
   |
44 |         dbg!(a);
   |              ^

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:44:9
   |
44 |         dbg!(a);
   |         ^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:44:9
   |
44 |         dbg!(a);
   |         ^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:44:9
   |
44 |         dbg!(a);
   |         ^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:44:9
   |
44 |         dbg!(a);
   |         ^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:44:9
   |
44 |         dbg!(a);
   |         ^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:44:9
   |
44 |         dbg!(a);
   |         ^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: PromoteTemps: MIR had errors
  --> src/lib.rs:40:1
   |
40 | / fn main() {
41 | |     let slice = &[1i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];
42 | |     
43 | |     for a in slice.const_chunks_exact::<{3usize}>() {
44 | |         dbg!(a);
45 | |     }
46 | | }
   | |_^

error: internal compiler error: broken MIR in DefId(0:27 ~ playground[7ff2]::main[0]) ("return type"): bad type [type error]
  --> src/lib.rs:40:1
   |
40 | / fn main() {
41 | |     let slice = &[1i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];
42 | |     
43 | |     for a in slice.const_chunks_exact::<{3usize}>() {
44 | |         dbg!(a);
45 | |     }
46 | | }
   | |_^

error: internal compiler error: broken MIR in DefId(0:27 ~ playground[7ff2]::main[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: src/lib.rs:40:1: 46:2, scope: scope[0] } }): bad type [type error]
  --> src/lib.rs:40:1
   |
40 | / fn main() {
41 | |     let slice = &[1i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];
42 | |     
43 | |     for a in slice.const_chunks_exact::<{3usize}>() {
44 | |         dbg!(a);
45 | |     }
46 | | }
   | |_^

error: internal compiler error: mir_const_qualif: MIR had errors
  --> src/lib.rs:43:41
   |
43 |     for a in slice.const_chunks_exact::<{3usize}>() {
   |                                         ^^^^^^^^

error: internal compiler error: PromoteTemps: MIR had errors
  --> src/lib.rs:43:41
   |
43 |     for a in slice.const_chunks_exact::<{3usize}>() {
   |                                         ^^^^^^^^

error: internal compiler error: broken MIR in DefId(0:28 ~ playground[7ff2]::main[0]::{{constant}}[0]) ("return type"): bad type [type error]
  --> src/lib.rs:43:41
   |
43 |     for a in slice.const_chunks_exact::<{3usize}>() {
   |                                         ^^^^^^^^

error: internal compiler error: broken MIR in DefId(0:28 ~ playground[7ff2]::main[0]::{{constant}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: src/lib.rs:43:41: 43:49, scope: scope[0] } }): bad type [type error]
  --> src/lib.rs:43:41
   |
43 |     for a in slice.const_chunks_exact::<{3usize}>() {
   |                                         ^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (75208942f 2020-03-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: could not compile `playground`.

To learn more, run the command again with --verbose.
