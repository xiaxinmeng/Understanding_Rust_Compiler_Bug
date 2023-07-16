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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_infer/src/infer/canonical/query_response.rs at line 219:
         original_values: &OriginalQueryValues<'tcx>,
         query_response: &Canonical<'tcx, QueryResponse<'tcx, R>>,
         output_query_region_constraints: &mut QueryRegionConstraints<'tcx>,
-        obligations: &mut PredicateObligations<'tcx>
+        obligations: &mut PredicateObligations<'tcx>,
     ) -> Result<R, TypeError<'tcx>>
     where
         R: Debug + TypeFoldable<'tcx>,
Diff in /checkout/compiler/rustc_trait_selection/src/traits/query/type_op/mod.rs at line 1:
 use crate::infer::canonical::{
     Canonicalized, CanonicalizedQueryResponse, OriginalQueryValues, QueryRegionConstraints,
 };
-use crate::infer::{InferCtxt};
+use crate::infer::InferCtxt;
 use crate::traits::query::Fallible;
 use crate::traits::ObligationCause;
 use rustc_infer::infer::canonical::{Canonical, Certainty};
Diff in /checkout/compiler/rustc_trait_selection/src/traits/query/type_op/mod.rs at line 82:
         infcx: &InferCtxt<'_, 'tcx>,
         output_query_region_constraints: &mut QueryRegionConstraints<'tcx>,
         obligations: &mut PredicateObligations<'tcx>,
-    ) -> Fallible<(
-        Self::QueryResponse,
-        Option<Canonical<'tcx, ParamEnvAnd<'tcx, Self>>>,
-        Certainty,
-    )> {
+    ) -> Fallible<(Self::QueryResponse, Option<Canonical<'tcx, ParamEnvAnd<'tcx, Self>>>, Certainty)>
+    {
         if let Some(result) = QueryTypeOp::try_fast_path(infcx.tcx, &query_key) {
             return Ok((result, None, Certainty::Proven));
         }
Diff in /checkout/compiler/rustc_trait_selection/src/traits/query/type_op/mod.rs at line 101:
             infcx.canonicalize_query_keep_static(query_key, &mut canonical_var_values);
         let canonical_result = Self::perform_query(infcx.tcx, canonical_self)?;
-        let value = infcx
-        let value = infcx
-            .instantiate_nll_query_response_and_region_obligations(
-                &ObligationCause::dummy(),
-                old_param_env,
-                &canonical_var_values,
-                canonical_result,
-                output_query_region_constraints,
-                obligations,
-            )?;
+        let value = infcx.instantiate_nll_query_response_and_region_obligations(
+            &ObligationCause::dummy(),
+            old_param_env,
+            &canonical_var_values,
+            canonical_result,
+            output_query_region_constraints,
+            obligations,
+        )?;
 
         Ok((value, Some(canonical_self), canonical_result.value.certainty))
Build completed unsuccessfully in 0:00:13
Build completed unsuccessfully in 0:00:13
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/different_lifetimes.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/placeholder_error.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/util.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/static_impl_trait.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/note.rs" "/checkout/compiler/rustc_infer/src/infer/canonical/query_response.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
