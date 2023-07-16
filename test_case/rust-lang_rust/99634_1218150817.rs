plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_lint/src/levels.rs at line 187:
         intravisit::walk_field_def(self, s);
 
-    fn visit_variant(
-        &mut self,
-        &mut self,
-        v: &'tcx hir::Variant<'tcx>,
-    ) {
+    fn visit_variant(&mut self, v: &'tcx hir::Variant<'tcx>) {
         self.add_id(v.id);
         intravisit::walk_variant(self, v);
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/nonstandard_style.rs" "/checkout/compiler/rustc_lint/src/levels.rs" "/checkout/compiler/rustc_lint/src/types.rs" "/checkout/compiler/rustc_lint/src/enum_intrinsics_non_enums.rs" "/checkout/compiler/rustc_lint/src/context.rs" "/checkout/compiler/rustc_lint/src/hidden_unicode_codepoints.rs" "/checkout/compiler/rustc_lint/src/late.rs" "/checkout/compiler/rustc_mir_build/src/thir/cx/block.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
