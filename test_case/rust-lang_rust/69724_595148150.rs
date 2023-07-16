plain
2020-03-05T09:37:58.3087575Z    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
2020-03-05T09:38:00.7931206Z error[E0432]: unresolved import `rustc_ast::ast::Mac`
2020-03-05T09:38:00.7932267Z  --> src/tools/clippy/clippy_lints/src/non_expressive_names.rs:3:92
2020-03-05T09:38:00.7932829Z   |
2020-03-05T09:38:00.7933887Z 3 |     Arm, AssocItem, AssocItemKind, Attribute, Block, FnDecl, Ident, Item, ItemKind, Local, Mac, Pat, PatKind,
2020-03-05T09:38:00.7935165Z   |                                                                                            ^^^ no `Mac` in `ast`
2020-03-05T09:38:00.7978692Z error[E0432]: unresolved import `rustc_ast::ast::Mac`
2020-03-05T09:38:00.7979316Z  --> src/tools/clippy/clippy_lints/src/write.rs:5:38
2020-03-05T09:38:00.7979752Z   |
2020-03-05T09:38:00.7979752Z   |
2020-03-05T09:38:00.7980315Z 5 | use rustc_ast::ast::{Expr, ExprKind, Mac, StrLit, StrStyle};
2020-03-05T09:38:00.7981437Z   |                                      ^^^ no `Mac` in `ast`
2020-03-05T09:38:01.1108136Z error[E0412]: cannot find type `Mac` in module `ast`
2020-03-05T09:38:01.1108824Z   --> src/tools/clippy/clippy_lints/src/dbg_macro.rs:33:63
2020-03-05T09:38:01.1109273Z    |
2020-03-05T09:38:01.1109273Z    |
2020-03-05T09:38:01.1109906Z 33 |     fn check_mac(&mut self, cx: &EarlyContext<'_>, mac: &ast::Mac) {
2020-03-05T09:38:01.1111374Z    |
2020-03-05T09:38:01.1111993Z help: there is an enum variant `rustc_ast::ast::ExprPrecedence::Mac`; try using the variant's enum
2020-03-05T09:38:01.1112545Z    |
2020-03-05T09:38:01.1112545Z    |
2020-03-05T09:38:01.1113206Z 33 |     fn check_mac(&mut self, cx: &EarlyContext<'_>, mac: &rustc_ast::ast::ExprPrecedence) {
2020-03-05T09:38:01.1119527Z 
2020-03-05T09:38:03.0104568Z [RUSTC-TIMING] cargo_metadata test:false 20.905
2020-03-05T09:38:03.3697307Z error[E0599]: no variant or associated item named `Mac` found for enum `rustc_ast::ast::ExprKind` in the current scope
2020-03-05T09:38:03.3698204Z    --> src/tools/clippy/clippy_lints/src/utils/sugg.rs:159:30
---
2020-03-05T09:38:04.0339195Z     |
2020-03-05T09:38:04.0340237Z 288 |                 let body = cx.tcx.hir().body(eid);
2020-03-05T09:38:04.0341835Z     |                                              ^^^
2020-03-05T09:38:04.0343015Z     |                                              |
2020-03-05T09:38:04.0344924Z     |                                              expected struct `rustc_hir::BodyId`, found `&rustc_hir::BodyId`
2020-03-05T09:38:04.0346404Z     |                                              help: consider dereferencing the borrow: `*eid`
2020-03-05T09:38:04.1374892Z error[E0599]: no variant or associated item named `Method` found for enum `rustc_hir::TraitItemKind<'_>` in the current scope
2020-03-05T09:38:04.1376539Z   --> src/tools/clippy/clippy_lints/src/inline_fn_without_body.rs:35:31
2020-03-05T09:38:04.1379162Z    |
2020-03-05T09:38:04.1380120Z 35 |         if let TraitItemKind::Method(_, TraitMethod::Required(_)) = item.kind {
---
2020-03-05T09:38:04.2155512Z 
2020-03-05T09:38:04.4368224Z error[E0308]: mismatched types
2020-03-05T09:38:04.4368810Z    --> src/tools/clippy/clippy_lints/src/lifetimes.rs:108:43
2020-03-05T09:38:04.4369260Z     |
2020-03-05T09:38:04.4369842Z 108 |             check_fn_inner(cx, &sig.decl, body, &item.generics, item.span, true);
2020-03-05T09:38:04.4370747Z     |                                           ^^^^ expected struct `rustc_hir::BodyId`, found `&rustc_hir::BodyId`
2020-03-05T09:38:04.4371988Z     = note: expected enum `std::option::Option<rustc_hir::BodyId>`
2020-03-05T09:38:04.4372624Z                found enum `std::option::Option<&rustc_hir::BodyId>`
2020-03-05T09:38:04.4372892Z 
2020-03-05T09:38:04.9481109Z error[E0599]: no variant or associated item named `Method` found for enum `rustc_hir::def::DefKind` in the current scope
---
2020-03-05T10:17:30.7585861Z Verifying status of embedded-book...
2020-03-05T10:17:30.7586501Z This PR updated 'src/doc/embedded-book', verifying if status is 'test-pass'...
2020-03-05T10:17:30.7587114Z Verifying status of rustc-guide...
2020-03-05T10:17:30.7634348Z Cloning into 'rust-toolstate'...
2020-03-05T10:17:31.4160977Z error: Tool `clippy-driver` has regressed from test-fail to build-fail during beta week.
2020-03-05T10:17:31.4178215Z Build completed unsuccessfully in 0:00:02
2020-03-05T10:17:31.4240512Z == clock drift check ==
2020-03-05T10:17:31.4251889Z   local time: Thu Mar  5 10:17:31 UTC 2020
2020-03-05T10:17:31.6904248Z   network time: Thu, 05 Mar 2020 10:17:31 GMT
2020-03-05T10:17:31.6904248Z   network time: Thu, 05 Mar 2020 10:17:31 GMT
2020-03-05T10:17:31.6904603Z == end clock drift check ==
2020-03-05T10:17:32.0101861Z 
2020-03-05T10:17:32.0177781Z ##[error]Bash exited with code '1'.
2020-03-05T10:17:32.0249445Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-05T10:17:32.0253626Z ==============================================================================
2020-03-05T10:17:32.0253944Z Task         : Get sources
2020-03-05T10:17:32.0254263Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
