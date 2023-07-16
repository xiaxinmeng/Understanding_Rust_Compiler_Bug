
diff --git a/src/bootstrap/configure.py b/src/bootstrap/configure.py
index f95a97518c5..eaa4c71b4a3 100755
--- a/src/bootstrap/configure.py
+++ b/src/bootstrap/configure.py
@@ -525,7 +525,8 @@ def write_config_toml(writer, section_order, targets, sections):
 
 def quit_if_file_exists(file):
     if os.path.isfile(file):
-        err("Existing '" + file + "' detected.")
+        os.rename(file, file + ".bak")
+        print("Existing '" + file + "' detected.")
 
 if __name__ == "__main__":
     # If 'config.toml' already exists, exit the script at this point
