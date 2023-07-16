
--- gcc/rust/rust-diagnostics.h	(original)
+++ gcc/rust/rust-diagnostics.h	(reformatted)
@@ -51,8 +51,7 @@
 // simple location
 extern void
 rust_internal_error_at (const Location, const char *fmt, ...)
-  RUST_ATTRIBUTE_GCC_DIAG (2, 3)
-  RUST_ATTRIBUTE_NORETURN;
+  RUST_ATTRIBUTE_GCC_DIAG (2, 3) RUST_ATTRIBUTE_NORETURN;
 extern void
 rust_error_at (const Location, const char *fmt, ...)
   RUST_ATTRIBUTE_GCC_DIAG (2, 3);
@@ -61,8 +60,7 @@
   RUST_ATTRIBUTE_GCC_DIAG (3, 4);
 extern void
 rust_fatal_error (const Location, const char *fmt, ...)
-  RUST_ATTRIBUTE_GCC_DIAG (2, 3)
-  RUST_ATTRIBUTE_NORETURN;
+  RUST_ATTRIBUTE_GCC_DIAG (2, 3) RUST_ATTRIBUTE_NORETURN;
...
