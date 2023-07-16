
 #[allow(unused)]
 #[inline(always)]
 pub(super) fn is_bit_set(byte: u8, bit: u8) -> bool {
Index: liblumen_alloc/src/erts/term/atom.rs
IDEA additional info:
Subsystem: com.intellij.openapi.diff.impl.patch.CharsetEP
<+>UTF-8
===================================================================
diff --git a/liblumen_alloc/src/erts/term/atom.rs b/liblumen_alloc/src/erts/term/atom.rs
--- a/liblumen_alloc/src/erts/term/atom.rs	(revision 614cd157dfa302e8235c0957b6fe0b5b8f59332e)
+++ b/liblumen_alloc/src/erts/term/atom.rs	(date 1612121050549)
@@ -173,7 +173,6 @@
     /// with an id that doesn't exist will result in undefined
     /// behavior. This should only be used by `Term` when converting
     /// to `TypedTerm`
-    /// 