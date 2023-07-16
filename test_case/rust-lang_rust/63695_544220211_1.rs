
error: internal compiler error: unexpected const parent path Expr(expr(HirId { owner: DefIndex(17), local_id: 8 }: <S>::test::<>))

error: internal compiler error: cat_expr Errd
  --> src/main.rs:78:15
   |
78 |     S::test::<{ 16i32 }>();
   |               ^^^^^^^^^

error: internal compiler error: mir_const_qualif: MIR had errors
  --> src/main.rs:78:15
   |
78 |     S::test::<{ 16i32 }>();
   |               ^^^^^^^^^

error: internal compiler error: QualifyAndPromoteConstants: MIR had errors
  --> src/main.rs:78:15
   |
78 |     S::test::<{ 16i32 }>();
   |               ^^^^^^^^^

error: internal compiler error: broken MIR in DefId(0:18 ~ rust_test[143a]::main[0]::{{constant}}[0]) ("return type"): bad type [type error]
  --> src/main.rs:78:15
   |
78 |     S::test::<{ 16i32 }>();
   |               ^^^^^^^^^

error: internal compiler error: broken MIR in DefId(0:18 ~ rust_test[143a]::main[0]::{{constant}}[0]) (LocalDecl { mutability: Mut, is_user_variable: None, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, name: None, source_info: SourceInfo { span: src/main.rs:78:15: 78:24, scope: scope[0] }, visibility_scope: scope[0] }): bad type [type error]
  --> src/main.rs:78:15
   |
78 |     S::test::<{ 16i32 }>();
   |               ^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:391:17
