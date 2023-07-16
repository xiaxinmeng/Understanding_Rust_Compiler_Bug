
// this is the HIR visibility
2:rustcDEBUG rustdoc::clean Restricted { path: Path { span: src/test/rustdoc/pub-restricted.rs:16:5: 16:9 (#0), res: Def(Mod, DefId(0:0 ~ foo[8787])), segments: [PathSegment { ident: self#0, hir_id: Some(HirId { owner: DefId(0:9 ~ foo[8787]::FooSelf), local_id: 2 }), res: Some(Err), args: None, infer_args: false }] }, hir_id: HirId { owner: DefId(0:9 ~ foo[8787]::FooSelf), local_id: 1 } } FooSelf
// this is tcx.visibility()
2:rustcDEBUG rustdoc::clean::types name=Some("FooSelf"), def_id=DefId(0:9 ~ foo[8787]::FooSelf), visibility=Inherited
