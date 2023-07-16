diff
diff --git a/src/libstd/sys/unix/rand.rs b/src/libstd/sys/unix/rand.rs
index 01c0ada4ff..967a517442 100644
--- a/src/libstd/sys/unix/rand.rs
+++ b/src/libstd/sys/unix/rand.rs
@@ -77,7 +77,7 @@ mod imp {
             let result = getrandom(&mut buf);
             let available = if result == -1 {
                 let err = io::Error::last_os_error().raw_os_error();
-                err != Some(libc::ENOSYS)
+                err != Some(libc::ENOSYS) && err != Some(libc::EPERM)
             } else {
                 true
             };
