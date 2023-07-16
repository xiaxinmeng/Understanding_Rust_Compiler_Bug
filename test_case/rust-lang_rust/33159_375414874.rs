patch
diff --git a/src/etc/gdb_load_rust_pretty_printers.py b/src/etc/gdb_load_rust_pretty_printers.py
index 755cac153d..48eba096fc 100644
--- a/src/etc/gdb_load_rust_pretty_printers.py
+++ b/src/etc/gdb_load_rust_pretty_printers.py
@@ -8,5 +8,12 @@
 # option. This file may not be copied, modified, or distributed
 # except according to those terms.
 
+# Hacky fix for paths being annoying
+import sys
+from os import path
+self_dir = path.dirname(path.realpath(__file__))
+sys.path.append(self_dir)
+
 import gdb_rust_pretty_printing
 gdb_rust_pretty_printing.register_printers(gdb.current_objfile())
