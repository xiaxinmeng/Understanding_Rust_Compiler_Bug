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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_lint/src/internal.rs at line 123:
 ]);
 
 impl<'tcx> LateLintPass<'tcx> for TyTyKind {
-    fn check_path(&mut self,cx: &LateContext< 'tcx>,path: & 'tcx rustc_hir::Path< 'tcx>,_:rustc_hir::HirId) {
+        &mut self,
+        &mut self,
+        cx: &LateContext<'tcx>,
+        path: &'tcx rustc_hir::Path<'tcx>,
+        _: rustc_hir::HirId,
+    ) {
         if let Some(segment) = path.segments.iter().nth_back(1)
         && let Some(res) = &segment.res
         && lint_ty_kind_usage(cx, res)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/tests.rs" "/checkout/compiler/rustc_lint/src/late.rs" "/checkout/compiler/rustc_lint/src/array_into_iter.rs" "/checkout/compiler/rustc_lint/src/lib.rs" "/checkout/compiler/rustc_lint/src/passes.rs" "/checkout/compiler/rustc_lint/src/internal.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
