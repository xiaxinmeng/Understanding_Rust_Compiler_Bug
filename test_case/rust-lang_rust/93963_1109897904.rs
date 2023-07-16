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
Diff in /checkout/src/librustdoc/clean/mod.rs at line 379:
     }
 }
 
-fn clean_projection<'tcx>(ty: ty::ProjectionTy<'tcx>, cx: &mut DocContext<'_>, def_id: Option<DefId>) -> Type {
+fn clean_projection<'tcx>(
+    ty: ty::ProjectionTy<'tcx>,
+    cx: &mut DocContext<'_>,
+    def_id: Option<DefId>,
+) -> Type {
     let lifted = ty.lift_to_tcx(cx.tcx).unwrap();
     let trait_ = lifted.trait_ref(cx.tcx).clean(cx);
     let self_type = ty.self_ty().clean(cx);
Diff in /checkout/src/librustdoc/clean/mod.rs at line 1591:
         }
         ty::Tuple(t) => Tuple(t.iter().map(|t| t.clean(cx)).collect()),
 
-        ty::Projection(ref data) => {
-            clean_projection(*data, cx, def_id)
-        }
+        ty::Projection(ref data) => clean_projection(*data, cx, def_id),
 
         ty::Param(ref p) => {
             if let Some(bounds) = cx.impl_trait_bounds.remove(&p.index.into()) {
Diff in /checkout/src/librustdoc/clean/mod.rs at line 1640:
                     let bindings: Vec<_> = bounds
                         .iter()
                         .filter_map(|bound| {
-                            if let ty::PredicateKind::Projection(proj) =
-                                bound.kind().skip_binder()
+                            if let ty::PredicateKind::Projection(proj) = bound.kind().skip_binder()
                             {
-                                if proj.projection_ty.trait_ref(cx.tcx)
-                                    == trait_ref.skip_binder()
-                                {
+                                if proj.projection_ty.trait_ref(cx.tcx) == trait_ref.skip_binder() {
                                     Some(TypeBinding {
-                                        assoc: projection_to_path_segment(
-                                            proj.projection_ty,
-                                            cx,
-                                        ),
+                                        assoc: projection_to_path_segment(proj.projection_ty, cx),
                                         kind: TypeBindingKind::Equality {
                                             term: proj.term.clean(cx),
                                         },
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/etc/test-float-parse/src/lib.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/etc/test-float-parse/src/bin/long-fractions.rs" "/checkout/src/etc/test-float-parse/src/bin/u32-small.rs" "/checkout/src/etc/test-float-parse/src/bin/tiny-pow10.rs" "/checkout/src/librustdoc/clean/cfg/tests.rs" "/checkout/src/etc/test-float-parse/src/bin/few-ones.rs" "/checkout/src/librustdoc/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
