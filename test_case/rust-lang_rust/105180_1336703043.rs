plain

---- compile_test stdout ----
diff of stdout:

 if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr)
     && let PatKind::Binding(BindingAnnotation::NONE, _, name, None) = pat.kind
     && name.as_str() == "y"
     && let ExprKind::Struct(qpath, fields, None) = arg.kind
     && matches!(qpath, QPath::LangItem(LangItem::Range, _))
     && fields.len() == 2
     && fields[0].ident.as_str() == "start"
     && let ExprKind::Lit(ref lit) = fields[0].expr.kind
     && let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node
     && fields[1].ident.as_str() == "end"
     && let ExprKind::Lit(ref lit1) = fields[1].expr.kind
     && let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node
     && let ExprKind::Block(block, None) = body.kind
     && block.stmts.len() == 1
     && let StmtKind::Local(local) = block.stmts[0].kind
     && let Some(init) = local.init
error: test failed, to rerun pass `--test compile-test`
     && let ExprKind::Path(ref qpath1) = init.kind
     && match_qpath(qpath1, &["y"])
     && let PatKind::Binding(BindingAnnotation::NONE, _, name1, None) = local.pat.kind
     && name1.as_str() == "z"
     && block.expr.is_none()
     // report your lint here
 }
 }
+if let ExprKind::Loop(body, None, LoopSource::ForLoop, _) = expr.kind
+    && body.stmts.len() == 1
+    && let StmtKind::Expr(e) = body.stmts[0].kind
+    && let ExprKind::Match(scrutinee, arms, MatchSource::ForLoopDesugar) = e.kind
+    && let ExprKind::Call(func, args) = scrutinee.kind
+    && let ExprKind::Path(ref qpath) = func.kind
+    && matches!(qpath, QPath::LangItem(LangItem::IteratorNext, _))
+    && args.len() == 1
+    && let ExprKind::AddrOf(BorrowKind::Ref, Mutability::Mut, inner) = args[0].kind
+    && let ExprKind::Path(ref qpath1) = inner.kind
+    && match_qpath(qpath1, &["iter"])
+    && arms.len() == 2
+    && let PatKind::Struct(ref qpath2, fields, false) = arms[0].pat.kind
+    && matches!(qpath2, QPath::LangItem(LangItem::OptionNone, _))
+    && fields.is_empty()
+    && arms[0].guard.is_none()
+    && let ExprKind::Break(destination, None) = arms[0].body.kind
+    && destination.label.is_none()
+    && let PatKind::Struct(ref qpath3, fields1, false) = arms[1].pat.kind
+    && matches!(qpath3, QPath::LangItem(LangItem::OptionSome, _))
+    && fields1.len() == 1
+    && fields1[0].ident.as_str() == "0"
+    && let PatKind::Binding(BindingAnnotation::NONE, _, name, None) = fields1[0].pat.kind
+    && name.as_str() == "y"
+    && arms[1].guard.is_none()
+    && let ExprKind::Block(block, None) = arms[1].body.kind
+    && block.stmts.len() == 1
+    && let StmtKind::Local(local) = block.stmts[0].kind
+    && let Some(init) = local.init
+    && let ExprKind::Path(ref qpath4) = init.kind
+    && match_qpath(qpath4, &["y"])
+    && let PatKind::Binding(BindingAnnotation::NONE, _, name1, None) = local.pat.kind
+    && name1.as_str() == "z"
+    && block.expr.is_none()
+    && body.expr.is_none()
+    // report your lint here
+}
+}
 if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr)
     && let PatKind::Wild = pat.kind
     && let ExprKind::Struct(qpath, fields, None) = arg.kind
     && matches!(qpath, QPath::LangItem(LangItem::Range, _))
     && fields.len() == 2
     && fields[0].ident.as_str() == "start"
     && let ExprKind::Lit(ref lit) = fields[0].expr.kind
     && let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node
     && fields[1].ident.as_str() == "end"
     && let ExprKind::Lit(ref lit1) = fields[1].expr.kind
     && let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node
     && let ExprKind::Block(block, None) = body.kind
     && block.stmts.len() == 1
     && let StmtKind::Semi(e) = block.stmts[0].kind
     && let ExprKind::Break(destination, None) = e.kind
     && destination.label.is_none()
     && block.expr.is_none()
     // report your lint here
 }
 }
+if let ExprKind::Loop(body, None, LoopSource::ForLoop, _) = expr.kind
+    && body.stmts.len() == 1
+    && let StmtKind::Expr(e) = body.stmts[0].kind
+    && let ExprKind::Match(scrutinee, arms, MatchSource::ForLoopDesugar) = e.kind
+    && let ExprKind::Call(func, args) = scrutinee.kind
+    && let ExprKind::Path(ref qpath) = func.kind
+    && matches!(qpath, QPath::LangItem(LangItem::IteratorNext, _))
+    && args.len() == 1
+    && let ExprKind::AddrOf(BorrowKind::Ref, Mutability::Mut, inner) = args[0].kind
+    && let ExprKind::Path(ref qpath1) = inner.kind
+    && match_qpath(qpath1, &["iter"])
+    && arms.len() == 2
+    && let PatKind::Struct(ref qpath2, fields, false) = arms[0].pat.kind
+    && matches!(qpath2, QPath::LangItem(LangItem::OptionNone, _))
+    && fields.is_empty()
+    && arms[0].guard.is_none()
+    && let ExprKind::Break(destination, None) = arms[0].body.kind
+    && destination.label.is_none()
+    && let PatKind::Struct(ref qpath3, fields1, false) = arms[1].pat.kind
+    && matches!(qpath3, QPath::LangItem(LangItem::OptionSome, _))
+    && fields1.len() == 1
+    && fields1[0].ident.as_str() == "0"
+    && let PatKind::Wild = fields1[0].pat.kind
+    && arms[1].guard.is_none()
+    && let ExprKind::Block(block, None) = arms[1].body.kind
+    && block.stmts.len() == 1
+    && let StmtKind::Semi(e1) = block.stmts[0].kind
+    && let ExprKind::Break(destination1, None) = e1.kind
+    && destination1.label.is_none()
+    && block.expr.is_none()
+    && body.expr.is_none()
+    // report your lint here
+}
+}
 if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr)
     && let PatKind::Wild = pat.kind
     && let ExprKind::Struct(qpath, fields, None) = arg.kind
     && matches!(qpath, QPath::LangItem(LangItem::Range, _))
     && fields.len() == 2
     && fields[0].ident.as_str() == "start"
     && let ExprKind::Lit(ref lit) = fields[0].expr.kind
     && let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node
     && fields[1].ident.as_str() == "end"
     && let ExprKind::Lit(ref lit1) = fields[1].expr.kind
     && let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node
     && let ExprKind::Block(block, None) = body.kind
     && block.stmts.len() == 1
     && let StmtKind::Semi(e) = block.stmts[0].kind
     && let ExprKind::Break(destination, None) = e.kind
     && let Some(label) = destination.label
     && label.ident.as_str() == "'label"
     && block.expr.is_none()
+    // report your lint here
+}
+}
+if let ExprKind::Loop(body, Some(label), LoopSource::ForLoop, _) = expr.kind
+    && body.stmts.len() == 1
+    && let StmtKind::Expr(e) = body.stmts[0].kind
+    && let ExprKind::Match(scrutinee, arms, MatchSource::ForLoopDesugar) = e.kind
+    && let ExprKind::Call(func, args) = scrutinee.kind
+    && let ExprKind::Path(ref qpath) = func.kind
+    && matches!(qpath, QPath::LangItem(LangItem::IteratorNext, _))
+    && args.len() == 1
+    && let ExprKind::AddrOf(BorrowKind::Ref, Mutability::Mut, inner) = args[0].kind
+    && let ExprKind::Path(ref qpath1) = inner.kind
+    && match_qpath(qpath1, &["iter"])
+    && arms.len() == 2
+    && let PatKind::Struct(ref qpath2, fields, false) = arms[0].pat.kind
+    && matches!(qpath2, QPath::LangItem(LangItem::OptionNone, _))
+    && fields.is_empty()
+    && arms[0].guard.is_none()
+    && let ExprKind::Break(destination, None) = arms[0].body.kind
+    && destination.label.is_none()
+    && let PatKind::Struct(ref qpath3, fields1, false) = arms[1].pat.kind
+    && matches!(qpath3, QPath::LangItem(LangItem::OptionSome, _))
+    && fields1.len() == 1
+    && fields1[0].ident.as_str() == "0"
+    && let PatKind::Wild = fields1[0].pat.kind
+    && arms[1].guard.is_none()
+    && let ExprKind::Block(block, None) = arms[1].body.kind
+    && block.stmts.len() == 1
+    && let StmtKind::Semi(e1) = block.stmts[0].kind
+    && let ExprKind::Break(destination1, None) = e1.kind
+    && let Some(label1) = destination1.label
+    && label1.ident.as_str() == "'label"
+    && block.expr.is_none()
+    && body.expr.is_none()
+    && label.ident.as_str() == "'label"
     // report your lint here
 }
 }
 if let Some(higher::While { condition: condition, body: body }) = higher::While::hir(expr)
     && let ExprKind::Path(ref qpath) = condition.kind
     && match_qpath(qpath, &["a"])
     && let ExprKind::Block(block, None) = body.kind
     && block.stmts.len() == 1
     && let StmtKind::Semi(e) = block.stmts[0].kind
     && let ExprKind::Break(destination, None) = e.kind
     && destination.label.is_none()
     && block.expr.is_none()
     // report your lint here
 }
 }
 if let Some(higher::WhileLet { let_pat: let_pat, let_expr: let_expr, if_then: if_then }) = higher::WhileLet::hir(expr)
     && let PatKind::Lit(lit_expr) = let_pat.kind
     && let ExprKind::Lit(ref lit) = lit_expr.kind
     && let LitKind::Bool(true) = lit.node
     && let ExprKind::Path(ref qpath) = let_expr.kind
     && match_qpath(qpath, &["a"])
     && let ExprKind::Block(block, None) = if_then.kind
     && block.stmts.len() == 1
     && let StmtKind::Semi(e) = block.stmts[0].kind
     && let ExprKind::Break(destination, None) = e.kind
     && destination.label.is_none()
     && block.expr.is_none()
     // report your lint here
 }
 }
 if let ExprKind::Loop(body, None, LoopSource::Loop, _) = expr.kind
     && body.stmts.len() == 1
     && let StmtKind::Semi(e) = body.stmts[0].kind
     && let ExprKind::Break(destination, None) = e.kind
     && destination.label.is_none()
     && body.expr.is_none()
     // report your lint here
 }
 


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/loop.stage-id.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author/loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author/loop.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/loop.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-992c1a552e84eb88.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-f2e351c56701685e.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ec78d46878d61bf1.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/loop.stage-id.aux"
------------------------------------------
------------------------------------------
if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr)
    && let PatKind::Binding(BindingAnnotation::NONE, _, name, None) = pat.kind
    && name.as_str() == "y"
    && let ExprKind::Struct(qpath, fields, None) = arg.kind
    && matches!(qpath, QPath::LangItem(LangItem::Range, _))
    && fields.len() == 2
    && fields[0].ident.as_str() == "start"
    && let ExprKind::Lit(ref lit) = fields[0].expr.kind
    && let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node
    && fields[1].ident.as_str() == "end"
    && let ExprKind::Lit(ref lit1) = fields[1].expr.kind
    && let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node
    && let ExprKind::Block(block, None) = body.kind
    && block.stmts.len() == 1
    && let StmtKind::Local(local) = block.stmts[0].kind
    && let Some(init) = local.init
    && let ExprKind::Path(ref qpath1) = init.kind
    && match_qpath(qpath1, &["y"])
    && let PatKind::Binding(BindingAnnotation::NONE, _, name1, None) = local.pat.kind
    && name1.as_str() == "z"
    && block.expr.is_none()
    // report your lint here
}
}
if let ExprKind::Loop(body, None, LoopSource::ForLoop, _) = expr.kind
    && body.stmts.len() == 1
    && let StmtKind::Expr(e) = body.stmts[0].kind
    && let ExprKind::Match(scrutinee, arms, MatchSource::ForLoopDesugar) = e.kind
    && let ExprKind::Call(func, args) = scrutinee.kind
    && let ExprKind::Path(ref qpath) = func.kind
    && matches!(qpath, QPath::LangItem(LangItem::IteratorNext, _))
    && args.len() == 1
    && let ExprKind::AddrOf(BorrowKind::Ref, Mutability::Mut, inner) = args[0].kind
    && let ExprKind::Path(ref qpath1) = inner.kind
    && match_qpath(qpath1, &["iter"])
    && arms.len() == 2
    && let PatKind::Struct(ref qpath2, fields, false) = arms[0].pat.kind
    && matches!(qpath2, QPath::LangItem(LangItem::OptionNone, _))
    && fields.is_empty()
    && arms[0].guard.is_none()
    && let ExprKind::Break(destination, None) = arms[0].body.kind
    && destination.label.is_none()
    && let PatKind::Struct(ref qpath3, fields1, false) = arms[1].pat.kind
    && matches!(qpath3, QPath::LangItem(LangItem::OptionSome, _))
    && fields1.len() == 1
    && fields1[0].ident.as_str() == "0"
    && let PatKind::Binding(BindingAnnotation::NONE, _, name, None) = fields1[0].pat.kind
    && name.as_str() == "y"
    && arms[1].guard.is_none()
    && let ExprKind::Block(block, None) = arms[1].body.kind
    && block.stmts.len() == 1
    && let StmtKind::Local(local) = block.stmts[0].kind
    && let Some(init) = local.init
    && let ExprKind::Path(ref qpath4) = init.kind
    && match_qpath(qpath4, &["y"])
    && let PatKind::Binding(BindingAnnotation::NONE, _, name1, None) = local.pat.kind
    && name1.as_str() == "z"
    && block.expr.is_none()
    && body.expr.is_none()
    // report your lint here
}
}
if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr)
    && let PatKind::Wild = pat.kind
    && let ExprKind::Struct(qpath, fields, None) = arg.kind
    && matches!(qpath, QPath::LangItem(LangItem::Range, _))
    && fields.len() == 2
    && fields[0].ident.as_str() == "start"
    && let ExprKind::Lit(ref lit) = fields[0].expr.kind
    && let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node
    && fields[1].ident.as_str() == "end"
    && let ExprKind::Lit(ref lit1) = fields[1].expr.kind
    && let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node
    && let ExprKind::Block(block, None) = body.kind
    && block.stmts.len() == 1
    && let StmtKind::Semi(e) = block.stmts[0].kind
    && let ExprKind::Break(destination, None) = e.kind
    && destination.label.is_none()
    && block.expr.is_none()
    // report your lint here
}
}
if let ExprKind::Loop(body, None, LoopSource::ForLoop, _) = expr.kind
    && body.stmts.len() == 1
    && let StmtKind::Expr(e) = body.stmts[0].kind
    && let ExprKind::Match(scrutinee, arms, MatchSource::ForLoopDesugar) = e.kind
    && let ExprKind::Call(func, args) = scrutinee.kind
    && let ExprKind::Path(ref qpath) = func.kind
    && matches!(qpath, QPath::LangItem(LangItem::IteratorNext, _))
    && args.len() == 1
    && let ExprKind::AddrOf(BorrowKind::Ref, Mutability::Mut, inner) = args[0].kind
    && let ExprKind::Path(ref qpath1) = inner.kind
    && match_qpath(qpath1, &["iter"])
    && arms.len() == 2
    && let PatKind::Struct(ref qpath2, fields, false) = arms[0].pat.kind
    && matches!(qpath2, QPath::LangItem(LangItem::OptionNone, _))
    && fields.is_empty()
    && arms[0].guard.is_none()
    && let ExprKind::Break(destination, None) = arms[0].body.kind
    && destination.label.is_none()
    && let PatKind::Struct(ref qpath3, fields1, false) = arms[1].pat.kind
    && matches!(qpath3, QPath::LangItem(LangItem::OptionSome, _))
    && fields1.len() == 1
    && fields1[0].ident.as_str() == "0"
    && let PatKind::Wild = fields1[0].pat.kind
    && arms[1].guard.is_none()
    && let ExprKind::Block(block, None) = arms[1].body.kind
    && block.stmts.len() == 1
    && let StmtKind::Semi(e1) = block.stmts[0].kind
    && let ExprKind::Break(destination1, None) = e1.kind
    && destination1.label.is_none()
    && block.expr.is_none()
    && body.expr.is_none()
    // report your lint here
}
}
if let Some(higher::ForLoop { pat: pat, arg: arg, body: body, .. }) = higher::ForLoop::hir(expr)
    && let PatKind::Wild = pat.kind
    && let ExprKind::Struct(qpath, fields, None) = arg.kind
    && matches!(qpath, QPath::LangItem(LangItem::Range, _))
    && fields.len() == 2
    && fields[0].ident.as_str() == "start"
    && let ExprKind::Lit(ref lit) = fields[0].expr.kind
    && let LitKind::Int(0, LitIntType::Unsuffixed) = lit.node
    && fields[1].ident.as_str() == "end"
    && let ExprKind::Lit(ref lit1) = fields[1].expr.kind
    && let LitKind::Int(10, LitIntType::Unsuffixed) = lit1.node
    && let ExprKind::Block(block, None) = body.kind
    && block.stmts.len() == 1
    && let StmtKind::Semi(e) = block.stmts[0].kind
    && let ExprKind::Break(destination, None) = e.kind
    && let Some(label) = destination.label
    && label.ident.as_str() == "'label"
    && block.expr.is_none()
    // report your lint here
}
}
if let ExprKind::Loop(body, Some(label), LoopSource::ForLoop, _) = expr.kind
    && body.stmts.len() == 1
    && let StmtKind::Expr(e) = body.stmts[0].kind
    && let ExprKind::Match(scrutinee, arms, MatchSource::ForLoopDesugar) = e.kind
    && let ExprKind::Call(func, args) = scrutinee.kind
    && let ExprKind::Path(ref qpath) = func.kind
    && matches!(qpath, QPath::LangItem(LangItem::IteratorNext, _))
    && args.len() == 1
    && let ExprKind::AddrOf(BorrowKind::Ref, Mutability::Mut, inner) = args[0].kind
    && let ExprKind::Path(ref qpath1) = inner.kind
    && match_qpath(qpath1, &["iter"])
    && arms.len() == 2
    && let PatKind::Struct(ref qpath2, fields, false) = arms[0].pat.kind
    && matches!(qpath2, QPath::LangItem(LangItem::OptionNone, _))
    && fields.is_empty()
    && arms[0].guard.is_none()
    && let ExprKind::Break(destination, None) = arms[0].body.kind
    && destination.label.is_none()
    && let PatKind::Struct(ref qpath3, fields1, false) = arms[1].pat.kind
    && matches!(qpath3, QPath::LangItem(LangItem::OptionSome, _))
    && fields1.len() == 1
    && fields1[0].ident.as_str() == "0"
    && let PatKind::Wild = fields1[0].pat.kind
    && arms[1].guard.is_none()
    && let ExprKind::Block(block, None) = arms[1].body.kind
    && block.stmts.len() == 1
    && let StmtKind::Semi(e1) = block.stmts[0].kind
    && let ExprKind::Break(destination1, None) = e1.kind
    && let Some(label1) = destination1.label
    && label1.ident.as_str() == "'label"
    && block.expr.is_none()
    && body.expr.is_none()
    && label.ident.as_str() == "'label"
    // report your lint here
}
}
if let Some(higher::While { condition: condition, body: body }) = higher::While::hir(expr)
    && let ExprKind::Path(ref qpath) = condition.kind
    && match_qpath(qpath, &["a"])
    && let ExprKind::Block(block, None) = body.kind
    && block.stmts.len() == 1
    && let StmtKind::Semi(e) = block.stmts[0].kind
    && let ExprKind::Break(destination, None) = e.kind
    && destination.label.is_none()
    && block.expr.is_none()
    // report your lint here
}
}
if let Some(higher::WhileLet { let_pat: let_pat, let_expr: let_expr, if_then: if_then }) = higher::WhileLet::hir(expr)
    && let PatKind::Lit(lit_expr) = let_pat.kind
    && let ExprKind::Lit(ref lit) = lit_expr.kind
    && let LitKind::Bool(true) = lit.node
    && let ExprKind::Path(ref qpath) = let_expr.kind
    && match_qpath(qpath, &["a"])
    && let ExprKind::Block(block, None) = if_then.kind
    && block.stmts.len() == 1
    && let StmtKind::Semi(e) = block.stmts[0].kind
    && let ExprKind::Break(destination, None) = e.kind
    && destination.label.is_none()
    && block.expr.is_none()
    // report your lint here
}
}
if let ExprKind::Loop(body, None, LoopSource::Loop, _) = expr.kind
    && body.stmts.len() == 1
    && let StmtKind::Semi(e) = body.stmts[0].kind
    && let ExprKind::Break(destination, None) = e.kind
    && destination.label.is_none()
    && body.expr.is_none()
    // report your lint here
}

------------------------------------------
