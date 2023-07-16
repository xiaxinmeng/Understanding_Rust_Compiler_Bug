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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_mir_build/src/thir/pattern/check_match.rs at line 366:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/thir/pattern/check_match.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 
 
 fn irrefutable_let_pattern(tcx: TyCtxt<'_>, span: Span, id: HirId, source: hir::MatchSource) {
-    tcx.struct_span_lint_hir(IRREFUTABLE_LET_PATTERNS, id, span, |lint| {
-        match source {
-            hir::MatchSource::IfLetDesugar { .. } => {
-                let mut diag = lint.build("irrefutable if-let pattern");
-                diag.help("this pattern will always match, so the if-let is useless");
-                diag.help("consider removing the if-let and using a `let`");
-                diag.emit()
-            }
-            hir::MatchSource::WhileLetDesugar => {
-                let mut diag = lint.build("irrefutable while-let pattern");
-                diag.help("this pattern will always match, so the loop will never exit");
-                diag.help("consider using a `loop { ... }` and adding a `let` inside it");
-                diag.emit()
-            }
-            hir::MatchSource::IfLetGuardDesugar => {
-                let mut diag = lint.build("irrefutable if-let guard pattern");
-                diag.help("this pattern will always match, so the guard is useless");
-                diag.help("consider removing the guard and adding a `let` inside the match arm");
-                diag.emit()
-            }
-            _ => bug!("expected if-let, while-let, or if-let guard HIR match source, found {:?}", source),
+    tcx.struct_span_lint_hir(IRREFUTABLE_LET_PATTERNS, id, span, |lint| match source {
+        hir::MatchSource::IfLetDesugar { .. } => {
+            let mut diag = lint.build("irrefutable if-let pattern");
+            diag.help("this pattern will always match, so the if-let is useless");
+            diag.help("consider removing the if-let and using a `let`");
+            diag.emit()
+        }
+        hir::MatchSource::WhileLetDesugar => {
+            let mut diag = lint.build("irrefutable while-let pattern");
+            diag.help("this pattern will always match, so the loop will never exit");
+            diag.help("consider using a `loop { ... }` and adding a `let` inside it");
+            diag.emit()
+        }
+        hir::MatchSource::IfLetGuardDesugar => {
+            let mut diag = lint.build("irrefutable if-let guard pattern");
+            diag.help("this pattern will always match, so the guard is useless");
+            diag.help("consider removing the guard and adding a `let` inside the match arm");
+            diag.emit()
+        _ => {
+        _ => {
+            bug!("expected if-let, while-let, or if-let guard HIR match source, found {:?}", source)
     });
 }
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:16
