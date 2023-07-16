diff
diff --git a/Users/fox/Downloads/before_cleanup_x86_64.json b/Users/fox/Downloads/after_cleanup_x86_64.json
index f06bcb33e37..30c851ceeeb 100644
--- a/Users/fox/Downloads/before_cleanup_x86_64.json
+++ b/Users/fox/Downloads/after_cleanup_x86_64.json
@@ -29,9 +29,9 @@
   "os": "macos",
   "pre-link-args": {
     "gcc": [
+      "-m64",
       "-arch",
-      "x86_64",
-      "-m64"
+      "x86_64"
     ],
     "ld": [
       "-arch",
