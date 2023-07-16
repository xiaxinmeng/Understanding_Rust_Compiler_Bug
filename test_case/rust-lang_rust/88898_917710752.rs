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
Diff in /checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs at line 162:
     fn register_unique_id_with_type_name(
         &mut self,
         unique_type_id: UniqueTypeId,
+        type_name: String,
     ) {
     ) {
         if self.unique_id_to_type_name.insert(unique_type_id, type_name).is_some() {
             bug!(
Diff in /checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs at line 350:
 }
 
-fn check_type_name_cache(
-fn check_type_name_cache(
-    cx: &CodegenCx<'ll, 'tcx>,
-    qualified: bool,
-) -> String {
-) -> String {
+fn check_type_name_cache(cx: &CodegenCx<'ll, 'tcx>, ty: Ty<'tcx>, qualified: bool) -> String {
     let mut type_map = debug_context(cx).type_map.borrow_mut();
     let unique_type_id = type_map.get_unique_type_id_of_type(cx, ty);
     match type_map.find_type_name_for_unique_id(unique_type_id) {
Diff in /checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs at line 361:
-        Some(type_name) => { type_name },
+        Some(type_name) => type_name,
         None => {
             let type_name = compute_debuginfo_type_name(cx.tcx, ty, qualified);
-            type_map.register_unique_id_with_type_name(
-                unique_type_id,
-                type_name.clone(),
-            );
+            type_map.register_unique_id_with_type_name(unique_type_id, type_name.clone());
-        },
+        }
     }
 }
 }
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/namespace.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/intrinsic.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/create_scope_map.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/backend.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/coverageinfo.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/gdb.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
