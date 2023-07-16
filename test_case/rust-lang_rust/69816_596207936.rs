rust

error: internal compiler error: unexpected const parent in type_of_def_id(): Expr(expr(HirId { owner: DefIndex(10), local_id: 20 }: ::std::ops::Range{start: 0, end: 10,}.default_for_size::<>()))

error: internal compiler error: mir_const_qualif: MIR had errors
  --> src/main.rs:16:49
   |
16 |     let arr:[u32;10]=(0..10).default_for_size::<10usize>();
   |                                                 ^^^^^^^

error: internal compiler error: PromoteTemps: MIR had errors
  --> src/main.rs:16:49
   |
16 |     let arr:[u32;10]=(0..10).default_for_size::<10usize>();
   |                                                 ^^^^^^^

error: internal compiler error: broken MIR in DefId(0:13 ~ playground[a375]::main[0]::{{constant}}[1]) ("return type"): bad type [type error]
  --> src/main.rs:16:49
   |
16 |     let arr:[u32;10]=(0..10).default_for_size::<10usize>();
   |                                                 ^^^^^^^

error: internal compiler error: broken MIR in DefId(0:13 ~ playground[a375]::main[0]::{{constant}}[1]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: src/main.rs:16:49: 16:56, scope: scope[0] } }): bad type [type error]
  --> src/main.rs:16:49
   |
16 |     let arr:[u32;10]=(0..10).default_for_size::<10usize>();
   |                                                 ^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic
