patch
Index: liblumen_alloc/src/lib.rs
IDEA additional info:
Subsystem: com.intellij.openapi.diff.impl.patch.CharsetEP
<+>UTF-8
===================================================================
diff --git a/liblumen_alloc/src/lib.rs b/liblumen_alloc/src/lib.rs
--- a/liblumen_alloc/src/lib.rs	(revision 614cd157dfa302e8235c0957b6fe0b5b8f59332e)
+++ b/liblumen_alloc/src/lib.rs	(date 1612127949750)
@@ -55,10 +55,6 @@
 
 /// The system allocator. Can be used with `#[global_allocator]`, like so:
 ///
-/// 