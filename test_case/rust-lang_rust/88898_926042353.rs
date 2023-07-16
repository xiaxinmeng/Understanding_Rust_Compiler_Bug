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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs at line 375:
 }
 
 fn check_type_name_cache(cx: &CodegenCx<'ll, 'tcx>, ty: Ty<'tcx>, qualified: bool) -> String {
-    compute_debuginfo_type_name(cx.tcx, ty, qualified, &mut debug_context(cx).type_name_cache.borrow_mut())
+    compute_debuginfo_type_name(
+        cx.tcx,
+        ty,
+        qualified,
+        &mut debug_context(cx).type_name_cache.borrow_mut(),
 }
 
 fn fixed_vec_metadata(
 fn fixed_vec_metadata(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/back/write.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/utils.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/profiling.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/create_scope_map.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/diagnostic.rs" "/checkout/compiler/rustc_codegen_llvm/src/abi.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/archive_ro.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
