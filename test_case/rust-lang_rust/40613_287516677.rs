diff
--- a/src/auto/application.rs
+++ b/src/auto/application.rs
@@ -94,9 +94,11 @@ impl Application {
         }
     }
 
-    pub fn inhibit<'a, T: IsA<Window>, U: Into<Option<&'a str>>>(&self, window: Option<&T>, flags: ApplicationInhibitFlags, reason: U) -> u32 {
+    pub fn inhibit<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(&self, window: Q, flags: ApplicationInhibitFlags, reason: R) -> u32 {
         unsafe {
-            ffi::gtk_application_inhibit(self.to_glib_none().0, window.to_glib_none().0, flags.to_glib(), reason.into().to_glib_none().0)
+            let window_i = window.into();
+            let reason = reason.into();
+            ffi::gtk_application_inhibit(self.to_glib_none().0, window_i.to_glib_none().0, flags.to_glib(), reason.to_glib_none().0)
         }
     }
 