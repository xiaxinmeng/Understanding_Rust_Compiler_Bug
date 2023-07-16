
diff --git a/src/libcore/array.rs b/src/libcore/array.rs
index e8f6e31..fc80dcb 100644
--- a/src/libcore/array.rs
+++ b/src/libcore/array.rs
@@ -14,6 +14,8 @@

 #![unstable(feature = "core")] // not yet reviewed

+#![doc(primitive = "array")]
+
 use clone::Clone;
 use cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
 use fmt;
