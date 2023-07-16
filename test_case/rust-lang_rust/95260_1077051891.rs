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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 1086:
 
     /// Compares two given types, eliding parts that are the same between them and highlighting
     /// relevant differences, and return two representation of those types for highlighted printing.
-    pub fn cmp(&self, t1: Ty<'tcx>, t2: Ty<'tcx>) -> (DiagnosticStyledString, DiagnosticStyledString) {
+    pub fn cmp(
+        &self,
+        t1: Ty<'tcx>,
+        t2: Ty<'tcx>,
+    ) -> (DiagnosticStyledString, DiagnosticStyledString) {
         debug!("cmp(t1={}, t1.kind={:?}, t2={}, t2.kind={:?})", t1, t1.kind(), t2, t2.kind());
         // helper functions
         // helper functions
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/lattice.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/note.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs" "/checkout/compiler/rustc_infer/src/infer/region_constraints/leak_check.rs" "/checkout/compiler/rustc_infer/src/infer/equate.rs" "/checkout/compiler/rustc_infer/src/traits/project.rs" "/checkout/compiler/rustc_infer/src/infer/freshen.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
