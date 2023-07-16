patch
--- old/src/std/io/mod.rs
+++ new/src/std/io/mod.rs
@@ -1650,8 +1650,12 @@
     #[stable(feature = "rust1", since = "1.0.0")]
     fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<()> {
+        if let Some(s) = fmt.as_str() {
+            return self.write_all(s.as_bytes());
+        }
+
         // Create a shim which translates a Write to a fmt::Write and saves
         // off I/O errors. instead of discarding them
         struct Adapter<'a, T: ?Sized + 'a> {
             inner: &'a mut T,
             error: Result<()>,
         }
--- old/src/core/fmt/mod.rs
+++ new/src/core/fmt/mod.rs
@@ -186,4 +186,8 @@
     #[stable(feature = "rust1", since = "1.0.0")]
     fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result {
+        if let Some(s) = args.as_str() {
+            self.write_str(s)
+        } else {
-        write(&mut self, args)
+            write(&mut self, args)
+        }
     }
