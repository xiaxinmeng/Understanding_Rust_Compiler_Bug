diff
diff --git a/.travis.yml b/.travis.yml
index 7985b6c0e1..df1182345b 100644
--- a/.travis.yml
+++ b/.travis.yml
@@ -121,7 +121,7 @@ matrix:
     - env: IMAGE=armhf-gnu
       if: branch = auto
     - env: IMAGE=dist-various-1 DEPLOY=1
-      if: branch = auto
+      if: branch = try
     - env: IMAGE=dist-various-2 DEPLOY=1
       if: branch = auto
     - env: IMAGE=dist-aarch64-linux DEPLOY=1
