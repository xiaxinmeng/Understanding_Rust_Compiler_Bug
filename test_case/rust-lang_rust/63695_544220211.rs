
error: internal compiler error: unexpected const parent in type_of_def_id(): Expr(expr(HirId { owner: DefIndex(20), local_id: 82 }: foo.test::<>()))

error: internal compiler error: cat_expr Errd
  --> src/main.rs:83:31
   |
83 |     println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |                               ^^^^^^^

error: internal compiler error: mir_const_qualif: MIR had errors
  --> src/main.rs:83:31
   |
83 |     println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |                               ^^^^^^^

error: internal compiler error: QualifyAndPromoteConstants: MIR had errors
  --> src/main.rs:83:31
   |
83 |     println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |                               ^^^^^^^

error: internal compiler error: broken MIR in DefId(0:22 ~ rust_test[143a]::main[0]::{{constant}}[1]) ("return type"): bad type [type error]
  --> src/main.rs:83:31
   |
83 |     println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |                               ^^^^^^^

error: internal compiler error: broken MIR in DefId(0:22 ~ rust_test[143a]::main[0]::{{constant}}[1]) (LocalDecl { mutability: Mut, is_user_variable: None, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, name: None, source_info: SourceInfo { span: src/main.rs:83:31: 83:38, scope: scope[0] }, visibility_scope: scope[0] }): bad type [type error]
  --> src/main.rs:83:31
   |
83 |     println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
   |                               ^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:391:17
