diff
diff --git a/src/librustc/traits/error_reporting.rs b/src/librustc/traits/error_reporting.rs
index 96d42dd73b1..0e021d658aa 100644
--- a/src/librustc/traits/error_reporting.rs
+++ b/src/librustc/traits/error_reporting.rs
@@ -1176,7 +1176,7 @@ impl<'a, 'tcx> InferCtxt<'a, 'tcx> {
                             // `fn foo<T>(t: T) where T: Debug {}` →
                             // `fn foo<T>(t: T) where T: Debug, T: Trait {}`
                             err.span_suggestion(
-                                generics.where_clause.span().unwrap().shrink_to_hi(),
+                                generics.where_clause.predicates, // TODO: fixme
                                 &format!(
                                     "consider further restricting type parameter `{}`",
                                     param_name,
diff --git a/src/librustc_typeck/check/mod.rs b/src/librustc_typeck/check/mod.rs
index 9107e993311..83a90e64eb1 100644
--- a/src/librustc_typeck/check/mod.rs
+++ b/src/librustc_typeck/check/mod.rs
@@ -1229,7 +1229,7 @@ fn check_fn<'a, 'tcx>(
         // for simple cases like `fn foo(x: Trait)`,
         // where we would error once on the parameter as a whole, and once on the binding `x`.
         if param.pat.simple_ident().is_none() && !fcx.tcx.features().unsized_locals {
-            fcx.require_type_is_sized(param_ty, decl.output.span(), traits::SizedArgumentType);
+            fcx.require_type_is_sized(param_ty, param.pat.span, traits::SizedArgumentType);
         }

         fcx.write_ty(param.hir_id, param_ty);
