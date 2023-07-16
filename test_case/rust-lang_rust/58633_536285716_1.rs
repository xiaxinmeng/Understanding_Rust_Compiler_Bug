diff
diff --git a/src/libsyntax/feature_gate/builtin_attrs.rs b/src/libsyntax/feature_gate/builtin_attrs.rs
index d14afc6deaa..2d92c4df399 100644
--- a/src/libsyntax/feature_gate/builtin_attrs.rs
+++ b/src/libsyntax/feature_gate/builtin_attrs.rs
@@ -331,7 +331,7 @@ pub const BUILTIN_ATTRIBUTES: &[BuiltinAttribute] = &[
         "the `#[rustc_const_unstable]` attribute is an internal feature",
     ),
     gated!(
-        allow_internal_unstable, Normal, template!(Word, List: "feat1, feat2, ..."),
+        allow_internal_unstable, Whitelisted, template!(Word, List: "feat1, feat2, ..."),
         EXPLAIN_ALLOW_INTERNAL_UNSTABLE,
     ),
     gated!(allow_internal_unsafe, Normal, template!(Word), EXPLAIN_ALLOW_INTERNAL_UNSAFE),
@@ -438,6 +438,11 @@ pub const BUILTIN_ATTRIBUTES: &[BuiltinAttribute] = &[
     // Internal attributes, Layout related:
     // ==========================================================================
 
+//    rustc_attr!(
+//        allow_internal_unstable, Whitelisted, template!(Word, List: "feat1, feat2, ..."),
+//        EXPLAIN_ALLOW_INTERNAL_UNSTABLE,
+//    ),
+
     rustc_attr!(
         rustc_layout_scalar_valid_range_start, Whitelisted, template!(List: "value"),
         "the `#[rustc_layout_scalar_valid_range_start]` attribute is just used to enable \
