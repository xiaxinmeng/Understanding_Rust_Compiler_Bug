 diff
diff --git a/src/etc/tidy.py b/src/etc/tidy.py
old mode 100644
new mode 100755
index 4815bc6..8f5abd4
--- a/src/etc/tidy.py
+++ b/src/etc/tidy.py
@@ -41,6 +41,7 @@ current_contents = ""

 try:
     for line in fileinput.input(file_names,
+                                mode='rU',
                                 openhook=fileinput.hook_encoded("utf-8")):

         if fileinput.filename().find("tidy.py") == -1:
