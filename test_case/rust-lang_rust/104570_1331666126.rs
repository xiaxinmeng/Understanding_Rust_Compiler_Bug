diff
diff --git a/Users/fox/Downloads/before_cleanup_aarch64.json b/Users/fox/Downloads/after_cleanup_aarch64.json
index 865bfa43837..41854d7f020 100644
--- a/Users/fox/Downloads/before_cleanup_aarch64.json
+++ b/Users/fox/Downloads/after_cleanup_aarch64.json
@@ -29,6 +29,7 @@
   "os": "macos",
   "pre-link-args": {
     "gcc": [
+      "-m64",
       "-arch",
       "arm64"
     ],
