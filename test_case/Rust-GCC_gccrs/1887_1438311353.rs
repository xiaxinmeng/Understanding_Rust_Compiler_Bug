diff
+// See <https://github.com/Rust-GCC/gccrs/issues/1887> "Non-conforming `printf` prototypes in the GCC/Rust testsuite".
+pub type c_char = i8;
+pub type c_int = i32;
 extern "C" {
-   fn printf(s: *const i8, ...);
+   fn printf(format: *const c_char, ...) -> c_int;
 }
