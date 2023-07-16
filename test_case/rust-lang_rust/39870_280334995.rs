diff
diff --git a/Library/Homebrew/extend/os/mac/keg_relocate.rb b/Library/Homebrew/extend/os/mac/keg_relocate.rb
index f44a97b31..476e5da4a 100644
--- a/Library/Homebrew/extend/os/mac/keg_relocate.rb
+++ b/Library/Homebrew/extend/os/mac/keg_relocate.rb
@@ -78,13 +78,19 @@ class Keg
     end
   end
 
+  def filename_contains_metavariable?(fn)
+    fn =~ /^@(loader_|executable_|r)path/
+  end
+
   def each_install_name_for(file, &block)
     dylibs = file.dynamically_linked_libraries
-    dylibs.reject! { |fn| fn =~ /^@(loader_|executable_|r)path/ }
+    dylibs.reject! { |fn| filename_contains_metavariable?(fn) }
     dylibs.each(&block)
   end
 
   def dylib_id_for(file)
+    return file.dylib_id if filename_contains_metavariable?(file.dylib_id)
+
     # The new dylib ID should have the same basename as the old dylib ID, not
     # the basename of the file itself.
     basename = File.basename(file.dylib_id)
