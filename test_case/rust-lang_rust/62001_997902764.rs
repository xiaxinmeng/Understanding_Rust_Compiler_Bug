
--- a/src/bootstrap/bootstrap.py
+++ b/src/bootstrap/bootstrap.py
@@ -793,7 +793,7 @@
             if "LIBRARY_PATH" in env else ""
         # preserve existing RUSTFLAGS
         env.setdefault("RUSTFLAGS", "")
-        env["RUSTFLAGS"] += " -Cdebuginfo=2"
+        env["RUSTFLAGS"] += " -Cdebuginfo=0"

         build_section = "target.{}".format(self.build)
         target_features = []
