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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_ty_utils/src/needs_drop.rs at line 10:
 
 type NeedsDropResult<T> = Result<T, AlwaysRequiresDrop>;
 
-fn needs_drop_raw<'tcx>(
-    tcx: TyCtxt<'tcx>,
-    query: ty::ParamEnvAnd<'tcx, Ty<'tcx>>,
-) -> bool {
-    let adt_components = move |adt_def: &ty::AdtDef| tcx.adt_drop_tys(adt_def.did).map(|tys| tys.iter());
+fn needs_drop_raw<'tcx>(tcx: TyCtxt<'tcx>, query: ty::ParamEnvAnd<'tcx, Ty<'tcx>>) -> bool {
+    let adt_components =
+        move |adt_def: &ty::AdtDef| tcx.adt_drop_tys(adt_def.did).map(|tys| tys.iter());
 
     // If we don't know a type doesn't need drop, for example if it's a type
     // parameter without a `Copy` bound, then we conservatively return that it
Diff in /checkout/compiler/rustc_ty_utils/src/needs_drop.rs at line 21:
     // needs drop.
-    let res = NeedsDropTypes::new(tcx, query.param_env, query.value, adt_components)
-            .next()
-            .is_some();
+    let res =
+        NeedsDropTypes::new(tcx, query.param_env, query.value, adt_components).next().is_some();
 
     debug!("needs_drop_raw({:?}) = {:?}", query, res);
Diff in /checkout/compiler/rustc_ty_utils/src/needs_drop.rs at line 33:
 ) -> bool {
     let significant_drop_fields =
     let significant_drop_fields =
         move |adt_def: &ty::AdtDef| tcx.adt_significant_drop_tys(adt_def.did).map(|tys| tys.iter());
-    let res =
-        NeedsDropTypes::new(tcx, query.param_env, query.value, significant_drop_fields)
-            .next()
-            .is_some();
+    let res = NeedsDropTypes::new(tcx, query.param_env, query.value, significant_drop_fields)
+        .next()
+        .is_some();
     debug!("has_significant_drop_raw({:?}) = {:?}", query, res);
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ty_utils/src/instance.rs" "/checkout/compiler/rustc_error_codes/src/error_codes.rs" "/checkout/compiler/rustc_ty_utils/src/needs_drop.rs" "/checkout/compiler/rustc_error_codes/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
