 diff
diff --git a/src/libcore/vec.rs b/src/libcore/vec.rs
index 14dcde2..65f8dab 100644
--- a/src/libcore/vec.rs
+++ b/src/libcore/vec.rs
@@ -22,7 +22,7 @@ use old_iter;
 use iterator::Iterator;
 use kinds::Copy;
 use libc;
-use old_iter::{BaseIter, CopyableIter};
+use old_iter::CopyableIter;
 use option::{None, Option, Some};
 use ptr::to_unsafe_ptr;
 use ptr;
