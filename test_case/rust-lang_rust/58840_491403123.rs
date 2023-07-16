patch
diff --git a/src/deps.rs b/src/deps.rs
index d5ad25a..a10b259 100644
--- a/src/deps.rs
+++ b/src/deps.rs
@@ -48,6 +48,7 @@ where
     {
         let requests: Vec<JuniperGraphQLRequest<S>> = vec![];
         future::join_all(requests.into_iter().map(move |request| {
+            let root_node = root_node.clone();
             future::poll_fn(move || {
                 let _res = request.execute(&root_node);
                 Ok(Async::Ready(()))
