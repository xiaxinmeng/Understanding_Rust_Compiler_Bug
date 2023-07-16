diff
diff --git a/gcc/rust/expand/rust-macro-builtins.cc b/gcc/rust/expand/rust-macro-builtins.cc
index f6ddb1a803e..936649d4d87 100644
--- a/gcc/rust/expand/rust-macro-builtins.cc
+++ b/gcc/rust/expand/rust-macro-builtins.cc
@@ -246,9 +246,13 @@ load_file_bytes (const char *filename)
     }
 
   FILE *f = file_wrap.get_raw ();
-  fseek (f, 0L, SEEK_END);
+  auto one = fseek (f, 0L, SEEK_END);
   long fsize = ftell (f);
-  fseek (f, 0L, SEEK_SET);
+  auto two = fseek (f, 0L, SEEK_SET);
+
+  rust_debug ("[ARTHUR]: fseek (f, 0L, SEEK_END) = %d", one);
+  rust_debug ("[ARTHUR]: fsize = %ld", fsize);
+  rust_debug ("[ARTHUR]: fseek (f, 0L, SEEK_SET) = %d", two);
 
   std::vector<uint8_t> buf (fsize);
 