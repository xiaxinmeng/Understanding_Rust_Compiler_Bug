plain
travis_time:end:305351ec:start=1550383286925904133,finish=1550383289370844663,duration=2444940530
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:08:21]    |
[00:08:21] 34 |                            body: &hir::Body) -> CFG {
[00:08:21]    |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21]    |
[00:08:21]    = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/cfg/construct.rs:344:50
[00:08:21]     |
[00:08:21]     |
[00:08:21] 344 |                 self.call(expr, pred, &e, None::<hir::Expr>.iter())
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/cfg/construct.rs:387:54
[00:08:21]     |
[00:08:21]     |
[00:08:21] 387 |                 self.straightline(expr, pred, None::<hir::Expr>.iter())
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/cfg/mod.rs:53:33
[00:08:21]    |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/check_attr.rs:95:39
[00:08:21]    |
[00:08:21] 95 |     fn check_attributes(&self, item: &hir::Item, target: Target) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/check_attr.rs:131:66
[00:08:21]     |
[00:08:21]     |
[00:08:21] 131 |     fn check_non_exhaustive(&self, attr: &hir::Attribute, item: &hir::Item, target: Target) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/check_attr.rs:147:58
[00:08:21]     |
[00:08:21]     |
[00:08:21] 147 |     fn check_marker(&self, attr: &hir::Attribute, item: &hir::Item, target: Target) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/check_attr.rs:161:33
[00:08:21]     |
[00:08:21]     |
[00:08:21] 161 |     fn check_repr(&self, item: &hir::Item, target: Target) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/check_attr.rs:275:44
[00:08:21]     |
[00:08:21]     |
[00:08:21] 275 |     fn check_stmt_attributes(&self, stmt: &hir::Stmt) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/check_attr.rs:294:44
[00:08:21]     |
[00:08:21]     |
[00:08:21] 294 |     fn check_expr_attributes(&self, expr: &hir::Expr) {
[00:08:21]     |                                            ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/check_attr.rs:314:33
[00:08:21]     |
[00:08:21] 314 |     fn check_used(&self, item: &hir::Item, target: Target) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/check_attr.rs:329:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 329 |     fn visit_item(&mut self, item: &'tcx hir::Item) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/check_attr.rs:336:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 336 |     fn visit_stmt(&mut self, stmt: &'tcx hir::Stmt) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/check_attr.rs:341:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 341 |     fn visit_expr(&mut self, expr: &'tcx hir::Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/check_attr.rs:353:26
[00:08:21]     |
[00:08:21]     |
[00:08:21] 353 | fn is_c_like_enum(item: &hir::Item) -> bool {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/def.rs:235:29
[00:08:21]     |
[00:08:21]     |
[00:08:21] 235 |     pub fn from_hir(vdata: &hir::VariantData) -> CtorKind {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/intravisit.rs:321:44
[00:08:21]     |
[00:08:21]     |
[00:08:21] 321 |                       enum_definition: &'v EnumDef,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/intravisit.rs:342:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 342 |     fn visit_qpath(&mut self, qpath: &'v QPath, id: HirId, span: Span) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/intravisit.rs:362:38
[00:08:21]     |
[00:08:21]     |
[00:08:21] 362 |     fn visit_vis(&mut self, vis: &'v Visibility) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/intravisit.rs:578:62
[00:08:21]     |
[00:08:21]     |
[00:08:21] 578 | pub fn walk_ty<'v, V: Visitor<'v>>(visitor: &mut V, typ: &'v Ty) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/intravisit.rs:772:20
[00:08:21]     |
[00:08:21]     |
[00:08:21] 772 |     predicate: &'v WherePredicate)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/intravisit.rs:825:62
[00:08:21]     |
[00:08:21]     |
[00:08:21] 825 |                                    function_declaration: &'v FnDecl,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/itemlikevisit.rs:69:42
[00:08:21]    |
[00:08:21] 69 |     fn visit_item(&mut self, item: &'hir Item) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/itemlikevisit.rs:73:54
[00:08:21]    |
[00:08:21] 73 |     fn visit_trait_item(&mut self, trait_item: &'hir TraitItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/itemlikevisit.rs:77:52
[00:08:21]    |
[00:08:21] 77 |     fn visit_impl_item(&mut self, impl_item: &'hir ImplItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/itemlikevisit.rs:84:38
[00:08:21]    |
[00:08:21] 84 |     fn visit_item(&self, item: &'hir Item);
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/itemlikevisit.rs:85:50
[00:08:21]    |
[00:08:21] 85 |     fn visit_trait_item(&self, trait_item: &'hir TraitItem);
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/itemlikevisit.rs:86:48
[00:08:21]    |
[00:08:21] 86 |     fn visit_impl_item(&self, impl_item: &'hir ImplItem);
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/itemlikevisit.rs:99:38
[00:08:21]    |
[00:08:21] 99 |     fn visit_item(&self, item: &'hir Item) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/itemlikevisit.rs:103:50
[00:08:21]     |
[00:08:21] 103 |     fn visit_trait_item(&self, trait_item: &'hir TraitItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/itemlikevisit.rs:107:48
[00:08:21]     |
[00:08:21] 107 |     fn visit_impl_item(&self, impl_item: &'hir ImplItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/lowering.rs:898:25
[00:08:21]     |
[00:08:21]     |
[00:08:21] 898 |         params: &HirVec<hir::GenericParam>,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/lowering.rs:898:18
[00:08:21]     |
[00:08:21]     |
[00:08:21] 898 |         params: &HirVec<hir::GenericParam>,
[00:08:21]     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `HirVec<'_, hir::GenericParam>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/lowering.rs:1494:18
[00:08:21]      |
[00:08:21] 1494 |         bounds: &hir::GenericBounds,
[00:08:21] 1494 |         bounds: &hir::GenericBounds,
[00:08:21]      |                  ^^^^^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/lowering.rs:3384:53
[00:08:21]      |
[00:08:21] 3384 |     fn renumber_segment_ids(&mut self, path: &P<'a, hir::Path>) -> P<'a, hir::Path<'a>> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/lowering.rs:4765:40
[00:08:21]      |
[00:08:21]      |
[00:08:21] 4765 |                 let mut ids: SmallVec<[hir::Stmt; 1]> = item_ids
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/lowering.rs:5397:44
[00:08:21]      |
[00:08:21]      |
[00:08:21] 5397 | fn body_ids(bodies: &BTreeMap<hir::BodyId, hir::Body>) -> Vec<hir::BodyId> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/blocks.rs:123:19
[00:08:21]     |
[00:08:21]     |
[00:08:21] 123 |     fn new(d: &'a FnDecl, b: ast::BodyId, id: NodeId, s: Span, attrs: &'a [Attribute]) -> Self {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/blocks.rs:155:35
[00:08:21]     |
[00:08:21]     |
[00:08:21] 155 |                     |_, _, _: &'a ast::MethodSig, _, body: ast::BodyId, _, _| body,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/blocks.rs:161:37
[00:08:21]     |
[00:08:21]     |
[00:08:21] 161 |                     |_, _, sig: &'a ast::MethodSig, _, _, _, _| &sig.decl,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/blocks.rs:167:35
[00:08:21]     |
[00:08:21]     |
[00:08:21] 167 |                     |_, _, _: &'a ast::MethodSig, _, _, span, _| span,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/blocks.rs:173:36
[00:08:21]     |
[00:08:21]     |
[00:08:21] 173 |                     |id, _, _: &'a ast::MethodSig, _, _, _, _| id,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:100:37
[00:08:21]     |
[00:08:21]     |
[00:08:21] 100 |                        krate: &'hir Crate,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:350:39
[00:08:21]     |
[00:08:21]     |
[00:08:21] 350 |     fn visit_item(&mut self, i: &'hir Item) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:368:58
[00:08:21]     |
[00:08:21]     |
[00:08:21] 368 |     fn visit_foreign_item(&mut self, foreign_item: &'hir ForeignItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:376:52
[00:08:21]     |
[00:08:21]     |
[00:08:21] 376 |     fn visit_generic_param(&mut self, param: &'hir GenericParam) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:381:46
[00:08:21]     |
[00:08:21]     |
[00:08:21] 381 |     fn visit_trait_item(&mut self, ti: &'hir TraitItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:393:45
[00:08:21]     |
[00:08:21]     |
[00:08:21] 393 |     fn visit_impl_item(&mut self, ii: &'hir ImplItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:405:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 405 |     fn visit_pat(&mut self, pat: &'hir Pat) {
[00:08:21]     |                                        ^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:426:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 426 |     fn visit_expr(&mut self, expr: &'hir Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:434:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 434 |     fn visit_stmt(&mut self, stmt: &'hir Stmt) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:443:75
[00:08:21]     |
[00:08:21]     |
[00:08:21] 443 |     fn visit_path_segment(&mut self, path_span: Span, path_segment: &'hir PathSegment) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:450:38
[00:08:21]     |
[00:08:21]     |
[00:08:21] 450 |     fn visit_ty(&mut self, ty: &'hir Ty) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:458:45
[00:08:21]     |
[00:08:21]     |
[00:08:21] 458 |     fn visit_trait_ref(&mut self, tr: &'hir TraitRef) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:466:68
[00:08:21]     |
[00:08:21]     |
[00:08:21] 466 |     fn visit_fn(&mut self, fk: intravisit::FnKind<'hir>, fd: &'hir FnDecl,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:472:44
[00:08:21]     |
[00:08:21]     |
[00:08:21] 472 |     fn visit_block(&mut self, block: &'hir Block) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:479:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 479 |     fn visit_local(&mut self, l: &'hir Local) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:490:47
[00:08:21]     |
[00:08:21]     |
[00:08:21] 490 |     fn visit_vis(&mut self, visibility: &'hir Visibility) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:504:52
[00:08:21]     |
[00:08:21]     |
[00:08:21] 504 |     fn visit_macro_def(&mut self, macro_def: &'hir MacroDef) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:512:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 512 |     fn visit_variant(&mut self, v: &'hir Variant, g: &'hir Generics, item_id: NodeId) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:512:60
[00:08:21]     |
[00:08:21]     |
[00:08:21] 512 |     fn visit_variant(&mut self, v: &'hir Variant, g: &'hir Generics, item_id: NodeId) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:520:51
[00:08:21]     |
[00:08:21]     |
[00:08:21] 520 |     fn visit_struct_field(&mut self, field: &'hir StructField) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/collector.rs:541:49
[00:08:21]     |
[00:08:21]     |
[00:08:21] 541 |     fn visit_impl_item_ref(&mut self, ii: &'hir ImplItemRef) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/map/hir_id_validator.rs:56:39
[00:08:21]    |
[00:08:21] 56 |     fn visit_item(&mut self, i: &'hir hir::Item) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/map/hir_id_validator.rs:61:45
[00:08:21]    |
[00:08:21] 61 |     fn visit_trait_item(&mut self, i: &'hir hir::TraitItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/map/hir_id_validator.rs:66:44
[00:08:21]    |
[00:08:21] 66 |     fn visit_impl_item(&mut self, i: &'hir hir::ImplItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/hir_id_validator.rs:181:48
[00:08:21]     |
[00:08:21] 181 |     fn visit_impl_item_ref(&mut self, _: &'hir hir::ImplItemRef) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/mod.rs:420:34
[00:08:21]     |
[00:08:21]     |
[00:08:21] 420 |     pub fn krate(&self) -> &'hir Crate {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/mod.rs:457:62
[00:08:21]     |
[00:08:21]     |
[00:08:21] 457 |     pub fn fn_decl_by_hir_id(&self, hir_id: HirId) -> Option<FnDecl> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/mod.rs:583:55
[00:08:21]     |
[00:08:21]     |
[00:08:21] 583 |     pub fn get_module(&self, module: DefId) -> (&'hir Mod, Span, NodeId)
[00:08:21]     |                                                       ^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/map/mod.rs:995:61
[00:08:21]     |
[00:08:21]     |
[00:08:21] 995 |     pub fn expect_expr_by_hir_id(&self, id: HirId) -> &'hir Expr {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/map/mod.rs:1184:35
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1184 |             fn item_is_mod(item: &Item) -> bool {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/map/mod.rs:1241:38
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1241 |                        forest: &'hir Forest,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/print.rs:51:57
[00:08:21]    |
[00:08:21]    |
[00:08:21] 51 |     fn try_fetch_item(&self, _: ast::NodeId) -> Option<&hir::Item> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/hir/print.rs:61:60
[00:08:21]    |
[00:08:21]    |
[00:08:21] 61 |     fn try_fetch_item(&self, item: ast::NodeId) -> Option<&hir::Item> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:123:32
[00:08:21]     |
[00:08:21]     |
[00:08:21] 123 |                        krate: &hir::Crate,
[00:08:21]     |                                ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:203:63
[00:08:21]     |
[00:08:21] 203 | pub fn visibility_qualified<S: Into<Cow<'static, str>>>(vis: &hir::Visibility, w: S) -> String {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:328:58
[00:08:21]     |
[00:08:21]     |
[00:08:21] 328 |     pub fn commasep_exprs(&mut self, b: Breaks, exprs: &[hir::Expr]) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:332:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 332 |     pub fn print_mod(&mut self, _mod: &hir::Mod, attrs: &[ast::Attribute]) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:341:37
[00:08:21]     |
[00:08:21]     |
[00:08:21] 341 |                              nmod: &hir::ForeignMod,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:359:39
[00:08:21]     |
[00:08:21]     |
[00:08:21] 359 |     pub fn print_type(&mut self, ty: &hir::Ty) -> io::Result<()> {
[00:08:21]     |                                       ^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:441:49
[00:08:21]     |
[00:08:21] 441 |     pub fn print_foreign_item(&mut self, item: &hir::ForeignItem) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:488:36
[00:08:21]     |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:507:46
[00:08:21]     |
[00:08:21] 507 |                              bounds: Option<&hir::GenericBounds>,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:508:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 508 |                              ty: Option<&hir::Ty>)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:524:41
[00:08:21]     |
[00:08:21]     |
[00:08:21] 524 |     pub fn print_item(&mut self, item: &hir::Item) -> io::Result<()> {
[00:08:21]     |                                         ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:765:43
[00:08:21]     |
[00:08:21] 765 |     pub fn print_trait_ref(&mut self, t: &hir::TraitRef) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:771:27
[00:08:21]     |
[00:08:21]     |
[00:08:21] 771 |         generic_params: &[hir::GenericParam]
[00:08:21]     |                           ^^^^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:781:44
[00:08:21]     |
[00:08:21] 781 |     fn print_poly_trait_ref(&mut self, t: &hir::PolyTraitRef) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:787:45
[00:08:21]     |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:819:46
[00:08:21]     |
[00:08:21] 819 |     pub fn print_visibility(&mut self, vis: &hir::Visibility) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:852:38
[00:08:21]     |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:898:41
[00:08:21]     |
[00:08:21] 898 |     pub fn print_variant(&mut self, v: &hir::Variant) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:911:33
[00:08:21]     |
[00:08:21]     |
[00:08:21] 911 |                             m: &hir::MethodSig,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:912:40
[00:08:21]     |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:926:45
[00:08:21]     |
[00:08:21] 926 |     pub fn print_trait_item(&mut self, ti: &hir::TraitItem) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:962:44
[00:08:21]     |
[00:08:21]     |
[00:08:21] 962 |     pub fn print_impl_item(&mut self, ii: &hir::ImplItem) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/print.rs:992:39
[00:08:21]     |
[00:08:21]     |
[00:08:21] 992 |     pub fn print_stmt(&mut self, st: &hir::Stmt) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1029:41
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1029 |     pub fn print_block(&mut self, blk: &hir::Block) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1033:50
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1033 |     pub fn print_block_unclosed(&mut self, blk: &hir::Block) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1038:46
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1081:43
[00:08:21]      |
[00:08:21] 1081 |     fn print_else(&mut self, els: Option<&hir::Expr>) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1113:28
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1115:38
[00:08:21]      |
[00:08:21] 1115 |                     elseopt: Option<&hir::Expr>)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1125:31
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1128:42
[00:08:21]      |
[00:08:21] 1128 |                         elseopt: Option<&hir::Expr>)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1144:43
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1144 |     fn print_call_post(&mut self, args: &[hir::Expr]) -> io::Result<()> {
[00:08:21]      |                                           ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1150:53
[00:08:21]      |
[00:08:21] 1150 |     pub fn print_expr_maybe_paren(&mut self, expr: &hir::Expr, prec: i8) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1164:49
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1164 |     pub fn print_expr_as_cond(&mut self, expr: &hir::Expr) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1185:43
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1185 |     fn print_expr_vec(&mut self, exprs: &[hir::Expr]) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1193:47
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1193 |     fn print_expr_repeat(&mut self, element: &hir::Expr, count: &hir::AnonConst) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1204:34
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1204 |                          qpath: &hir::QPath,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1205:36
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1205 |                          fields: &[hir::Field],
[00:08:21]      |                                    ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1206:41
[00:08:21]      |
[00:08:21] 1206 |                          wth: &Option<P<hir::Expr>>)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1206:39
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1206 |                          wth: &Option<P<hir::Expr>>)
[00:08:21]      |                                       ^^^^^^^^^^^^ help: indicate the anonymous lifetime: `P<'_, hir::Expr>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1241:43
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1241 |     fn print_expr_tup(&mut self, exprs: &[hir::Expr]) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1250:42
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1250 |     fn print_expr_call(&mut self, func: &hir::Expr, args: &[hir::Expr]) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1250:61
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1250 |     fn print_expr_call(&mut self, func: &hir::Expr, args: &[hir::Expr]) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1262:41
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1309:58
[00:08:21]      |
[00:08:21] 1309 |     fn print_expr_unary(&mut self, op: hir::UnOp, expr: &hir::Expr) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1316:34
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1571:46
[00:08:21]      |
[00:08:21] 1571 |     pub fn print_local_decl(&mut self, loc: &hir::Local) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1597:44
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1597 |     pub fn print_for_decl(&mut self, loc: &hir::Local, coll: &hir::Expr) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1597:63
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1597 |     pub fn print_for_decl(&mut self, loc: &hir::Local, coll: &hir::Expr) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1605:30
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1605 |                       path: &hir::Path,
[00:08:21]      |                              ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1626:52
[00:08:21]      |
[00:08:21] 1626 |     pub fn print_path_segment(&mut self, segment: &hir::PathSegment) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1637:32
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1637 |                        qpath: &hir::QPath,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1690:45
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1690 |                              generic_args: &hir::GenericArgs,
[00:08:21]      |                                             ^^^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1764:39
[00:08:21]      |
[00:08:21] 1764 |     pub fn print_pat(&mut self, pat: &hir::Pat) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1923:35
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1923 |     fn print_arm(&mut self, arm: &hir::Arm) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:1978:28
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2022:45
[00:08:21]      |
[00:08:21] 2022 |     fn print_closure_args(&mut self, decl: &hir::FnDecl, body_id: hir::BodyId) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2064:68
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2064 |     pub fn print_bounds(&mut self, prefix: &'static str, bounds: &[hir::GenericBound])
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2095:62
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2095 |     pub fn print_generic_params(&mut self, generic_params: &[GenericParam]) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2108:51
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2108 |     pub fn print_generic_param(&mut self, param: &GenericParam) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2143:57
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2143 |     pub fn print_where_clause(&mut self, where_clause: &hir::WhereClause) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2207:37
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2207 |     pub fn print_mt(&mut self, mt: &hir::MutTy) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2212:46
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2212 |     pub fn print_fn_output(&mut self, decl: &hir::FnDecl) -> io::Result<()> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2235:31
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2380:38
[00:08:21]      |
[00:08:21] 2380 | fn expr_requires_semi_to_be_stmt(e: &hir::Expr) -> bool {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2394:31
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2394 | fn stmt_ends_with_semi(stmt: &hir::StmtKind) -> bool {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/print.rs:2433:41
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2433 | fn contains_exterior_struct_lit(value: &hir::Expr) -> bool {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/mod.rs:599:63
[00:08:21]     |
[00:08:21]     |
[00:08:21] 599 |     pub fn get_named(&self, name: &InternedString) -> Option<&GenericParam> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/mod.rs:735:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 735 |     pub fn item(&self, id: NodeId) -> &Item {
[00:08:21]     |                                        ^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/mod.rs:739:51
[00:08:21]     |
[00:08:21] 739 |     pub fn trait_item(&self, id: TraitItemId) -> &TraitItem {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/mod.rs:743:49
[00:08:21]     |
[00:08:21]     |
[00:08:21] 743 |     pub fn impl_item(&self, id: ImplItemId) -> &ImplItem {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/mod.rs:790:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 790 |     pub fn body(&self, id: BodyId) -> &Body {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/mod.rs:852:25
[00:08:21]     |
[00:08:21]     |
[00:08:21] 852 |         where G: FnMut(&Pat) -> bool
[00:08:21]     |                         ^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/hir/mod.rs:886:25
[00:08:21]     |
[00:08:21]     |
[00:08:21] 886 |         where F: FnMut(&Pat) -> bool
[00:08:21]     |                         ^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/hir/mod.rs:2341:39
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2341 |     pub fn generics(&self) -> Option<&Generics> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/ich/hcx.rs:81:27
[00:08:21] 81 |                krate: &'a hir::Crate,
[00:08:21]    |                           ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/infer/error_reporting/need_type_info.rs:48:44
[00:08:21]    |
[00:08:21] 48 |     fn visit_local(&mut self, local: &'gcx Local) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/infer/error_reporting/need_type_info.rs:55:42
[00:08:21]    |
[00:08:21]    |
[00:08:21] 55 |     fn visit_body(&mut self, body: &'gcx Body) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/infer/error_reporting/nice_region_error/find_anon_type.rs:63:20
[00:08:21]    |
[00:08:21]    |
[00:08:21] 63 |         arg: &'gcx hir::Ty,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/infer/error_reporting/nice_region_error/find_anon_type.rs:100:39
[00:08:21]     |
[00:08:21]     |
[00:08:21] 100 |     fn visit_ty(&mut self, arg: &'gcx hir::Ty) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/infer/error_reporting/nice_region_error/find_anon_type.rs:270:39
[00:08:21]     |
[00:08:21]     |
[00:08:21] 270 |     fn visit_ty(&mut self, arg: &'gcx hir::Ty) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/infer/error_reporting/nice_region_error/util.rs:109:16
[00:08:21]     |
[00:08:21]     |
[00:08:21] 109 |         decl: &hir::FnDecl,
[00:08:21]     |                ^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/infer/error_reporting/mod.rs:252:30
[00:08:21]     |
[00:08:21] 252 |     fn item_scope_tag(item: &hir::Item) -> &'static str {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/infer/error_reporting/mod.rs:264:36
[00:08:21]     |
[00:08:21]     |
[00:08:21] 264 |     fn trait_item_scope_tag(item: &hir::TraitItem) -> &'static str {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/infer/error_reporting/mod.rs:271:35
[00:08:21]     |
[00:08:21]     |
[00:08:21] 271 |     fn impl_item_scope_tag(item: &hir::ImplItem) -> &'static str {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:180:36
[00:08:21]     |
[00:08:21]     |
[00:08:21] 180 |             fn check_body(a: &$hir hir::Body);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:181:41
[00:08:21]     |
[00:08:21]     |
[00:08:21] 181 |             fn check_body_post(a: &$hir hir::Body);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:183:37
[00:08:21]     |
[00:08:21]     |
[00:08:21] 183 |             fn check_crate(a: &$hir hir::Crate);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:184:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 184 |             fn check_crate_post(a: &$hir hir::Crate);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:185:35
[00:08:21]     |
[00:08:21]     |
[00:08:21] 185 |             fn check_mod(a: &$hir hir::Mod, b: Span, c: ast::NodeId);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:186:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 186 |             fn check_mod_post(a: &$hir hir::Mod, b: Span, c: ast::NodeId);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:187:44
[00:08:21]     |
[00:08:21]     |
[00:08:21] 187 |             fn check_foreign_item(a: &$hir hir::ForeignItem);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:188:49
[00:08:21]     |
[00:08:21]     |
[00:08:21] 188 |             fn check_foreign_item_post(a: &$hir hir::ForeignItem);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:189:36
[00:08:21]     |
[00:08:21]     |
[00:08:21] 189 |             fn check_item(a: &$hir hir::Item);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:190:41
[00:08:21]     |
[00:08:21]     |
[00:08:21] 190 |             fn check_item_post(a: &$hir hir::Item);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:191:37
[00:08:21]     |
[00:08:21]     |
[00:08:21] 191 |             fn check_local(a: &$hir hir::Local);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:192:37
[00:08:21]     |
[00:08:21]     |
[00:08:21] 192 |             fn check_block(a: &$hir hir::Block);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:193:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 193 |             fn check_block_post(a: &$hir hir::Block);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:194:36
[00:08:21]     |
[00:08:21]     |
[00:08:21] 194 |             fn check_stmt(a: &$hir hir::Stmt);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:195:35
[00:08:21]     |
[00:08:21]     |
[00:08:21] 195 |             fn check_arm(a: &$hir hir::Arm);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:196:35
[00:08:21]     |
[00:08:21]     |
[00:08:21] 196 |             fn check_pat(a: &$hir hir::Pat);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:197:36
[00:08:21]     |
[00:08:21]     |
[00:08:21] 197 |             fn check_expr(a: &$hir hir::Expr);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:198:41
[00:08:21]     |
[00:08:21]     |
[00:08:21] 198 |             fn check_expr_post(a: &$hir hir::Expr);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:199:34
[00:08:21]     |
[00:08:21]     |
[00:08:21] 199 |             fn check_ty(a: &$hir hir::Ty);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:200:45
[00:08:21]     |
[00:08:21]     |
[00:08:21] 200 |             fn check_generic_param(a: &$hir hir::GenericParam);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:201:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 201 |             fn check_generics(a: &$hir hir::Generics);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:202:47
[00:08:21]     |
[00:08:21]     |
[00:08:21] 202 |             fn check_where_predicate(a: &$hir hir::WherePredicate);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:203:46
[00:08:21]     |
[00:08:21]     |
[00:08:21] 203 |             fn check_poly_trait_ref(a: &$hir hir::PolyTraitRef, b: hir::TraitBoundModifier);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:206:26
[00:08:21]     |
[00:08:21]     |
[00:08:21] 206 |                 b: &$hir hir::FnDecl,
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:207:26
[00:08:21]     |
[00:08:21]     |
[00:08:21] 207 |                 c: &$hir hir::Body,
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:212:26
[00:08:21]     |
[00:08:21]     |
[00:08:21] 212 |                 b: &$hir hir::FnDecl,
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:213:26
[00:08:21]     |
[00:08:21]     |
[00:08:21] 213 |                 c: &$hir hir::Body,
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:217:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 217 |             fn check_trait_item(a: &$hir hir::TraitItem);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:218:47
[00:08:21]     |
[00:08:21]     |
[00:08:21] 218 |             fn check_trait_item_post(a: &$hir hir::TraitItem);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:219:41
[00:08:21]     |
[00:08:21]     |
[00:08:21] 219 |             fn check_impl_item(a: &$hir hir::ImplItem);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:220:46
[00:08:21]     |
[00:08:21]     |
[00:08:21] 220 |             fn check_impl_item_post(a: &$hir hir::ImplItem);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:222:26
[00:08:21]     |
[00:08:21]     |
[00:08:21] 222 |                 a: &$hir hir::VariantData,
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:224:26
[00:08:21]     |
[00:08:21]     |
[00:08:21] 224 |                 c: &$hir hir::Generics,
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:228:26
[00:08:21]     |
[00:08:21]     |
[00:08:21] 228 |                 a: &$hir hir::VariantData,
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:230:26
[00:08:21]     |
[00:08:21]     |
[00:08:21] 230 |                 c: &$hir hir::Generics,
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:233:44
[00:08:21]     |
[00:08:21]     |
[00:08:21] 233 |             fn check_struct_field(a: &$hir hir::StructField);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:234:39
[00:08:21]     |
[00:08:21]     |
[00:08:21] 234 |             fn check_variant(a: &$hir hir::Variant, b: &$hir hir::Generics);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:234:62
[00:08:21]     |
[00:08:21]     |
[00:08:21] 234 |             fn check_variant(a: &$hir hir::Variant, b: &$hir hir::Generics);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:235:44
[00:08:21]     |
[00:08:21]     |
[00:08:21] 235 |             fn check_variant_post(a: &$hir hir::Variant, b: &$hir hir::Generics);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:235:67
[00:08:21]     |
[00:08:21]     |
[00:08:21] 235 |             fn check_variant_post(a: &$hir hir::Variant, b: &$hir hir::Generics);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/mod.rs:237:36
[00:08:21]     |
[00:08:21]     |
[00:08:21] 237 |             fn check_path(a: &$hir hir::Path, b: hir::HirId);
[00:08:21] ...
[00:08:21] ...
[00:08:21] 273 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:08:21]     | ------------------------------------------------------- in this macro invocation
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/lint/context.rs:940:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 940 |     fn visit_local(&mut self, l: &'tcx hir::Local) {
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/expr_use_visitor.rs:645:43
[00:08:21]     |
[00:08:21] 645 |                         opt_with: &Option<P<hir::Expr<'_>>>) {
[00:08:21]     |                                           ^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `P<'_, hir::Expr<'_>>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:102:45
[00:08:21]     |
[00:08:21] 102 |     fn handle_field_access(&mut self, lhs: &hir::Expr, node_id: ast::NodeId) {
[00:08:21] 102 |     fn handle_field_access(&mut self, lhs: &hir::Expr, node_id: ast::NodeId) {
[00:08:21]     |                                             ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:113:52
[00:08:21]     |
[00:08:21] 113 |     fn handle_field_pattern_match(&mut self, lhs: &hir::Pat, def: Def,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:114:63
[00:08:21]     |
[00:08:21]     |
[00:08:21] 114 |                                   pats: &[source_map::Spanned<hir::FieldPat>]) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:190:80
[00:08:21]     |
[00:08:21]     |
[00:08:21] 190 |     fn mark_as_used_if_union(&mut self, adt: &ty::AdtDef, fields: &hir::HirVec<hir::Field>) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:190:68
[00:08:21]     |
[00:08:21]     |
[00:08:21] 190 |     fn mark_as_used_if_union(&mut self, adt: &ty::AdtDef, fields: &hir::HirVec<hir::Field>) {
[00:08:21]     |                                                                    ^^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `hir::HirVec<'_, hir::Field>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:213:49
[00:08:21]     |
[00:08:21]     |
[00:08:21] 213 |     fn visit_variant_data(&mut self, def: &'tcx hir::VariantData, _: ast::Name,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:214:31
[00:08:21]     |
[00:08:21]     |
[00:08:21] 214 |                           _: &hir::Generics, _: ast::NodeId, _: syntax_pos::Span) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:225:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 225 |     fn visit_expr(&mut self, expr: &'tcx hir::Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:248:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 248 |     fn visit_arm(&mut self, arm: &'tcx hir::Arm) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:264:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 264 |     fn visit_pat(&mut self, pat: &'tcx hir::Pat) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:281:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 281 |     fn visit_path(&mut self, path: &'tcx hir::Path, _: hir::HirId) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:344:37
[00:08:21]     |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:467:51
[00:08:21]     |
[00:08:21] 467 |     fn should_warn_about_field(&mut self, field: &hir::StructField) -> bool {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:475:55
[00:08:21]     |
[00:08:21]     |
[00:08:21] 475 |     fn should_warn_about_variant(&mut self, variant: &hir::VariantKind) -> bool {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:482:55
[00:08:21]     |
[00:08:21]     |
[00:08:21] 482 |     fn should_warn_about_foreign_item(&mut self, fi: &hir::ForeignItem) -> bool {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:539:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 539 |     fn visit_item(&mut self, item: &'tcx hir::Item) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:571:37
[00:08:21]     |
[00:08:21]     |
[00:08:21] 571 |                      variant: &'tcx hir::Variant,
[00:08:21]     |                                     ^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:572:31
[00:08:21]     |
[00:08:21] 572 |                      g: &'tcx hir::Generics,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:582:48
[00:08:21]     |
[00:08:21]     |
[00:08:21] 582 |     fn visit_foreign_item(&mut self, fi: &'tcx hir::ForeignItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:590:51
[00:08:21]     |
[00:08:21]     |
[00:08:21] 590 |     fn visit_struct_field(&mut self, field: &'tcx hir::StructField) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:597:52
[00:08:21]     |
[00:08:21]     |
[00:08:21] 597 |     fn visit_impl_item(&mut self, impl_item: &'tcx hir::ImplItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/dead.rs:622:54
[00:08:21]     |
[00:08:21]     |
[00:08:21] 622 |     fn visit_trait_item(&mut self, trait_item: &'tcx hir::TraitItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/middle/entry.rs:34:42
[00:08:21]    |
[00:08:21]    |
[00:08:21] 34 |     fn visit_item(&mut self, item: &'tcx Item) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/middle/entry.rs:41:55
[00:08:21]    |
[00:08:21]    |
[00:08:21] 41 |     fn visit_trait_item(&mut self, _trait_item: &'tcx TraitItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/middle/entry.rs:45:53
[00:08:21]    |
[00:08:21]    |
[00:08:21] 45 |     fn visit_impl_item(&mut self, _impl_item: &'tcx ImplItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/middle/entry.rs:82:28
[00:08:21]    |
[00:08:21]    |
[00:08:21] 82 | fn entry_point_type(item: &Item, at_root: bool) -> EntryPointType {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/entry.rs:105:21
[00:08:21]     |
[00:08:21]     |
[00:08:21] 105 | fn find_item(item: &Item, ctxt: &mut EntryContext<'_, '_>, at_root: bool) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/intrinsicck.rs:160:42
[00:08:21]     |
[00:08:21] 160 |     fn visit_expr(&mut self, expr: &'tcx hir::Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/lang_items.rs:97:37
[00:08:21]     |
[00:08:21]     |
[00:08:21] 97  |       fn visit_item(&mut self, item: &hir::Item) {
[00:08:21]     |                                       ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] ...
[00:08:21] 226 | / language_item_table! {
[00:08:21] 227 | | //  Variant name,                Name,                 Method name,             Target;
[00:08:21] 228 | |     CharImplItem,                "char",               char_impl,               Target::Impl;
[00:08:21] 229 | |     StrImplItem,                 "str",                str_impl,                Target::Impl;
[00:08:21] ...   |
[00:08:21] 381 | |     Rc,                          "rc",                 rc,                      Target::Struct;
[00:08:21]     | |_- in this macro invocation
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/lang_items.rs:139:50
[00:08:21]    --> src/librustc/middle/lang_items.rs:139:50
[00:08:21]     |
[00:08:21] 139 |       fn visit_trait_item(&mut self, _trait_item: &hir::TraitItem) {
[00:08:21]     |                                                    ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] ...
[00:08:21] 226 | / language_item_table! {
[00:08:21] 227 | | //  Variant name,                Name,                 Method name,             Target;
[00:08:21] 228 | |     CharImplItem,                "char",               char_impl,               Target::Impl;
[00:08:21] 229 | |     StrImplItem,                 "str",                str_impl,                Target::Impl;
[00:08:21] ...   |
[00:08:21] 381 | |     Rc,                          "rc",                 rc,                      Target::Struct;
[00:08:21]     | |_- in this macro invocation
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/lang_items.rs:143:48
[00:08:21]    --> src/librustc/middle/lang_items.rs:143:48
[00:08:21]     |
[00:08:21] 143 |       fn visit_impl_item(&mut self, _impl_item: &hir::ImplItem) {
[00:08:21]     |                                                  ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] ...
[00:08:21] 226 | / language_item_table! {
[00:08:21] 227 | | //  Variant name,                Name,                 Method name,             Target;
[00:08:21] 228 | |     CharImplItem,                "char",               char_impl,               Target::Impl;
[00:08:21] 229 | |     StrImplItem,                 "str",                str_impl,                Target::Impl;
[00:08:21] ...   |
[00:08:21] 381 | |     Rc,                          "rc",                 rc,                      Target::Struct;
[00:08:21]     | |_- in this macro invocation
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:174:56
[00:08:21]    --> src/librustc/middle/liveness.rs:174:56
[00:08:21]     |
[00:08:21] 174 |     fn visit_fn(&mut self, fk: FnKind<'tcx>, fd: &'tcx hir::FnDecl,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:179:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 179 |     fn visit_local(&mut self, l: &'tcx hir::Local) { visit_local(self, l); }
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:180:40
[00:08:21]     |
[00:08:21]     |
[00:08:21] 180 |     fn visit_expr(&mut self, ex: &'tcx Expr) { visit_expr(self, ex); }
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:181:38
[00:08:21]     |
[00:08:21]     |
[00:08:21] 181 |     fn visit_arm(&mut self, a: &'tcx hir::Arm) { visit_arm(self, a); }
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:358:39
[00:08:21]     |
[00:08:21]     |
[00:08:21] 358 |                           decl: &'tcx hir::FnDecl,
[00:08:21]     |                                       ^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:401:62
[00:08:21]     |
[00:08:21] 401 | fn add_from_pat<'a, 'tcx>(ir: &mut IrMaps<'a, 'tcx>, pat: &P<hir::Pat>) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:401:60
[00:08:21]     |
[00:08:21]     |
[00:08:21] 401 | fn add_from_pat<'a, 'tcx>(ir: &mut IrMaps<'a, 'tcx>, pat: &P<hir::Pat>) {
[00:08:21]     |                                                            ^^^^^^^^^^^ help: indicate the anonymous lifetime: `P<'_, hir::Pat>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:447:66
[00:08:21]     |
[00:08:21]     |
[00:08:21] 447 | fn visit_local<'a, 'tcx>(ir: &mut IrMaps<'a, 'tcx>, local: &'tcx hir::Local) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:452:62
[00:08:21]     |
[00:08:21]     |
[00:08:21] 452 | fn visit_arm<'a, 'tcx>(ir: &mut IrMaps<'a, 'tcx>, arm: &'tcx hir::Arm) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:459:64
[00:08:21]     |
[00:08:21]     |
[00:08:21] 459 | fn visit_expr<'a, 'tcx>(ir: &mut IrMaps<'a, 'tcx>, expr: &'tcx Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:724:41
[00:08:21]     |
[00:08:21]     |
[00:08:21] 724 |     fn pat_bindings<F>(&mut self, pat: &hir::Pat, mut f: F) where
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:734:53
[00:08:21]     |
[00:08:21]     |
[00:08:21] 734 |     fn arm_pats_bindings<F>(&mut self, pat: Option<&hir::Pat>, f: F) where
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:742:48
[00:08:21]     |
[00:08:21]     |
[00:08:21] 742 |     fn define_bindings_in_pat(&mut self, pat: &hir::Pat, succ: LiveNode)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:747:60
[00:08:21]     |
[00:08:21]     |
[00:08:21] 747 |     fn define_bindings_in_arm_pats(&mut self, pat: Option<&hir::Pat>, succ: LiveNode)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:927:34
[00:08:21]     |
[00:08:21]     |
[00:08:21] 927 |     fn compute(&mut self, body: &hir::Expr) -> LiveNode {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:950:49
[00:08:21]     |
[00:08:21]     |
[00:08:21] 950 |     fn propagate_through_block(&mut self, blk: &hir::Block, succ: LiveNode)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:961:49
[00:08:21]     |
[00:08:21]     |
[00:08:21] 961 |     fn propagate_through_stmt(&mut self, stmt: &hir::Stmt, succ: LiveNode)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/liveness.rs:992:52
[00:08:21]     |
[00:08:21]     |
[00:08:21] 992 |     fn propagate_through_exprs(&mut self, exprs: &[Expr], succ: LiveNode)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1000:53
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1000 |                                   opt_expr: Option<&Expr>,
[00:08:21]      |                                                     ^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1006:49
[00:08:21]      |
[00:08:21] 1006 |     fn propagate_through_expr(&mut self, expr: &Expr, succ: LiveNode)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1256:50
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1256 |                                           expr: &Expr,
[00:08:21]      |                                                  ^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1316:38
[00:08:21]      |
[00:08:21] 1316 |     fn write_place(&mut self, expr: &Expr, succ: LiveNode, acc: u32) -> LiveNode {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1342:53
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1342 |     fn access_path(&mut self, hir_id: HirId, path: &hir::Path, succ: LiveNode, acc: u32)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1353:38
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1432:40
[00:08:21]      |
[00:08:21] 1432 |     fn visit_local(&mut self, l: &'tcx hir::Local) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1435:40
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1435 |     fn visit_expr(&mut self, ex: &'tcx Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1438:38
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1438 |     fn visit_arm(&mut self, a: &'tcx hir::Arm) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1443:70
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1443 | fn check_local<'a, 'tcx>(this: &mut Liveness<'a, 'tcx>, local: &'tcx hir::Local) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1459:66
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1459 | fn check_arm<'a, 'tcx>(this: &mut Liveness<'a, 'tcx>, arm: &'tcx hir::Arm) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1486:68
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1486 | fn check_expr<'a, 'tcx>(this: &mut Liveness<'a, 'tcx>, expr: &'tcx Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1535:43
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1535 |     fn check_place(&mut self, expr: &'tcx Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1566:45
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1566 |     fn warn_about_unused_args(&self, body: &hir::Body, entry_ln: LiveNode) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/liveness.rs:1583:63
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1583 |     fn warn_about_unused_or_dead_vars_in_pat(&mut self, pat: &hir::Pat) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/mem_categorization.rs:503:34
[00:08:21]     |
[00:08:21]     |
[00:08:21] 503 |     pub fn expr_ty(&self, expr: &hir::Expr) -> McResult<Ty<'tcx>> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/mem_categorization.rs:507:43
[00:08:21]     |
[00:08:21]     |
[00:08:21] 507 |     pub fn expr_ty_adjusted(&self, expr: &hir::Expr) -> McResult<Ty<'tcx>> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/mem_categorization.rs:521:41
[00:08:21]     |
[00:08:21]     |
[00:08:21] 521 |     pub fn pat_ty_adjusted(&self, pat: &hir::Pat) -> McResult<Ty<'tcx>> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/mem_categorization.rs:538:39
[00:08:21]     |
[00:08:21]     |
[00:08:21] 538 |     fn pat_ty_unadjusted(&self, pat: &hir::Pat) -> McResult<Ty<'tcx>> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/mem_categorization.rs:573:35
[00:08:21]     |
[00:08:21]     |
[00:08:21] 573 |     pub fn cat_expr(&self, expr: &hir::Expr) -> McResult<cmt_<'tcx>> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/mem_categorization.rs:577:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 577 |                                   expr: &hir::Expr,
[00:08:21]     |                                          ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/mem_categorization.rs:591:44
[00:08:21]     |
[00:08:21] 591 |     pub fn cat_expr_adjusted(&self, expr: &hir::Expr,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/mem_categorization.rs:598:48
[00:08:21]     |
[00:08:21]     |
[00:08:21] 598 |     fn cat_expr_adjusted_with<F>(&self, expr: &hir::Expr,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/mem_categorization.rs:634:46
[00:08:21]     |
[00:08:21]     |
[00:08:21] 634 |     pub fn cat_expr_unadjusted(&self, expr: &hir::Expr) -> McResult<cmt_<'tcx>> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/mem_categorization.rs:1015:16
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/mem_categorization.rs:1155:36
[00:08:21]      |
[00:08:21] 1155 |         where F: FnMut(cmt<'tcx>, &hir::Pat),
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/mem_categorization.rs:1154:56
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1154 |     pub fn cat_pattern<F>(&self, cmt: cmt<'tcx>, pat: &hir::Pat, mut op: F) -> McResult<()>
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/mem_categorization.rs:1162:37
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1162 |         where F : FnMut(cmt<'tcx>, &hir::Pat)
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/mem_categorization.rs:1161:57
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1161 |     fn cat_pattern_<F>(&self, mut cmt: cmt<'tcx>, pat: &hir::Pat, op: &mut F) -> McResult<()>
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/middle/reachable.rs:32:33
[00:08:21]    |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/middle/reachable.rs:95:42
[00:08:21]    |
[00:08:21] 95 |     fn visit_expr(&mut self, expr: &'tcx hir::Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/reachable.rs:341:37
[00:08:21]     |
[00:08:21]     |
[00:08:21] 341 |     fn visit_item(&mut self, item: &hir::Item) {
[00:08:21]     |                                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/reachable.rs:379:50
[00:08:21]     |
[00:08:21] 379 |     fn visit_trait_item(&mut self, _trait_item: &hir::TraitItem) {}
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/reachable.rs:381:48
[00:08:21]     |
[00:08:21]     |
[00:08:21] 381 |     fn visit_impl_item(&mut self, _impl_item: &hir::ImplItem) {
[00:08:21]     |                                                ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/region.rs:410:40
[00:08:21]     |
[00:08:21] 410 |     fn visit_pat(&mut self, pat: &'tcx Pat) {
[00:08:21]     |                                        ^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/region.rs:420:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 420 |     fn visit_expr(&mut self, expr: &'tcx Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/region.rs:711:48
[00:08:21]     |
[00:08:21]     |
[00:08:21] 711 |                                    body: &'tcx hir::Body) -> Option<Span> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/region.rs:750:88
[00:08:21]     |
[00:08:21]     |
[00:08:21] 750 | fn resolve_block<'a, 'tcx>(visitor: &mut RegionResolutionVisitor<'a, 'tcx>, blk: &'tcx hir::Block) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/region.rs:818:86
[00:08:21]     |
[00:08:21]     |
[00:08:21] 818 | fn resolve_arm<'a, 'tcx>(visitor: &mut RegionResolutionVisitor<'a, 'tcx>, arm: &'tcx hir::Arm) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/region.rs:828:86
[00:08:21]     |
[00:08:21]     |
[00:08:21] 828 | fn resolve_pat<'a, 'tcx>(visitor: &mut RegionResolutionVisitor<'a, 'tcx>, pat: &'tcx hir::Pat) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/region.rs:845:88
[00:08:21]     |
[00:08:21]     |
[00:08:21] 845 | fn resolve_stmt<'a, 'tcx>(visitor: &mut RegionResolutionVisitor<'a, 'tcx>, stmt: &'tcx hir::Stmt) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/region.rs:864:88
[00:08:21]     |
[00:08:21]     |
[00:08:21] 864 | fn resolve_expr<'a, 'tcx>(visitor: &mut RegionResolutionVisitor<'a, 'tcx>, expr: &'tcx hir::Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/region.rs:977:46
[00:08:21]     |
[00:08:21]     |
[00:08:21] 977 |                            pat: Option<&'tcx hir::Pat>,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/region.rs:978:47
[00:08:21]     |
[00:08:21]     |
[00:08:21] 978 |                            init: Option<&'tcx hir::Expr>) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/region.rs:1065:29
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1065 |     fn is_binding_pat(pat: &hir::Pat) -> bool {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/region.rs:1128:16
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/region.rs:1239:40
[00:08:21]      |
[00:08:21] 1239 |     fn visit_block(&mut self, b: &'tcx Block) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/region.rs:1243:42
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1243 |     fn visit_body(&mut self, body: &'tcx hir::Body) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/region.rs:1309:38
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1309 |     fn visit_arm(&mut self, a: &'tcx Arm) {
[00:08:21]      |                                      ^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/region.rs:1312:38
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1312 |     fn visit_pat(&mut self, p: &'tcx Pat) {
[00:08:21]      |                                      ^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/region.rs:1315:39
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1315 |     fn visit_stmt(&mut self, s: &'tcx Stmt) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/region.rs:1318:40
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1318 |     fn visit_expr(&mut self, ex: &'tcx Expr) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/region.rs:1321:40
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1321 |     fn visit_local(&mut self, l: &'tcx Local) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/middle/resolve_lifetime.rs:46:27
[00:08:21]    |
[00:08:21]    |
[00:08:21] 46 |     fn from_param(param: &GenericParam) -> Self {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/middle/resolve_lifetime.rs:84:58
[00:08:21]    |
[00:08:21]    |
[00:08:21] 84 |     fn early(hir_map: &Map<'_>, index: &mut u32, param: &GenericParam) -> (ParamName, Region) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]   --> src/librustc/middle/resolve_lifetime.rs:93:40
[00:08:21]    |
[00:08:21]    |
[00:08:21] 93 |     fn late(hir_map: &Map<'_>, param: &GenericParam) -> (ParamName, Region) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/resolve_lifetime.rs:442:37
[00:08:21]     |
[00:08:21]     |
[00:08:21] 442 | fn sub_items_have_self_param(node: &hir::ItemKind) -> bool {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1017:26
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1017 |         trait_ref: &'tcx hir::PolyTraitRef,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1096:30
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1096 | fn shadower_lifetime(param: &hir::GenericParam) -> Shadower {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1112:79
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1112 | fn check_mixed_explicit_and_in_band_defs(tcx: TyCtxt<'_, '_, '_>, params: &P<[hir::GenericParam]>) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1112:76
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1112 | fn check_mixed_explicit_and_in_band_defs(tcx: TyCtxt<'_, '_, '_>, params: &P<[hir::GenericParam]>) {
[00:08:21]      |                                                                            ^^^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `P<'_, [hir::GenericParam]>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1178:62
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1178 | fn extract_labels(ctxt: &mut LifetimeContext<'_, '_>, body: &hir::Body) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1197:39
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1197 |         fn visit_expr(&mut self, ex: &hir::Expr) {
[00:08:21]      |                                       ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1219:30
[00:08:21]      |
[00:08:21] 1219 |     fn expression_label(ex: &hir::Expr) -> Option<ast::Ident> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1327:16
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1327 |     generics: &hir::Generics,
[00:08:21]      |                ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1329:64
[00:08:21]      |
[00:08:21] 1329 |     fn add_bounds(set: &mut Set1<hir::LifetimeName>, bounds: &[hir::GenericBound]) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1445:67
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1445 |     fn lifetime_deletion_span(&self, name: ast::Ident, generics: &hir::Generics) -> Option<Span> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1490:59
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1490 |         let mut find_arg_use_span = |inputs: &hir::HirVec<hir::Ty>| {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1490:47
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1490 |         let mut find_arg_use_span = |inputs: &hir::HirVec<hir::Ty>| {
[00:08:21]      |                                               ^^^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `hir::HirVec<'_, hir::Ty>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1683:21
[00:08:21]      |
[00:08:21] 1683 |         decl: &'tcx hir::FnDecl,
[00:08:21] 1683 |         decl: &'tcx hir::FnDecl,
[00:08:21]      |                     ^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1684:25
[00:08:21]      |
[00:08:21] 1684 |         generics: &'tcx hir::Generics,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:1888:82
[00:08:21]      |
[00:08:21]      |
[00:08:21] 1888 |     fn visit_segment_args(&mut self, def: Def, depth: usize, generic_args: &'tcx hir::GenericArgs) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:2023:56
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2023 |     fn visit_fn_like_elision(&mut self, inputs: &'tcx [hir::Ty], output: Option<&'tcx P<hir::Ty>>) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:2023:89
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2023 |     fn visit_fn_like_elision(&mut self, inputs: &'tcx [hir::Ty], output: Option<&'tcx P<hir::Ty>>) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:2023:87
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2023 |     fn visit_fn_like_elision(&mut self, inputs: &'tcx [hir::Ty], output: Option<&'tcx P<hir::Ty>>) {
[00:08:21]      |                                                                                       ^^^^^^^^^^ help: indicate the anonymous lifetime: `P<'_, hir::Ty>`
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:2223:41
[00:08:21]      |
[00:08:21] 2223 |             fn visit_ty(&mut self, ty: &hir::Ty) {
[00:08:21] 2223 |             fn visit_ty(&mut self, ty: &hir::Ty) {
[00:08:21]      |                                         ^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:2245:55
[00:08:21]      |
[00:08:21] 2245 |             fn visit_generic_param(&mut self, param: &hir::GenericParam) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:2257:29
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2257 |                 trait_ref: &hir::PolyTraitRef,
[00:08:21]      |                             ^^^^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:2476:24
[00:08:21]      |
[00:08:21] 2476 |         params: &'tcx [hir::GenericParam],
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:2564:22
[00:08:21]      |
[00:08:21]      |
[00:08:21] 2564 |         param: &'tcx hir::GenericParam,
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:2720:12
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/middle/resolve_lifetime.rs:2811:40
[00:08:21]      |
[00:08:21] 2811 |         fn visit_ty(&mut self, ty: &'v hir::Ty) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:250:39
[00:08:21]     |
[00:08:21]     |
[00:08:21] 250 |     fn visit_item(&mut self, i: &'tcx Item) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:279:46
[00:08:21]     |
[00:08:21]     |
[00:08:21] 279 |     fn visit_trait_item(&mut self, ti: &'tcx hir::TraitItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:285:45
[00:08:21]     |
[00:08:21]     |
[00:08:21] 285 |     fn visit_impl_item(&mut self, ii: &'tcx hir::ImplItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:296:44
[00:08:21]     |
[00:08:21]     |
[00:08:21] 296 |     fn visit_variant(&mut self, var: &'tcx Variant, g: &'tcx Generics, item_id: NodeId) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:296:62
[00:08:21]     |
[00:08:21]     |
[00:08:21] 296 |     fn visit_variant(&mut self, var: &'tcx Variant, g: &'tcx Generics, item_id: NodeId) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:302:47
[00:08:21]     |
[00:08:21]     |
[00:08:21] 302 |     fn visit_struct_field(&mut self, s: &'tcx StructField) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:308:47
[00:08:21]     |
[00:08:21]     |
[00:08:21] 308 |     fn visit_foreign_item(&mut self, i: &'tcx hir::ForeignItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:314:45
[00:08:21]     |
[00:08:21]     |
[00:08:21] 314 |     fn visit_macro_def(&mut self, md: &'tcx hir::MacroDef) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:345:39
[00:08:21]     |
[00:08:21]     |
[00:08:21] 345 |     fn visit_item(&mut self, i: &'tcx Item) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:359:46
[00:08:21]     |
[00:08:21]     |
[00:08:21] 359 |     fn visit_trait_item(&mut self, ti: &'tcx hir::TraitItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:364:45
[00:08:21]     |
[00:08:21]     |
[00:08:21] 364 |     fn visit_impl_item(&mut self, ii: &'tcx hir::ImplItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:372:44
[00:08:21]     |
[00:08:21]     |
[00:08:21] 372 |     fn visit_variant(&mut self, var: &'tcx Variant, g: &'tcx Generics, item_id: NodeId) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:372:62
[00:08:21]     |
[00:08:21]     |
[00:08:21] 372 |     fn visit_variant(&mut self, var: &'tcx Variant, g: &'tcx Generics, item_id: NodeId) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:377:47
[00:08:21]     |
[00:08:21]     |
[00:08:21] 377 |     fn visit_struct_field(&mut self, s: &'tcx StructField) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:382:47
[00:08:21]     |
[00:08:21]     |
[00:08:21] 382 |     fn visit_foreign_item(&mut self, i: &'tcx hir::ForeignItem) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:387:45
[00:08:21]     |
[00:08:21]     |
[00:08:21] 387 |     fn visit_macro_def(&mut self, md: &'tcx hir::MacroDef) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:760:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 760 |     fn visit_item(&mut self, item: &'tcx hir::Item) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/stability.rs:819:42
[00:08:21]     |
[00:08:21]     |
[00:08:21] 819 |     fn visit_path(&mut self, path: &'tcx hir::Path, id: hir::HirId) {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/middle/weak_lang_items.rs:137:42
[00:08:21]     |
[00:08:21] 137 |       fn visit_foreign_item(&mut self, i: &hir::ForeignItem) {
[00:08:21] ...
[00:08:21] ...
[00:08:21] 156 | / weak_lang_items! {
[00:08:21] 157 | |     panic_impl,         PanicImplLangItem,          rust_begin_unwind;
[00:08:21] 158 | |     eh_personality,     EhPersonalityLangItem,      rust_eh_personality;
[00:08:21] 159 | |     eh_unwind_resume,   EhUnwindResumeLangItem,     rust_eh_unwind_resume;
[00:08:21] 160 | |     oom,                OomLangItem,                rust_oom;
[00:08:21]     | |_- in this macro invocation
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/ty/context.rs:459:37
[00:08:21]    --> src/librustc/ty/context.rs:459:37
[00:08:21]     |
[00:08:21] 459 |     pub fn qpath_def(&self, qpath: &hir::QPath, id: hir::HirId) -> Def {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/ty/context.rs:560:32
[00:08:21]     |
[00:08:21]     |
[00:08:21] 560 |     pub fn pat_ty(&self, pat: &hir::Pat) -> Ty<'tcx> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/ty/context.rs:564:36
[00:08:21]     |
[00:08:21]     |
[00:08:21] 564 |     pub fn pat_ty_opt(&self, pat: &hir::Pat) -> Option<Ty<'tcx>> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/ty/context.rs:578:34
[00:08:21]     |
[00:08:21]     |
[00:08:21] 578 |     pub fn expr_ty(&self, expr: &hir::Expr) -> Ty<'tcx> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/ty/context.rs:582:38
[00:08:21]     |
[00:08:21]     |
[00:08:21] 582 |     pub fn expr_ty_opt(&self, expr: &hir::Expr) -> Option<Ty<'tcx>> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/ty/context.rs:601:43
[00:08:21]     |
[00:08:21]     |
[00:08:21] 601 |     pub fn expr_adjustments(&self, expr: &hir::Expr)
[00:08:21]     |                                           ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/ty/context.rs:609:43
[00:08:21]     |
[00:08:21] 609 |     pub fn expr_ty_adjusted(&self, expr: &hir::Expr) -> Ty<'tcx> {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/ty/context.rs:615:47
[00:08:21]     |
---
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]    --> src/librustc/ty/mod.rs:265:34
[00:08:21]     |
[00:08:21] 265 |     pub fn from_hir(visibility: &hir::Visibility, id: NodeId, tcx: TyCtxt<'_, '_, '_>) -> Self {
[00:08:21] 
[00:08:21] error: hidden lifetime parameters in types are deprecated
[00:08:21]     --> src/librustc/ty/mod.rs:2739:57
[00:08:21]      |
---
[00:08:21] 
[00:08:21] error: unused import: `DerefMut`
[00:08:21]  --> src/librustc/hir/ptr.rs:3:23
[00:08:21]   |
[00:08:21] 3 | use std::ops::{Deref, DerefMut};
[00:08:21]   |
[00:08:21]   = note: `-D unused-imports` implied by `-D warnings`
[00:08:21] 
[00:08:21] error: unused import: `std::marker::PhantomData`
---
[00:08:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:49] expected success, got: exit code: 101
[00:08:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:49] Build completed unsuccessfully in 0:04:30
[00:08:49] Makefile:18: recipe for target 'all' failed
[00:08:49] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:299c6d90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 17 06:10:29 UTC 2019
---
travis_time:end:044e0c90:start=1550383830099646577,finish=1550383830105040814,duration=5394237
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:017528ac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04fdccc8
travis_time:start:04fdccc8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-l
