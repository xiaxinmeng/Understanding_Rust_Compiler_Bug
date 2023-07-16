diff
diff --git a/library/core/src/iter/traits/iterator.rs b/library/core/src/iter/traits/iterator.rs
index a1a336a0574..bdee0e202c6 100644
--- a/library/core/src/iter/traits/iterator.rs
+++ b/library/core/src/iter/traits/iterator.rs
@@ -25,6 +25,13 @@ fn _assert_is_object_safe(_: &dyn Iterator<Item = ()>) {}
 /// [impl]: crate::iter#implementing-iterator
 #[stable(feature = "rust1", since = "1.0.0")]
 #[rustc_on_unimplemented(
+    on(
+        all(
+            any(_Self = "[]", _Self = "std::vec::Vec", _Self = "std::string::String", _Self = "str"),
+            from_method = "count",
+        ),
+        label = "consider using `.len()`"
+    ),
     on(
         _Self = "std::ops::RangeTo<Idx>",
         label = "if you meant to iterate until a value, add a starting value",
