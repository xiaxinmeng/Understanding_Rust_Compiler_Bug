shell
$ rustc --edition=2021 main.rs 
error[E0412]: cannot find type `Duration` in this scope
 --> main.rs:9:19
  |
9 |         interval: Duration,
  |                   ^^^^^^^^ not found in this scope
  |
help: consider importing one of these items
  |
3 | use core::time::Duration;
  |
3 | use std::time::Duration;
  |

error: internal compiler error: compiler/rustc_typeck/src/check/upvar.rs:1701:13: Drop location span error: need to handle more Node ImplItem(ImplItem { ident: new#0, def_id: DefId(0:6 ~ main[a756]::{impl#0}::new), vis: Spanned { node: Crate(PubCrate), span: main.rs:8:5: 8:15 (#0) }, defaultness: Final, generics: Generics { params: [], where_clause: WhereClause { predicates: [], span: main.rs:10:18: 10:18 (#0) }, span: main.rs:8:28: 8:28 (#0) }, kind: Fn(FnSig { header: FnHeader { unsafety: Normal, constness: NotConst, asyncness: Async, abi: Rust }, decl: FnDecl { inputs: [Ty { hir_id: HirId { owner: DefId(0:6 ~ main[a756]::{impl#0}::new), local_id: 20 }, kind: Path(Resolved(None, Path { span: main.rs:9:19: 9:27 (#0), res: Err, segments: [PathSegment { ident: Duration#0, hir_id: Some(HirId { owner: DefId(0:6 ~ main[a756]::{impl#0}::new), local_id: 21 }), res: Some(Err), args: None, infer_args: false }] })), span: main.rs:9:19: 9:27 (#0) }], output: Return(Ty { hir_id: HirId { owner: DefId(0:6 ~ main[a756]::{impl#0}::new), local_id: 22 }, kind: OpaqueDef(ItemId { def_id: DefId(0:7 ~ main[a756]::{impl#0}::new::{opaque#0}) }, []), span: main.rs:10:10: 10:18 (#82) }), c_variadic: false, implicit_self: None }, span: main.rs:8:5: 10:18 (#0) }, BodyId { hir_id: HirId { owner: DefId(0:6 ~ main[a756]::{impl#0}::new), local_id: 19 } }), span: main.rs:8:5: 12:6 (#0) })

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0 (9d1b2106e 2022-02-23) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [typeck] type-checking `<impl at main.rs:7:1: 13:2>::new`
#1 [mir_built] building MIR for `<impl at main.rs:7:1: 13:2>::new`
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0412`.
