plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 78a891d364a7358ed9eb9c93099ba2f3e6817ca6 and bf6b73f13d216e85e65db14d01f64913fd3f5da9
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stdout:

 if_chain! {
     if let ExprKind::Struct(qpath, fields, None) = expr.kind;
     if match_qpath(qpath, &["Test"]);
     if fields.len() == 1;
     if fields[0].ident.as_str() == "field";
     if let ExprKind::If(cond, then, Some(else_expr)) = fields[0].expr.kind;
     if let ExprKind::DropTemps(expr1) = cond.kind;
     if let ExprKind::Lit(ref lit) = expr1.kind;
     if let LitKind::Bool(true) = lit.node;
     if let ExprKind::Block(block, None) = then.kind;
     if block.stmts.is_empty();
     if let Some(trailing_expr) = block.expr;
     if let ExprKind::Lit(ref lit1) = trailing_expr.kind;
     if let LitKind::Int(1, LitIntType::Unsuffixed) = lit1.node;
     if let ExprKind::Block(block1, None) = else_expr.kind;
     if block1.stmts.is_empty();
     if let Some(trailing_expr1) = block1.expr;
     if let ExprKind::Lit(ref lit2) = trailing_expr1.kind;
     if let LitKind::Int(0, LitIntType::Unsuffixed) = lit2.node;
     then {
         // report your lint here
 }
 if_chain! {
 if_chain! {
     if let PatKind::Struct(ref qpath, fields, false) = arm.pat.kind;
     if match_qpath(qpath, &["Test"]);
     if fields.len() == 1;
     if fields[0].ident.as_str() == "field";
     if let PatKind::Lit(lit_expr) = fields[0].pat.kind;
     if let ExprKind::Lit(ref lit) = lit_expr.kind;
     if let LitKind::Int(1, LitIntType::Unsuffixed) = lit.node;
     if arm.guard.is_none();
     if let ExprKind::Block(block, None) = arm.body.kind;
     if block.stmts.is_empty();
     if block.expr.is_none();
     then {
         // report your lint here
 }
 if_chain! {
 if_chain! {
-    if let PatKind::TupleStruct(ref qpath, fields, None) = arm.pat.kind;
+    if let PatKind::TupleStruct(ref qpath, fields, DotDotPos(4294967295)) = arm.pat.kind;
     if match_qpath(qpath, &["TestTuple"]);
     if fields.len() == 1;
     if let PatKind::Lit(lit_expr) = fields[0].kind;
     if let ExprKind::Lit(ref lit) = lit_expr.kind;
     if let LitKind::Int(1, LitIntType::Unsuffixed) = lit.node;
     if arm.guard.is_none();
     if let ExprKind::Block(block, None) = arm.body.kind;
     if block.stmts.is_empty();
     if block.expr.is_none();
     then {
         // report your lint here
 }
 if_chain! {
 if_chain! {
     if let ExprKind::MethodCall(method_name, receiver, args, _) = expr.kind;
     if method_name.ident.as_str() == "test";
     if let ExprKind::Path(ref qpath) = receiver.kind;
     if match_qpath(qpath, &["test_method_call"]);
     if args.is_empty();
     then {
         // report your lint here
 }
 

The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/struct.stage-id.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author/struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author/struct.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/struct.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-eed8221ad604f845.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-e92c80438a94fde5.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c6aa3eacac0eeebb.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-a74d54f3750f2f75.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4b46e2e2788394f2.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-14dbc812a1f5dba0.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-47229815ed3188f9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/author/struct.stage-id.aux"
------------------------------------------
if_chain! {
if_chain! {
    if let ExprKind::Struct(qpath, fields, None) = expr.kind;
    if match_qpath(qpath, &["Test"]);
    if fields.len() == 1;
    if fields[0].ident.as_str() == "field";
    if let ExprKind::If(cond, then, Some(else_expr)) = fields[0].expr.kind;
    if let ExprKind::DropTemps(expr1) = cond.kind;
    if let ExprKind::Lit(ref lit) = expr1.kind;
    if let LitKind::Bool(true) = lit.node;
    if let ExprKind::Block(block, None) = then.kind;
    if block.stmts.is_empty();
    if let Some(trailing_expr) = block.expr;
    if let ExprKind::Lit(ref lit1) = trailing_expr.kind;
    if let LitKind::Int(1, LitIntType::Unsuffixed) = lit1.node;
    if let ExprKind::Block(block1, None) = else_expr.kind;
    if block1.stmts.is_empty();
    if let Some(trailing_expr1) = block1.expr;
    if let ExprKind::Lit(ref lit2) = trailing_expr1.kind;
    if let LitKind::Int(0, LitIntType::Unsuffixed) = lit2.node;
    then {
        // report your lint here
}
if_chain! {
if_chain! {
    if let PatKind::Struct(ref qpath, fields, false) = arm.pat.kind;
    if match_qpath(qpath, &["Test"]);
    if fields.len() == 1;
    if fields[0].ident.as_str() == "field";
    if let PatKind::Lit(lit_expr) = fields[0].pat.kind;
    if let ExprKind::Lit(ref lit) = lit_expr.kind;
    if let LitKind::Int(1, LitIntType::Unsuffixed) = lit.node;
    if arm.guard.is_none();
    if let ExprKind::Block(block, None) = arm.body.kind;
    if block.stmts.is_empty();
    if block.expr.is_none();
    then {
        // report your lint here
}
if_chain! {
if_chain! {
    if let PatKind::TupleStruct(ref qpath, fields, DotDotPos(4294967295)) = arm.pat.kind;
    if match_qpath(qpath, &["TestTuple"]);
    if fields.len() == 1;
    if let PatKind::Lit(lit_expr) = fields[0].kind;
    if let ExprKind::Lit(ref lit) = lit_expr.kind;
    if let LitKind::Int(1, LitIntType::Unsuffixed) = lit.node;
    if arm.guard.is_none();
    if let ExprKind::Block(block, None) = arm.body.kind;
    if block.stmts.is_empty();
    if block.expr.is_none();
    then {
        // report your lint here
}
if_chain! {
if_chain! {
    if let ExprKind::MethodCall(method_name, receiver, args, _) = expr.kind;
    if method_name.ident.as_str() == "test";
    if let ExprKind::Path(ref qpath) = receiver.kind;
    if match_qpath(qpath, &["test_method_call"]);
    if args.is_empty();
    then {
        // report your lint here
}

------------------------------------------
stderr:
