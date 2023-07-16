diff
diff --git a/Users/fox/Downloads/before_cleanup_i686.json b/Users/fox/Downloads/after_cleanup_i686.json
index e39cb59d694..b4184032496 100644
--- a/Users/fox/Downloads/before_cleanup_i686.json
+++ b/Users/fox/Downloads/after_cleanup_i686.json
@@ -29,9 +29,9 @@
   "os": "macos",
   "pre-link-args": {
     "gcc": [
+      "-m32",
       "-arch",
-      "i386",
-      "-m32"
+      "i386"
     ],
     "ld": [
       "-arch",
