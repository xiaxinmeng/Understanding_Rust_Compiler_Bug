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
Diff in /checkout/compiler/rustc_typeck/src/check/method/suggest.rs at line 69:
 
 
     fn is_slice_ty(&self, ty: Ty<'tcx>, span: Span) -> bool {
-        self.autoderef(span, ty).any(|(ty, _)| {
-            self.probe(|_| {
-                matches!(ty.kind(), ty::Slice(..) | ty::Array(..))
-        })
-        })
+        self.autoderef(span, ty)
+            .any(|(ty, _)| self.probe(|_| matches!(ty.kind(), ty::Slice(..) | ty::Array(..))))
 
     pub fn report_method_error(
     pub fn report_method_error(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/method/mod.rs" "/checkout/compiler/rustc_typeck/src/check/expr.rs" "/checkout/compiler/rustc_typeck/src/check/compare_method.rs" "/checkout/compiler/rustc_typeck/src/check/upvar.rs" "/checkout/compiler/rustc_typeck/src/check/method/suggest.rs" "/checkout/compiler/rustc_typeck/src/check/cast.rs" "/checkout/compiler/rustc_typeck/src/check/callee.rs" "/checkout/compiler/rustc_typeck/src/outlives/explicit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
