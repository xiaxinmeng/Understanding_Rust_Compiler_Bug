diff
diff --git a/src/ci/docker/x86_64-gnu-tools/checkregression.py b/src/ci/docker/x86_64-gnu-tools/checkregression.py
index 8aa90319d6..0cc0a6329e 100755
--- a/src/ci/docker/x86_64-gnu-tools/checkregression.py
+++ b/src/ci/docker/x86_64-gnu-tools/checkregression.py
@@ -21,12 +21,7 @@ if __name__ == '__main__':
         state = cur[os_name]
         new_state = toolstate.get(tool, '')
         if verb == 'regressed':
-            if tool == 'rls':
-                # Temporary override until
-                # https://github.com/rust-lang/rust/issues/60848 is fixed.
-                updated = False
-            else:
-                updated = new_state < state
+            updated = new_state < state
         elif verb == 'changed':
             updated = new_state != state
         else:
