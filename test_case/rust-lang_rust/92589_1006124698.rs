diff
diff --git a/src/bootstrap/bootstrap.py b/src/bootstrap/bootstrap.py
index dc44b27c2a6..496a4dad898 100644
--- a/src/bootstrap/bootstrap.py
+++ b/src/bootstrap/bootstrap.py
@@ -41,7 +41,7 @@ def acquire_lock(build_dir):
             curs = con.cursor()
             while True:
                 try:
-                    curs.execute("BEGIN EXCLUSIVE")
+                    return curs.execute("BEGIN EXCLUSIVE")
                 except sqlite3.OperationalError:
                     pass
             return curs
