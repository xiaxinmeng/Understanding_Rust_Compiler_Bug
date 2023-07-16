plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:11f435bab7d0fe51e2f33ae0ce77f457216db8a6)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Successfully built 73204e7f0008
Successfully tagged rust-ci:latest
Built container sha256:73204e7f00088ac0451ee114d683dd746286291c8f9a0dff9d75e6df232eafbe
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281
upload failed: - to s3://rust-lang-ci-sccache2/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 15.85s
##[endgroup]
fmt check
Diff in /checkout/compiler/rustc_borrowck/src/nll.rs at line 195:
     if infcx.tcx.trait_solver_next() {
         assert!(opaque_type_values.is_empty());
-    
+
+
     if let Some(all_facts) = &mut all_facts {
         let _prof_timer = infcx.tcx.prof.generic_activity("polonius_fact_generation");
         all_facts.universal_region.extend(universal_regions.universal_regions());
Diff in /checkout/compiler/rustc_borrowck/src/region_infer/opaque_types.rs at line 44:
 
             let substs = opaque_ty.substs;
             let substs = opaque_ty.substs;
-            let opaque_type_key = OpaqueTypeKey {
-                def_id: opaque_ty.def_id.expect_local(),
-                substs,
-            };
+            let opaque_type_key = OpaqueTypeKey { def_id: opaque_ty.def_id.expect_local(), substs };
 
             let mut subst_regions = vec![self.universal_regions.fr_static];
Diff in /checkout/compiler/rustc_borrowck/src/region_infer/opaque_types.rs at line 114:
Diff in /checkout/compiler/rustc_borrowck/src/region_infer/opaque_types.rs at line 114:
                 OpaqueTypeKey { def_id: opaque_type_key.def_id, substs: universal_substs };
             let ty = infcx.infer_opaque_definition_from_instantiation(
-                OpaqueHiddenType {
-                OpaqueHiddenType {
-                    span: DUMMY_SP,
-                    ty: universal_concrete_type,
-                },
+                OpaqueHiddenType { span: DUMMY_SP, ty: universal_concrete_type },
                 OpaqueTyOrigin::TyAlias,
             );
             // Sometimes two opaque types are the same only after we remap the generic parameters
Diff in /checkout/compiler/rustc_borrowck/src/region_infer/opaque_types.rs at line 127:
             if let Some(prev) = result.get_mut(&opaque_type_key.def_id) {
                 if prev.ty != ty {
                     let guar = ty.error_reported().err().unwrap_or_else(|| {
-                        prev.report_mismatch(
-                            &OpaqueHiddenType { ty, span: DUMMY_SP },
-                            infcx.tcx,
-                        )
+                        prev.report_mismatch(&OpaqueHiddenType { ty, span: DUMMY_SP }, infcx.tcx)
                     });
                     prev.ty = infcx.tcx.ty_error(guar);
Diff in /checkout/compiler/rustc_borrowck/src/region_infer/opaque_types.rs at line 138:
Diff in /checkout/compiler/rustc_borrowck/src/region_infer/opaque_types.rs at line 138:
                 // FIXME(oli-obk): collect multiple spans for better diagnostics down the road.
                 prev.span = prev.span;
-                result.insert(
-                    opaque_type_key.def_id,
-                    opaque_type_key.def_id,
-                    OpaqueHiddenType { ty, span: DUMMY_SP },
-                );
+                result.insert(opaque_type_key.def_id, OpaqueHiddenType { ty, span: DUMMY_SP });
         }
         result
Diff in /checkout/compiler/rustc_borrowck/src/region_infer/opaque_types.rs at line 148:
     }
     }
-
 
     /// Resolve any opaque types that were encountered while borrow checking
     /// this item. This is then used to get the type in the `type_of` query.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/lib.rs" "/checkout/compiler/rustc_borrowck/src/nll.rs" "/checkout/compiler/rustc_borrowck/src/used_muts.rs" "/checkout/compiler/rustc_baked_icu_data/src/data/fallback/supplement/mod.rs" "/checkout/compiler/rustc_borrowck/src/place_ext.rs" "/checkout/compiler/rustc_borrowck/src/invalidation.rs" "/checkout/compiler/rustc_borrowck/src/dataflow.rs" "/checkout/compiler/rustc_borrowck/src/borrow_set.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
