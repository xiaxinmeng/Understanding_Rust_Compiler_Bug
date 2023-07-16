diff
diff --git a/src/ci/azure-pipelines/try.yml b/src/ci/azure-pipelines/try.yml
index 818306a0092..eea08d1f26a 100644
--- a/src/ci/azure-pipelines/try.yml
+++ b/src/ci/azure-pipelines/try.yml
@@ -26,6 +26,7 @@ jobs:
   strategy:
     matrix:
       dist-x86_64-linux: {}
+      x86_64-gnu-aux: {}
 
 # The macOS and Windows builds here are currently disabled due to them not being
 # overly necessary on `try` builds. We also don't actually have anything that
