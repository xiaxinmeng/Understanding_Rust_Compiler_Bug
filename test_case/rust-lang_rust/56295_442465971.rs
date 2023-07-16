
diff --git a/src/libstd/ffi/c_str.rs b/src/libstd/ffi/c_str.rs
index 0152e9fc2e..2ede59cbd5 100644
--- a/src/libstd/ffi/c_str.rs
+++ b/src/libstd/ffi/c_str.rs
@@ -414,12 +414,14 @@ impl CString {
 
     /// Consumes the `CString` and transfers ownership of the string to a C caller.
     ///
-    /// If the system allocator is in use, it is safe to have C invoke the corresponding
-    /// system-level deallocation function (e.g. `free()` on Unix, `HeapFree` on Windows)
-    /// on the returned pointer.
-    ///
-    /// If you are using a custom allocator, then this value should be returned
-    /// to Rust and reconstituted using [`from_raw`] to be properly deallocated.
+    /// The safe best practice is to then return the pointer back to Rust and invoke [`from_raw`]
+    /// to deallocate it.  Particularly when Rust is configured to use a custom allocator,
+    /// you *must* do this.
+    ///
+    /// If the system allocator is in use (and your code can rely on this), it
+    /// is safe to have C invoke the corresponding system-level deallocation
+    /// function (e.g. `free()` on Unix, `HeapFree` on Windows) on the returned
+    /// pointer.
     ///
     /// As a general rule, best practice is for library crates to not make an assumption
     /// about the global allocator.
