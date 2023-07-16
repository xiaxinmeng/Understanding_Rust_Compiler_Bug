 diff
diff --git a/src/libstd/sys/unix/os.rs b/src/libstd/sys/unix/os.rs
index bc4c22a..6b8dfa5 100644
--- a/src/libstd/sys/unix/os.rs
+++ b/src/libstd/sys/unix/os.rs
@@ -400,7 +400,7 @@ pub fn args() -> Args {
         }
     }

-    Args { iter: res.into_iter(), _dont_send_or_sync_me: ptr::null_mut() }
+    Args { iter: res.into_iter(), _dont_send_or_sync_me: PhantomData }
 }

 #[cfg(any(target_os = "linux",
@@ -419,7 +419,7 @@ pub fn args() -> Args {
     let v: Vec<OsString> = bytes.into_iter().map(|v| {
         OsStringExt::from_vec(v)
     }).collect();
-    Args { iter: v.into_iter(), _dont_send_or_sync_me: ptr::null_mut() }
+    Args { iter: v.into_iter(), _dont_send_or_sync_me: PhantomData }
 }

 pub struct Env {
