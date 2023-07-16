patch
Subject: [PATCH] Add MaybeUninit array transpose impls

Signed-off-by: Alex Saveau <saveau.alexandre@gmail.com>
---
Index: library/core/src/mem/maybe_uninit.rs
IDEA additional info:
Subsystem: com.intellij.openapi.diff.impl.patch.CharsetEP
<+>UTF-8
===================================================================
diff --git a/library/core/src/mem/maybe_uninit.rs b/library/core/src/mem/maybe_uninit.rs
--- a/library/core/src/mem/maybe_uninit.rs	(revision 8147e6e427a1b3c4aedcd9fd85bd457888f80972)
+++ b/library/core/src/mem/maybe_uninit.rs	(date 1665873934720)
@@ -117,15 +117,12 @@
 /// `MaybeUninit<T>` can be used to initialize a large array element-by-element:
 ///
 /// 