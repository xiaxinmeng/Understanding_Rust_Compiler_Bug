diff
-                           -> Box<Future<Item = Option<SysrootEntry>, Error = Never> + Send> {
+                           -> impl Future<Item = Option<SysrootEntry>, Error = Never> + Send {
