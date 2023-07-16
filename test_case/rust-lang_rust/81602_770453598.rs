patchIndex: liblumen_alloc/src/erts/process/monitor.rs
IDEA additional info:
Subsystem: com.intellij.openapi.diff.impl.patch.CharsetEP
<+>UTF-8
===================================================================
diff --git a/liblumen_alloc/src/erts/process/monitor.rs b/liblumen_alloc/src/erts/process/monitor.rs
--- a/liblumen_alloc/src/erts/process/monitor.rs	(revision 614cd157dfa302e8235c0957b6fe0b5b8f59332e)
+++ b/liblumen_alloc/src/erts/process/monitor.rs	(date 1612120689957)
@@ -7,43 +7,9 @@
     /// When monitoring a name, it does not matter if the name change after the monitor, the name
     /// passed to monitor is always returned, but the node name reflects the current node name.
     ///
-    /// 