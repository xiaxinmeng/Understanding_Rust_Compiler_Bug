diff
diff --git a/src/bootstrap/builder/tests.rs b/src/bootstrap/builder/tests.rs
index 3574f11189e..c9daa0a3b9a 100644
--- a/src/bootstrap/builder/tests.rs
+++ b/src/bootstrap/builder/tests.rs
@@ -501,15 +501,13 @@ fn build_all() {
                 std!(A => C, stage = 2),
             ]
         );
-        assert_eq!(builder.cache.all::<compile::Assemble>().len(), 5);
+        assert_eq!(builder.cache.all::<compile::Assemble>().len(), 4);
         assert_eq!(
             first(builder.cache.all::<compile::Rustc>()),
             &[
                 rustc!(A => A, stage = 0),
                 rustc!(A => A, stage = 1),
-                rustc!(A => A, stage = 2),
                 rustc!(A => B, stage = 1),
-                rustc!(A => B, stage = 2),
             ]
         );
     }
