plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Diff in /checkout/compiler/rustc_passes/src/dead.rs at line 734:
         }
     }
 
-    fn warn_multiple_dead_codes(&self, dead_codes: &[(hir::HirId, Span, Symbol)], participle: &str) {
+    fn warn_multiple_dead_codes(
+        &self,
+        dead_codes: &[(hir::HirId, Span, Symbol)],
+        participle: &str,
+    ) {
         if let Some((id, _, name)) = dead_codes.first()
             && !name.as_str().starts_with('_')
         {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/rustbook/src/main.rs" "/checkout/src/tools/rustc-workspace-hack/lib.rs" "/checkout/compiler/rustc_passes/src/dead.rs" "/checkout/compiler/rustc_passes/src/upvars.rs" "/checkout/src/tools/linkchecker/main.rs" "/checkout/compiler/rustc_passes/src/lib_features.rs" "/checkout/src/tools/linkchecker/tests/checks.rs" "/checkout/compiler/rustc_passes/src/hir_stats.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
