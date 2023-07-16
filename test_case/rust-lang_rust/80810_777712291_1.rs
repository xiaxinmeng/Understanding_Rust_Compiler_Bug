text
 Documenting core v0.0.0 (/builddir/build/BUILD/rustc-1.50.0-src/library/core)
error: internal compiler error: compiler/rustc_privacy/src/lib.rs:500:25: item Item { ident: #0, hir_id: HirId { owner: DefId(0:474 ~ core[8787]::num::flt2dec::{misc#0}), local_id: 0 }, attrs: [], kind: Use(Path { span: library/core/src/num/flt2dec/mod.rs:125:9: 125:70 (#0), res: Err, segments: [PathSegment { ident: self#0, hir_id: Some(HirId { owner: DefId(0:474 ~ core[8787]::num::flt2dec::{misc#0}), local_id: 1 }), res: Some(Err), args: None, infer_args: false }, PathSegment { ident: decoder#0, hir_id: Some(HirId { owner: DefId(0:474 ~ core[8787]::num::flt2dec::{misc#0}), local_id: 2 }), res: Some(Def(Mod, DefId(0:480 ~ core[8787]::num::flt2dec::decoder))), args: None, infer_args: false }] }, ListStem), vis: Spanned { node: Inherited, span: library/core/src/num/flt2dec/mod.rs:125:9: 125:9 (#0) }, span: library/core/src/num/flt2dec/mod.rs:125:1: 125:71 (#0) } with DefKind Struct
thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: Unrecognized option: 'markdown-css'
error: aborting due to previous error
error: could not document `core`
