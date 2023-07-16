diff
diff --git a/compiler/rustc_error_messages/src/lib.rs b/compiler/rustc_error_messages/src/lib.rs
index 1882d4b698e..32aa90afa9b 100644
--- a/compiler/rustc_error_messages/src/lib.rs
+++ b/compiler/rustc_error_messages/src/lib.rs
@@ -176,6 +176,10 @@ pub fn fluent_bundle(

     let fallback_locale = langid!("en-US");
     let requested_fallback_locale = requested_locale.as_ref() == Some(&fallback_locale);
+    trace!(?requested_fallback_locale);
+    if requested_fallback_locale && additional_ftl_path.is_none() {
+        return Ok(None);
+    }

     // If there is only `-Z additional-ftl-path`, assume locale is "en-US", otherwise use user
     // provided locale.
@@ -194,7 +198,7 @@ pub fn fluent_bundle(
     bundle.set_use_isolating(with_directionality_markers);

     // If the user requests the default locale then don't try to load anything.
-    if !requested_fallback_locale && let Some(requested_locale) = requested_locale {
+    if let Some(requested_locale) = requested_locale {
         let mut found_resources = false;
         for sysroot in user_provided_sysroot.iter_mut().chain(sysroot_candidates.iter_mut()) {
             sysroot.push("share");

