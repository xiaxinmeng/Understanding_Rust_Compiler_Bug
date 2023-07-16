diff
diff --git a/old.txt b/new.txt
index e22f6b1433f..b9fd6b98937 100644
--- a/old.txt
+++ b/new.txt
@@ -6,10 +6,10 @@ error[E0433]: failed to resolve: use of undeclared type `NonZeroU32`
   |
 help: consider importing one of these items
   |
-6 | use core::num::NonZeroU32;
-  |
 6 | use std::num::NonZeroU32;
   |
+6 | use core::num::NonZeroU32;
+  |
 
 error: aborting due to previous error
