diff
diff --git a/.travis.yml b/.travis.yml
index 63831cd596..9789b8eb75 100644
--- a/.travis.yml
+++ b/.travis.yml
@@ -169,7 +169,7 @@ matrix:
     - env: IMAGE=x86_64-gnu-aux
       if: branch = auto
     - env: IMAGE=x86_64-gnu-tools
-      if: branch = auto
+      # if: branch = auto
     - env: IMAGE=x86_64-gnu-debug
       if: branch = auto
     - env: IMAGE=x86_64-gnu-nopt
