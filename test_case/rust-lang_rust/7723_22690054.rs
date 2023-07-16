
--- a/src/etc/licenseck.py
+++ b/src/etc/licenseck.py
@@ -80,6 +80,7 @@ exceptions = [

 def check_license(name, contents):
     valid_license = False
+    contents = contents.replace("\r\n", "\n")
     for a_valid_license in licenses:
         if contents.startswith(a_valid_license):
             valid_license = True
