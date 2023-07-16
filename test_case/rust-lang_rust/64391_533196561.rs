diff
diff --git a/tokio-postgres/src/lib.rs b/tokio-postgres/src/lib.rs
index 29f378a..98c245f 100644
--- a/tokio-postgres/src/lib.rs
+++ b/tokio-postgres/src/lib.rs
@@ -168,10 +168,7 @@ where
     T: MakeTlsConnect<Socket>,
 {
     let config = config.parse::<Config>()?;
-    // FIXME https://github.com/rust-lang/rust/issues/64391
-    async move {
-        config.connect(tls).await
-    }.await
+    config.connect(tls).await
 }

 /// An asynchronous notification.
