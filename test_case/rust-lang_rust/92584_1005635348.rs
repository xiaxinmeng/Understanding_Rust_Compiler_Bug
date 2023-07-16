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
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 1070:
     }
 
     /// Warns against some misuses of `#[must_use]`
-    fn check_must_use(
-        &self,
-        hir_id: HirId,
-        attr: &Attribute,
-        span: Span,
-        _target: Target,
-    ) -> bool {
+    fn check_must_use(&self, hir_id: HirId, attr: &Attribute, span: Span, _target: Target) -> bool {
         let node = self.tcx.hir().get(hir_id);
         if let Some(fn_node) = node.fn_kind() {
             if let rustc_hir::IsAsync::Async = fn_node.asyncness() {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_passes/src/naked_functions.rs" "/checkout/compiler/rustc_passes/src/check_attr.rs" "/checkout/compiler/rustc_passes/src/intrinsicck.rs" "/checkout/compiler/rustc_passes/src/lang_items.rs" "/checkout/compiler/rustc_passes/src/hir_stats.rs" "/checkout/compiler/rustc_passes/src/region.rs" "/checkout/compiler/rustc_data_structures/src/base_n.rs" "/checkout/compiler/rustc_passes/src/liveness.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
