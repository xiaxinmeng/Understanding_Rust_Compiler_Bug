diff
Index: rustc-1.27.1+dfsg1/src/bootstrap/bootstrap.py
===================================================================
--- rustc-1.27.1+dfsg1.orig/src/bootstrap/bootstrap.py
+++ rustc-1.27.1+dfsg1/src/bootstrap/bootstrap.py
@@ -591,6 +591,8 @@ class RustBuild(object):
             (os.pathsep + env["LIBRARY_PATH"]) \
             if "LIBRARY_PATH" in env else ""
         env["RUSTFLAGS"] = "-Cdebuginfo=2"
+        if self.build_triple().startswith('mips'):
+            env["RUSTFLAGS"] += " -Cllvm-args=-mxgot"
         env["PATH"] = os.path.join(self.bin_root(), "bin") + \
             os.pathsep + env["PATH"]
         if not os.path.isfile(self.cargo()):
