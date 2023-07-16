diff
diff --git a/src/libstd/time/mod.rs b/src/libstd/time/mod.rs
index 5b893505b3..bd96f2133e 100644
--- a/src/libstd/time/mod.rs
+++ b/src/libstd/time/mod.rs
@@ -498,7 +498,7 @@ mod tests {
                 let dur = dur.duration();
                 assert!(a > b);
                 assert_almost_eq!(b + dur, a);
-                assert_almost_eq!(b - dur, a);
+                assert_almost_eq!(a - dur, b);
             }
