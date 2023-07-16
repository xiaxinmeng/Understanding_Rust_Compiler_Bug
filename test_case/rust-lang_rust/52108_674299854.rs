diff
--- rust.orig/src/bootstrap/bootstrap.py
+++ rust/src/bootstrap/bootstrap.py
@@ -681,6 +681,8 @@ class RustBuild(object):
         # preserve existing RUSTFLAGS
         env.setdefault("RUSTFLAGS", "")
         env["RUSTFLAGS"] += " -Cdebuginfo=2"
+        if self.build_triple().startswith('mips'):
+            env["RUSTFLAGS"] += " -Ctarget-feature=+xgot"
 
         build_section = "target.{}".format(self.build_triple())
         target_features = []
