
diff --git a/src/libcore/ffi.rs b/src/libcore/ffi.rs
index 3fe65ab344..823d040d7b 100644
--- a/src/libcore/ffi.rs
+++ b/src/libcore/ffi.rs
@@ -198,10 +198,10 @@ impl<'a> VaList<'a> {
                   windows))]
         let mut ap = va_copy(self);
         #[cfg(all(any(target_arch = "aarch64", target_arch = "powerpc", target_arch = "x86_64"),
-                  not(windows)))]
+                  not(windows), not(all(target_arch = "aarch64", target_os = "ios"))))]
         let mut ap_inner = va_copy(self);
         #[cfg(all(any(target_arch = "aarch64", target_arch = "powerpc", target_arch = "x86_64"),
-                  not(windows)))]
+                  not(windows), not(all(target_arch = "aarch64", target_os = "ios"))))]
         let mut ap = VaList(&mut ap_inner);
         let ret = f(VaList(ap.0));
         va_end(&mut ap);
@@ -217,10 +217,11 @@ extern "rust-intrinsic" {
     /// Copy the current location of arglist `src` to the arglist `dst`.
     #[cfg(any(all(not(target_arch = "aarch64"), not(target_arch = "powerpc"),
                   not(target_arch = "x86_64")),
+              all(target_arch = "aarch64", target_os = "ios"),
               windows))]
     fn va_copy<'a>(src: &VaList<'a>) -> VaList<'a>;
     #[cfg(all(any(target_arch = "aarch64", target_arch = "powerpc", target_arch = "x86_64"),
-              not(windows)))]
+              not(windows), not(all(target_arch = "aarch64", target_os = "ios"))))]
     fn va_copy(src: &VaList) -> VaListImpl;

     /// Loads an argument of type `T` from the `va_list` `ap` and increment the
