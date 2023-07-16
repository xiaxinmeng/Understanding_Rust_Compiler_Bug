diff
diff --git a/library/proc_macro/src/bridge/server.rs b/library/proc_macro/src/bridge/server.rs
index e068ec60b6a..a29fae17292 100644
--- a/library/proc_macro/src/bridge/server.rs
+++ b/library/proc_macro/src/bridge/server.rs
@@ -213,6 +213,8 @@ impl<P> ExecutionStrategy for CrossThread<P>
 where
     P: MessagePipe<Buffer> + Send + 'static,
 {
+    #[cold]
+    #[inline(never)]
     fn run_bridge_and_client(
         &self,
         dispatcher: &mut impl DispatcherTrait,
