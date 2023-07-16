diff
diff --git a/linkerd/app/inbound/src/http/router.rs b/linkerd/app/inbound/src/http/router.rs
index 239123488..7ed8132aa 100644
--- a/linkerd/app/inbound/src/http/router.rs
+++ b/linkerd/app/inbound/src/http/router.rs
@@ -116,6 +116,7 @@ impl<C> Inbound<C> {
                     rt.span_sink.clone(),
                     super::trace_labels(),
                 ))
+                .push_on_service(svc::BoxService::layer())
                 .push_on_service(http::BoxResponse::layer());
 
             // Attempts to discover a service profile for each logical target (as
