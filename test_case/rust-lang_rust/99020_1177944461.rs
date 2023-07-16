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

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_typeck/src/check/check.rs at line 1333:
             return (span, zst, align1, None);
 
 
-        fn check_non_exhaustive<'tcx>(tcx: TyCtxt<'tcx>, t: Ty<'tcx>) -> ControlFlow<(&'static str, DefId, SubstsRef<'tcx>, bool)> {
+        fn check_non_exhaustive<'tcx>(
+            tcx: TyCtxt<'tcx>,
+            t: Ty<'tcx>,
+        ) -> ControlFlow<(&'static str, DefId, SubstsRef<'tcx>, bool)> {
             match t.kind() {
                 ty::Tuple(list) => list.iter().try_for_each(|t| check_non_exhaustive(tcx, t)),
                 ty::Array(ty, _) => check_non_exhaustive(tcx, *ty),
Diff in /checkout/compiler/rustc_typeck/src/check/check.rs at line 1340:
                 ty::Adt(def, subst) => {
                     if !def.did().is_local() {
-                        let non_exhaustive = def.is_variant_list_non_exhaustive() || def.variants().iter().any(ty::VariantDef::is_field_list_non_exhaustive);
+                        let non_exhaustive = def.is_variant_list_non_exhaustive()
+                            || def
+                                .variants()
+                                .iter()
+                                .any(ty::VariantDef::is_field_list_non_exhaustive);
                         let has_priv = def.all_fields().any(|f| !f.vis.is_public());
                         if non_exhaustive || has_priv {
-                            return ControlFlow::Break((def.descr(), def.did(), subst, non_exhaustive))
+                            return ControlFlow::Break((
+                                def.descr(),
+                                def.did(),
+                                non_exhaustive,
+                            ));
                         }
                     }
                     }
-                    def.all_fields().map(|field| field.ty(tcx, subst)).try_for_each(|t| check_non_exhaustive(tcx, t))
+                    def.all_fields()
+                        .map(|field| field.ty(tcx, subst))
+                        .try_for_each(|t| check_non_exhaustive(tcx, t))
                 }
                 _ => ControlFlow::Continue(()),
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/impl_wf_check.rs" "/checkout/compiler/rustc_typeck/src/hir_wf_check.rs" "/checkout/compiler/rustc_typeck/src/mem_categorization.rs" "/checkout/compiler/rustc_typeck/src/structured_errors/wrong_number_of_generic_args.rs" "/checkout/compiler/rustc_typeck/src/constrained_generic_params.rs" "/checkout/compiler/rustc_typeck/src/structured_errors/sized_unsized_cast.rs" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/structured_errors/missing_cast_for_variadic_arg.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
