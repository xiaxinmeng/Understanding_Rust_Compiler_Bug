diff
diff --git a/.travis.yml b/.travis.yml
index 9e62b895ed..dbfe2739e9 100644
--- a/.travis.yml
+++ b/.travis.yml
@@ -109,7 +109,7 @@ matrix:
 
     # Linux builders, remaining docker images
     - env: IMAGE=arm-android
-      if: branch = auto
+      # if: branch = auto
     - env: IMAGE=armhf-gnu
       if: branch = auto
     - env: IMAGE=dist-various-1 DEPLOY=1
