plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_lint/src/enum_intrinsics_non_enums.rs at line 36:
 
 declare_lint_pass!(EnumIntrinsicsNonEnums => [ENUM_INTRINSICS_NON_ENUMS]);
 
-fn enforce_mem_discriminant(cx: &LateContext<'_>, func_expr: &hir::Expr<'_>, expr_span: Span, args_span: Span) {
+fn enforce_mem_discriminant(
+    cx: &LateContext<'_>,
+    func_expr: &hir::Expr<'_>,
+    expr_span: Span,
+    args_span: Span,
+) {
     let ty_param = cx.typeck_results().node_substs(func_expr.hir_id).type_at(0);
     if !ty_param.is_enum() {
         cx.struct_span_lint(ENUM_INTRINSICS_NON_ENUMS, expr_span, |builder| {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/late.rs" "/checkout/src/tools/tidy/src/unstable_book.rs" "/checkout/src/tools/rustdoc/main.rs" "/checkout/src/tools/tidy/src/style.rs" "/checkout/src/tools/tidy/src/features.rs" "/checkout/compiler/rustc_lint/src/enum_intrinsics_non_enums.rs" "/checkout/src/tools/tidy/src/error_codes_check.rs" "/checkout/src/tools/tidy/src/deps.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
