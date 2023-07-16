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
Diff in /checkout/compiler/rustc_typeck/src/check/check.rs at line 717:
         );
 
         for (OpaqueTypeKey { def_id, substs }, opaque_defn) in opaque_type_map {
-            match infcx.at(&misc_cause, param_env).eq(
-                opaque_defn.concrete_ty,
-                tcx.type_of(def_id).subst(tcx, substs),
-            ) {
+            match infcx
+                .at(&misc_cause, param_env)
+                .eq(opaque_defn.concrete_ty, tcx.type_of(def_id).subst(tcx, substs))
+            {
                 Ok(infer_ok) => inh.register_infer_ok_obligations(infer_ok),
                 Err(ty_err) => tcx.sess.delay_span_bug(
                     opaque_defn.definition_span,
Diff in /checkout/compiler/rustc_typeck/src/check/writeback.rs at line 475:
 
 
     fn visit_opaque_types(&mut self, span: Span) {
-        for &(opaque_type_key @ OpaqueTypeKey { def_id, substs }, opaque_defn) in self.fcx.opaque_types.borrow().iter() {
-            let hir_id =
-                self.tcx().hir().local_def_id_to_hir_id(def_id.expect_local());
+        for &(opaque_type_key @ OpaqueTypeKey { def_id, substs }, opaque_defn) in
+            self.fcx.opaque_types.borrow().iter()
+        {
+            let hir_id = self.tcx().hir().local_def_id_to_hir_id(def_id.expect_local());
             let instantiated_ty = self.resolve(opaque_defn.concrete_ty, &hir_id);
 
             debug_assert!(!instantiated_ty.has_escaping_bound_vars());
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/diverges.rs" "/checkout/compiler/rustc_typeck/src/check/inherited.rs" "/checkout/compiler/rustc_errors/src/diagnostic.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior.rs" "/checkout/compiler/rustc_errors/src/styled_buffer.rs" "/checkout/compiler/rustc_typeck/src/check/writeback.rs" "/checkout/compiler/rustc_errors/src/snippet.rs" "/checkout/compiler/rustc_typeck/src/check/pat.rs"` failed.
Build completed unsuccessfully in 0:00:15
Build completed unsuccessfully in 0:00:15
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
