diff
diff --git a/tokio-postgres/src/lib.rs b/tokio-postgres/src/lib.rs
index 98c245f..eaf43f1 100644
--- a/tokio-postgres/src/lib.rs
+++ b/tokio-postgres/src/lib.rs
@@ -168,7 +168,9 @@ where
     T: MakeTlsConnect<Socket>,
 {
     let config = config.parse::<Config>()?;
-    config.connect(tls).await
+    async move {
+        config.connect(tls).await
+    }.await
 }
