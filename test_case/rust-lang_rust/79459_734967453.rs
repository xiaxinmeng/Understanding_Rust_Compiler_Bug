diff
diff --git a/compiler/rustc_trait_selection/src/traits/select/mod.rs b/compiler/rustc_trait_selection/src/traits/select/mod.rs
index 5b31998b7d3..8a78fa96d53 100644
--- a/compiler/rustc_trait_selection/src/traits/select/mod.rs
+++ b/compiler/rustc_trait_selection/src/traits/select/mod.rs
@@ -218,7 +218,7 @@ pub fn new(infcx: &'cx InferCtxt<'cx, 'tcx>) -> SelectionContext<'cx, 'tcx> {
             intercrate: false,
             intercrate_ambiguity_causes: None,
             allow_negative_impls: false,
-            query_mode: TraitQueryMode::Standard,
+            query_mode: TraitQueryMode::Canonical,
         }
     }
 
