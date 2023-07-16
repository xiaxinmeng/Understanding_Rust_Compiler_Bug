 diff
diff --git a/src/libstd/process.rs b/src/libstd/process.rs
index be921d9..3d5090f 100644
--- a/src/libstd/process.rs
+++ b/src/libstd/process.rs
@@ -21,6 +21,7 @@ use fmt;
 use io::{self, Error, ErrorKind};
 use path;
 use sync::mpsc::{channel, Receiver};
+use sync::Once;
 use sys::pipe::{self, AnonPipe};
 use sys::process as imp;
 use sys_common::{AsInner, AsInnerMut, FromInner, IntoInner};
@@ -582,6 +583,8 @@ impl Child {
 /// to run.
 #[stable(feature = "rust1", since = "1.0.0")]
 pub fn exit(code: i32) -> ! {
+    static CLEANUP: Once = Once::new();
+    CLEANUP.call_once(|| unsafe { ::rt::cleanup() });
     ::sys::os::exit(code)
 }
