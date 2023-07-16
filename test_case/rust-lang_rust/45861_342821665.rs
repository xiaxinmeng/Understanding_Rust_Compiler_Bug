diff
diff --git a/.travis.yml b/.travis.yml
index db34f14044..a28954761b 100644
--- a/.travis.yml
+++ b/.travis.yml
@@ -161,16 +161,16 @@ matrix:
       if: branch = auto
     - env: IMAGE=i686-gnu-nopt
       if: branch = auto
-    # - env: IMAGE=wasm32 issue 42646
-    #   if: branch = auto
+    - env: IMAGE=wasm32
+      if: branch = auto
     - env: IMAGE=x86_64-gnu
       if: branch = auto
     - env: IMAGE=x86_64-gnu-full-bootstrap
       if: branch = auto
+    - env: IMAGE=x86_64-gnu-tools
+      if: branch IN (master, beta, stable) # (<- uncomment to disable PR test) AND type != pull_request
     - env: IMAGE=x86_64-gnu-aux
       if: branch = auto
-    - env: IMAGE=x86_64-gnu-cargotest
-      if: branch = auto
     - env: IMAGE=x86_64-gnu-debug
       if: branch = auto
     - env: IMAGE=x86_64-gnu-nopt
