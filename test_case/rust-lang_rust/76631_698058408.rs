python
diff --git a/src/bootstrap/configure.py b/src/bootstrap/configure.py
index 47673ce1e87..ac213b55873 100755
--- a/src/bootstrap/configure.py
+++ b/src/bootstrap/configure.py
@@ -34,6 +34,8 @@ def v(*args):
 o("debug", "rust.debug", "enables debugging environment; does not affect optimization of bootstrapped code (use `--disable-optimize` for that)")
 o("docs", "build.docs", "build standard library documentation")
 o("compiler-docs", "build.compiler-docs", "build compiler documentation")
+v("profile", "profile", "the latest version of the x.py changelog viewed")
 o("optimize-tests", "rust.optimize-tests", "build tests with optimizations")
 o("parallel-compiler", "rust.parallel-compiler", "build a multi-threaded rustc")
 o("verbose-tests", "rust.verbose-tests", "enable verbose output when running tests")
@@ -279,6 +281,7 @@ def set(key, value):
     arr = config
     parts = key.split('.')
     for i, part in enumerate(parts):
+        print(i, part)
         if i == len(parts) - 1:
             arr[part] = value
         else:
@@ -362,6 +365,7 @@ set('build.configure-args', sys.argv[1:])
 sections = {}
 cur_section = None
 sections[None] = []
+sections[0] = ["profile"]
 section_order = [None]
 targets = {}
 
@@ -392,6 +396,8 @@ for target in configured_targets:
     targets[target] = sections['target'][:]
     targets[target][0] = targets[target][0].replace("x86_64-unknown-linux-gnu", target)
 
+# Set the default profile to 'user', since contributors won't be using `configure`
+set("profile", "user")
 
 def is_number(value):
     try:
