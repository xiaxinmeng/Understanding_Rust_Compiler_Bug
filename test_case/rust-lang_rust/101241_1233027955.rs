plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 12e4fd0755d7d976d4ee0f2004dc938290752ff7 and 1fe0cdf1bdc4970ac5503dc8a0c61e4c82c40e85
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stdout:

 if_chain! {
     if let ExprKind::Block(block, None) = expr.kind;
     if block.stmts.len() == 3;
     if let StmtKind::Local(local) = block.stmts[0].kind;
     if let Some(init) = local.init;
     if let ExprKind::Lit(ref lit) = init.kind;
     if let LitKind::Int(42, LitIntType::Signed(IntTy::I32)) = lit.node;
-    if let PatKind::Binding(BindingAnnotation::Unannotated, _, name, None) = local.pat.kind;
+    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name, None) = local.pat.kind;
     if name.as_str() == "x";
     if let StmtKind::Local(local1) = block.stmts[1].kind;
     if let Some(init1) = local1.init;
     if let ExprKind::Lit(ref lit1) = init1.kind;
     if let LitKind::Float(_, LitFloatType::Suffixed(FloatTy::F32)) = lit1.node;
-    if let PatKind::Binding(BindingAnnotation::Unannotated, _, name1, None) = local1.pat.kind;
+    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name1, None) = local1.pat.kind;
     if name1.as_str() == "_t";
     if let StmtKind::Semi(e) = block.stmts[2].kind;
     if let ExprKind::Unary(UnOp::Neg, inner) = e.kind;
     if let ExprKind::Path(ref qpath) = inner.kind;
     if match_qpath(qpath, &["x"]);
     if block.expr.is_none();
     then {
         // report your lint here
 }
 if_chain! {
 if_chain! {
     if let ExprKind::Block(block, None) = expr.kind;
     if block.stmts.len() == 1;
     if let StmtKind::Local(local) = block.stmts[0].kind;
     if let Some(init) = local.init;
     if let ExprKind::Call(func, args) = init.kind;
     if let ExprKind::Path(ref qpath) = func.kind;
     if match_qpath(qpath, &["String", "new"]);
     if args.is_empty();
-    if let PatKind::Binding(BindingAnnotation::Unannotated, _, name, None) = local.pat.kind;
+    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name, None) = local.pat.kind;
     if name.as_str() == "expr";
     if let Some(trailing_expr) = block.expr;
     if let ExprKind::Call(func1, args1) = trailing_expr.kind;
     if let ExprKind::Path(ref qpath1) = func1.kind;
     if match_qpath(qpath1, &["drop"]);
     if args1.len() == 1;
     if let ExprKind::Path(ref qpath2) = args1[0].kind;
     if match_qpath(qpath2, &["expr"]);
     then {
         // report your lint here
 }
 if_chain! {
 if_chain! {
     if let ExprKind::Closure(CaptureBy::Value, fn_decl, body_id, _, None) = expr.kind;
     if let FnRetTy::DefaultReturn(_) = fn_decl.output;
     let expr1 = &cx.tcx.hir().body(body_id).value;
     if let ExprKind::Call(func, args) = expr1.kind;
     if let ExprKind::Path(ref qpath) = func.kind;
     if matches!(qpath, QPath::LangItem(LangItem::FromGenerator, _));
     if args.len() == 1;
     if let ExprKind::Closure(CaptureBy::Value, fn_decl1, body_id1, _, Some(Movability::Static)) = args[0].kind;
     if let FnRetTy::DefaultReturn(_) = fn_decl1.output;
     let expr2 = &cx.tcx.hir().body(body_id1).value;
     if let ExprKind::Block(block, None) = expr2.kind;
     if block.stmts.is_empty();
     if block.expr.is_none();
     then {
         // report your lint here
 }
 

The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/blocks.stage-id.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author/blocks.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author/blocks.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/blocks.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-56bce9bcc023120a.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c6aa3eacac0eeebb.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4b46e2e2788394f2.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-68b3adac889f3bfe.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-eed8221ad604f845.so" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-14dbc812a1f5dba0.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e993ea424d40e3e9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/blocks.stage-id.aux"
------------------------------------------
if_chain! {
if_chain! {
    if let ExprKind::Block(block, None) = expr.kind;
    if block.stmts.len() == 3;
    if let StmtKind::Local(local) = block.stmts[0].kind;
    if let Some(init) = local.init;
    if let ExprKind::Lit(ref lit) = init.kind;
    if let LitKind::Int(42, LitIntType::Signed(IntTy::I32)) = lit.node;
    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name, None) = local.pat.kind;
    if name.as_str() == "x";
    if let StmtKind::Local(local1) = block.stmts[1].kind;
    if let Some(init1) = local1.init;
    if let ExprKind::Lit(ref lit1) = init1.kind;
    if let LitKind::Float(_, LitFloatType::Suffixed(FloatTy::F32)) = lit1.node;
    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name1, None) = local1.pat.kind;
    if name1.as_str() == "_t";
    if let StmtKind::Semi(e) = block.stmts[2].kind;
    if let ExprKind::Unary(UnOp::Neg, inner) = e.kind;
    if let ExprKind::Path(ref qpath) = inner.kind;
    if match_qpath(qpath, &["x"]);
    if block.expr.is_none();
    then {
        // report your lint here
}
if_chain! {
if_chain! {
    if let ExprKind::Block(block, None) = expr.kind;
    if block.stmts.len() == 1;
    if let StmtKind::Local(local) = block.stmts[0].kind;
    if let Some(init) = local.init;
    if let ExprKind::Call(func, args) = init.kind;
    if let ExprKind::Path(ref qpath) = func.kind;
    if match_qpath(qpath, &["String", "new"]);
    if args.is_empty();
    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name, None) = local.pat.kind;
    if name.as_str() == "expr";
    if let Some(trailing_expr) = block.expr;
    if let ExprKind::Call(func1, args1) = trailing_expr.kind;
    if let ExprKind::Path(ref qpath1) = func1.kind;
    if match_qpath(qpath1, &["drop"]);
    if args1.len() == 1;
    if let ExprKind::Path(ref qpath2) = args1[0].kind;
    if match_qpath(qpath2, &["expr"]);
    then {
        // report your lint here
}
if_chain! {
if_chain! {
    if let ExprKind::Closure(CaptureBy::Value, fn_decl, body_id, _, None) = expr.kind;
    if let FnRetTy::DefaultReturn(_) = fn_decl.output;
    let expr1 = &cx.tcx.hir().body(body_id).value;
    if let ExprKind::Call(func, args) = expr1.kind;
    if let ExprKind::Path(ref qpath) = func.kind;
error: test failed, to rerun pass '--test compile-test'
    if matches!(qpath, QPath::LangItem(LangItem::FromGenerator, _));
    if args.len() == 1;
    if let ExprKind::Closure(CaptureBy::Value, fn_decl1, body_id1, _, Some(Movability::Static)) = args[0].kind;
    if let FnRetTy::DefaultReturn(_) = fn_decl1.output;
    let expr2 = &cx.tcx.hir().body(body_id1).value;
    if let ExprKind::Block(block, None) = expr2.kind;
    if block.stmts.is_empty();
    if block.expr.is_none();
    then {
        // report your lint here
}

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stdout:

 if_chain! {
     if let StmtKind::Local(local) = stmt.kind;
     if let Some(init) = local.init;
     if let ExprKind::Cast(expr, cast_ty) = init.kind;
     if let TyKind::Path(ref qpath) = cast_ty.kind;
     if match_qpath(qpath, &["char"]);
     if let ExprKind::Lit(ref lit) = expr.kind;
     if let LitKind::Int(69, LitIntType::Unsuffixed) = lit.node;
-    if let PatKind::Binding(BindingAnnotation::Unannotated, _, name, None) = local.pat.kind;
+    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name, None) = local.pat.kind;
     if name.as_str() == "x";
     then {
         // report your lint here
 }
 

The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author.stage-id.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-56bce9bcc023120a.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c6aa3eacac0eeebb.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4b46e2e2788394f2.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-68b3adac889f3bfe.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-eed8221ad604f845.so" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-14dbc812a1f5dba0.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e993ea424d40e3e9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author.stage-id.aux"
------------------------------------------
if_chain! {
if_chain! {
    if let StmtKind::Local(local) = stmt.kind;
    if let Some(init) = local.init;
    if let ExprKind::Cast(expr, cast_ty) = init.kind;
    if let TyKind::Path(ref qpath) = cast_ty.kind;
    if match_qpath(qpath, &["char"]);
    if let ExprKind::Lit(ref lit) = expr.kind;
    if let LitKind::Int(69, LitIntType::Unsuffixed) = lit.node;
    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name, None) = local.pat.kind;
    if name.as_str() == "x";
    then {
        // report your lint here
}

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stdout:

 if_chain! {
     if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr);
-    if let PatKind::Binding(BindingAnnotation::Unannotated, _, name, None) = pat.kind;
+    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name, None) = pat.kind;
     if name.as_str() == "y";
     if let ExprKind::Struct(qpath, fields, None) = arg.kind;
     if matches!(qpath, QPath::LangItem(LangItem::Range, _));
     if fields.len() == 2;
     if fields[0].ident.as_str() == "start";
     if let ExprKind::Lit(ref lit) = fields[0].expr.kind;
     if let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node;
     if fields[1].ident.as_str() == "end";
     if let ExprKind::Lit(ref lit1) = fields[1].expr.kind;
     if let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node;
     if let ExprKind::Block(block, None) = body.kind;
     if block.stmts.len() == 1;
     if let StmtKind::Local(local) = block.stmts[0].kind;
     if let Some(init) = local.init;
     if let ExprKind::Path(ref qpath1) = init.kind;
     if match_qpath(qpath1, &["y"]);
-    if let PatKind::Binding(BindingAnnotation::Unannotated, _, name1, None) = local.pat.kind;
+    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name1, None) = local.pat.kind;
     if name1.as_str() == "z";
     if block.expr.is_none();
     then {
         // report your lint here
 }
 if_chain! {
 if_chain! {
     if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr);
     if let PatKind::Wild = pat.kind;
     if let ExprKind::Struct(qpath, fields, None) = arg.kind;
     if matches!(qpath, QPath::LangItem(LangItem::Range, _));
     if fields.len() == 2;
     if fields[0].ident.as_str() == "start";
     if let ExprKind::Lit(ref lit) = fields[0].expr.kind;
     if let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node;
     if fields[1].ident.as_str() == "end";
     if let ExprKind::Lit(ref lit1) = fields[1].expr.kind;
     if let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node;
     if let ExprKind::Block(block, None) = body.kind;
     if block.stmts.len() == 1;
     if let StmtKind::Semi(e) = block.stmts[0].kind;
     if let ExprKind::Break(destination, None) = e.kind;
     if destination.label.is_none();
     if block.expr.is_none();
     then {
         // report your lint here
 }
 if_chain! {
 if_chain! {
     if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr);
     if let PatKind::Wild = pat.kind;
     if let ExprKind::Struct(qpath, fields, None) = arg.kind;
     if matches!(qpath, QPath::LangItem(LangItem::Range, _));
     if fields.len() == 2;
     if fields[0].ident.as_str() == "start";
     if let ExprKind::Lit(ref lit) = fields[0].expr.kind;
     if let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node;
     if fields[1].ident.as_str() == "end";
     if let ExprKind::Lit(ref lit1) = fields[1].expr.kind;
     if let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node;
     if let ExprKind::Block(block, None) = body.kind;
     if block.stmts.len() == 1;
     if let StmtKind::Semi(e) = block.stmts[0].kind;
     if let ExprKind::Break(destination, None) = e.kind;
     if let Some(label) = destination.label;
     if label.ident.as_str() == "'label";
     if block.expr.is_none();
     then {
         // report your lint here
 }
 if_chain! {
 if_chain! {
     if let Some(higher::While { condition: condition, body: body }) = higher::While::hir(expr);
     if let ExprKind::Path(ref qpath) = condition.kind;
     if match_qpath(qpath, &["a"]);
     if let ExprKind::Block(block, None) = body.kind;
     if block.stmts.len() == 1;
     if let StmtKind::Semi(e) = block.stmts[0].kind;
     if let ExprKind::Break(destination, None) = e.kind;
     if destination.label.is_none();
     if block.expr.is_none();
     then {
         // report your lint here
 }
 if_chain! {
 if_chain! {
     if let Some(higher::WhileLet { let_pat: let_pat, let_expr: let_expr, if_then: if_then }) = higher::WhileLet::hir(expr);
     if let PatKind::Lit(lit_expr) = let_pat.kind;
     if let ExprKind::Lit(ref lit) = lit_expr.kind;
     if let LitKind::Bool(true) = lit.node;
     if let ExprKind::Path(ref qpath) = let_expr.kind;
     if match_qpath(qpath, &["a"]);
     if let ExprKind::Block(block, None) = if_then.kind;
     if block.stmts.len() == 1;
     if let StmtKind::Semi(e) = block.stmts[0].kind;
     if let ExprKind::Break(destination, None) = e.kind;
     if destination.label.is_none();
     if block.expr.is_none();
     then {
         // report your lint here
 }
 if_chain! {
 if_chain! {
     if let ExprKind::Loop(body, None, LoopSource::Loop, _) = expr.kind;
     if body.stmts.len() == 1;
     if let StmtKind::Semi(e) = body.stmts[0].kind;
     if let ExprKind::Break(destination, None) = e.kind;
     if destination.label.is_none();
     if body.expr.is_none();
     then {
         // report your lint here
 }
 

The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/loop.stage-id.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author/loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author/loop.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/loop.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-56bce9bcc023120a.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c6aa3eacac0eeebb.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4b46e2e2788394f2.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-68b3adac889f3bfe.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-eed8221ad604f845.so" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-14dbc812a1f5dba0.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e993ea424d40e3e9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/loop.stage-id.aux"
------------------------------------------
if_chain! {
if_chain! {
    if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr);
    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name, None) = pat.kind;
    if name.as_str() == "y";
    if let ExprKind::Struct(qpath, fields, None) = arg.kind;
    if matches!(qpath, QPath::LangItem(LangItem::Range, _));
    if fields.len() == 2;
    if fields[0].ident.as_str() == "start";
    if let ExprKind::Lit(ref lit) = fields[0].expr.kind;
    if let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node;
    if fields[1].ident.as_str() == "end";
    if let ExprKind::Lit(ref lit1) = fields[1].expr.kind;
    if let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node;
    if let ExprKind::Block(block, None) = body.kind;
    if block.stmts.len() == 1;
    if let StmtKind::Local(local) = block.stmts[0].kind;
    if let Some(init) = local.init;
    if let ExprKind::Path(ref qpath1) = init.kind;
    if match_qpath(qpath1, &["y"]);
    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name1, None) = local.pat.kind;
    if name1.as_str() == "z";
    if block.expr.is_none();
    then {
        // report your lint here
}
if_chain! {
if_chain! {
    if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr);
    if let PatKind::Wild = pat.kind;
    if let ExprKind::Struct(qpath, fields, None) = arg.kind;
    if matches!(qpath, QPath::LangItem(LangItem::Range, _));
    if fields.len() == 2;
    if fields[0].ident.as_str() == "start";
    if let ExprKind::Lit(ref lit) = fields[0].expr.kind;
    if let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node;
    if fields[1].ident.as_str() == "end";
    if let ExprKind::Lit(ref lit1) = fields[1].expr.kind;
    if let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node;
    if let ExprKind::Block(block, None) = body.kind;
    if block.stmts.len() == 1;
    if let StmtKind::Semi(e) = block.stmts[0].kind;
    if let ExprKind::Break(destination, None) = e.kind;
    if destination.label.is_none();
    if block.expr.is_none();
    then {
        // report your lint here
}
if_chain! {
if_chain! {
    if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr);
    if let PatKind::Wild = pat.kind;
    if let ExprKind::Struct(qpath, fields, None) = arg.kind;
    if matches!(qpath, QPath::LangItem(LangItem::Range, _));
    if fields.len() == 2;
    if fields[0].ident.as_str() == "start";
    if let ExprKind::Lit(ref lit) = fields[0].expr.kind;
    if let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node;
    if fields[1].ident.as_str() == "end";
    if let ExprKind::Lit(ref lit1) = fields[1].expr.kind;
    if let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node;
    if let ExprKind::Block(block, None) = body.kind;
    if block.stmts.len() == 1;
    if let StmtKind::Semi(e) = block.stmts[0].kind;
    if let ExprKind::Break(destination, None) = e.kind;
    if let Some(label) = destination.label;
    if label.ident.as_str() == "'label";
    if block.expr.is_none();
    then {
        // report your lint here
}
if_chain! {
if_chain! {
    if let Some(higher::While { condition: condition, body: body }) = higher::While::hir(expr);
    if let ExprKind::Path(ref qpath) = condition.kind;
    if match_qpath(qpath, &["a"]);
    if let ExprKind::Block(block, None) = body.kind;
    if block.stmts.len() == 1;
    if let StmtKind::Semi(e) = block.stmts[0].kind;
    if let ExprKind::Break(destination, None) = e.kind;
    if destination.label.is_none();
    if block.expr.is_none();
    then {
        // report your lint here
}
if_chain! {
if_chain! {
    if let Some(higher::WhileLet { let_pat: let_pat, let_expr: let_expr, if_then: if_then }) = higher::WhileLet::hir(expr);
    if let PatKind::Lit(lit_expr) = let_pat.kind;
    if let ExprKind::Lit(ref lit) = lit_expr.kind;
    if let LitKind::Bool(true) = lit.node;
    if let ExprKind::Path(ref qpath) = let_expr.kind;
    if match_qpath(qpath, &["a"]);
    if let ExprKind::Block(block, None) = if_then.kind;
    if block.stmts.len() == 1;
    if let StmtKind::Semi(e) = block.stmts[0].kind;
    if let ExprKind::Break(destination, None) = e.kind;
    if destination.label.is_none();
    if block.expr.is_none();
    then {
        // report your lint here
}
if_chain! {
if_chain! {
    if let ExprKind::Loop(body, None, LoopSource::Loop, _) = expr.kind;
    if body.stmts.len() == 1;
    if let StmtKind::Semi(e) = body.stmts[0].kind;
    if let ExprKind::Break(destination, None) = e.kind;
    if destination.label.is_none();
    if body.expr.is_none();
    then {
        // report your lint here
}

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stdout:

 if_chain! {
     if let StmtKind::Local(local) = stmt.kind;
     if let Some(init) = local.init;
     if let ExprKind::Match(scrutinee, arms, MatchSource::Normal) = init.kind;
     if let ExprKind::Lit(ref lit) = scrutinee.kind;
     if let LitKind::Int(42, LitIntType::Unsuffixed) = lit.node;
     if arms.len() == 3;
     if let PatKind::Lit(lit_expr) = arms[0].pat.kind;
     if let ExprKind::Lit(ref lit1) = lit_expr.kind;
     if let LitKind::Int(16, LitIntType::Unsuffixed) = lit1.node;
     if arms[0].guard.is_none();
     if let ExprKind::Lit(ref lit2) = arms[0].body.kind;
     if let LitKind::Int(5, LitIntType::Unsuffixed) = lit2.node;
     if let PatKind::Lit(lit_expr1) = arms[1].pat.kind;
     if let ExprKind::Lit(ref lit3) = lit_expr1.kind;
     if let LitKind::Int(17, LitIntType::Unsuffixed) = lit3.node;
     if arms[1].guard.is_none();
     if let ExprKind::Block(block, None) = arms[1].body.kind;
     if block.stmts.len() == 1;
     if let StmtKind::Local(local1) = block.stmts[0].kind;
     if let Some(init1) = local1.init;
     if let ExprKind::Lit(ref lit4) = init1.kind;
     if let LitKind::Int(3, LitIntType::Unsuffixed) = lit4.node;
-    if let PatKind::Binding(BindingAnnotation::Unannotated, _, name, None) = local1.pat.kind;
+    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name, None) = local1.pat.kind;
     if name.as_str() == "x";
     if let Some(trailing_expr) = block.expr;
     if let ExprKind::Path(ref qpath) = trailing_expr.kind;
     if match_qpath(qpath, &["x"]);
     if let PatKind::Wild = arms[2].pat.kind;
     if arms[2].guard.is_none();
     if let ExprKind::Lit(ref lit5) = arms[2].body.kind;
     if let LitKind::Int(1, LitIntType::Unsuffixed) = lit5.node;
-    if let PatKind::Binding(BindingAnnotation::Unannotated, _, name1, None) = local.pat.kind;
+    if let PatKind::Binding(BindingAnnotation::BindingAnnotation(No, Not), _, name1, None) = local.pat.kind;
     if name1.as_str() == "a";
     then {
         // report your lint here
 }
 

The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/matches.stage-id.stdout
