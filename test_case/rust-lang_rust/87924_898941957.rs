diff
diff --git i/src/main.rs w/src/main.rs
index a3b18d4..62d8001 100644
--- i/src/main.rs
+++ w/src/main.rs
@@ -16,11 +16,14 @@ async fn main() {
                     route("/create", post(handler))
                         .route("/login", post(handler))
                         .route("/info", get(handler))
-                        .route("/update_password", get(handler)),
+                        .route("/update_password", get(handler))
+                        .boxed(),
                 )
                 .nest(
                     "/product",
-                    route("/list", get(handler)).route("/detail", get(handler)),
+                    route("/list", get(handler))
+                        .route("/detail", get(handler))
+                        .boxed(),
                 ),
         )
         .layer(
