 diff
diff --git a/src/bootstrap/bootstrap.py b/src/bootstrap/bootstrap.py
index 2c2260a..056bd36 100644
--- a/src/bootstrap/bootstrap.py
+++ b/src/bootstrap/bootstrap.py
@@ -370,7 +370,7 @@ def main():
     rb.config_toml = ''
     rb.config_mk = ''
     rb.rust_root = os.path.abspath(os.path.join(__file__, '../../..'))
-    rb.build_dir = os.path.join(os.getcwd(), "build")
+    rb.build_dir = os.path.join(rb.rust_root, "build")
     rb.verbose = args.verbose
     rb.clean = args.clean
