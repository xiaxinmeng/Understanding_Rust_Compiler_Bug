 diff
diff --git a/src/libstd/unstable/dynamic_lib.rs b/src/libstd/unstable/dynamic_lib.rs
index 68f0aaa..865b9cb 100644
--- a/src/libstd/unstable/dynamic_lib.rs
+++ b/src/libstd/unstable/dynamic_lib.rs
@@ -73,7 +73,7 @@ impl DynamicLibrary {
             ("LD_LIBRARY_PATH", ':' as u8)
         };
         let newenv = os::getenv_as_bytes(envvar).unwrap_or(box []);
-        let newenv = newenv + &[sep] + path.as_vec();
+        let newenv = path.as_vec() + &[sep] + newenv;
         os::setenv(envvar, str::from_utf8(newenv).unwrap());
     }

