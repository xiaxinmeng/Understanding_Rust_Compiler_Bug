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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_mir_transform/src/const_prop_lint.rs at line 285:
     }
 
     /// Returns the value, if any, of evaluating `c`.
-    fn eval_constant(
-        &mut self,
-        c: &Constant<'tcx>,
-        source_info: SourceInfo,
-    ) -> Option<OpTy<'tcx>> {
+    fn eval_constant(&mut self, c: &Constant<'tcx>, source_info: SourceInfo) -> Option<OpTy<'tcx>> {
         // FIXME we need to revisit this for #67176
         if c.needs_subst() {
             return None;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_transform/src/check_packed_ref.rs" "/checkout/compiler/rustc_mir_transform/src/const_prop_lint.rs" "/checkout/compiler/rustc_mir_transform/src/ffi_unwind_calls.rs" "/checkout/compiler/rustc_mir_transform/src/deref_separator.rs" "/checkout/compiler/rustc_mir_transform/src/deduplicate_blocks.rs" "/checkout/compiler/rustc_mir_transform/src/add_retag.rs" "/checkout/compiler/rustc_mir_transform/src/simplify_comparison_integral.rs" "/checkout/compiler/rustc_mir_transform/src/abort_unwinding_calls.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
