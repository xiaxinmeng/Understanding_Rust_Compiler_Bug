diff
diff --git a/src/ci/azure-pipelines/try.yml b/src/ci/azure-pipelines/try.yml
index 38a0685e0f7..6244e1413d0 100644
--- a/src/ci/azure-pipelines/try.yml
+++ b/src/ci/azure-pipelines/try.yml
@@ -28,6 +28,8 @@ jobs:
       dist-x86_64-linux: {}
       dist-x86_64-linux-alt:
         IMAGE: dist-x86_64-linux
+      dist-android: {}
+      arm-android: {}
 
 # The macOS and Windows builds here are currently disabled due to them not being
 # overly necessary on `try` builds. We also don't actually have anything that
