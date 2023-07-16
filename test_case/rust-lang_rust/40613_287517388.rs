diff
--- a/src/auto/application.rs
+++ b/src/auto/application.rs
@@ -97,8 +97,9 @@ impl Application {
     pub fn inhibit<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(&self, window: Q, flags: ApplicationInhibitFlags, reason: R) -> u32 {
         unsafe {
             let window_i = window.into();
+            let window_s = window_i.to_glib_none();
             let reason = reason.into();
-            ffi::gtk_application_inhibit(self.to_glib_none().0, window_i.to_glib_none().0, flags.to_glib(), reason.to_glib_none().0)
+            ffi::gtk_application_inhibit(self.to_glib_none().0, window_s.0, flags.to_glib(), reason.to_glib_none().0)
         }
     }
