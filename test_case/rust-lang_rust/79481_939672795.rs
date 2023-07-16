diff
diff --git a/library/core/tests/iter/traits/iterator.rs b/library/core/tests/iter/traits/iterator.rs
index 422e389e380..77e1effc764 100644
--- a/library/core/tests/iter/traits/iterator.rs
+++ b/library/core/tests/iter/traits/iterator.rs
@@ -26,6 +26,7 @@ fn cmp(&self, other: &Self) -> core::cmp::Ordering {
 
 #[test]
 fn test_lt() {
+    debug_assert!(false);
     let empty: [isize; 0] = [];
     let xs = [1, 2, 3];
     let ys = [1, 2, 0];

