patch
$OpenBSD: patch-src_libstd_sys_unix_os_rs,v 1.10 2019/03/11 19:13:40 semarie Exp $
argv0 isn't suitable as current_exe() in all cases.
Index: src/libstd/sys/unix/os.rs
--- src/libstd/sys/unix/os.rs.orig
+++ src/libstd/sys/unix/os.rs
@@ -273,13 +273,14 @@ pub fn current_exe() -> io::Result<PathBuf> {
         argv.set_len(argv_len as usize);
         if argv[0].is_null() {
             return Err(io::Error::new(io::ErrorKind::Other,
-                                      "no current exe available"))
+                                      "no current exe available (null)"))
         }
         let argv0 = CStr::from_ptr(argv[0]).to_bytes();
         if argv0[0] == b'.' || argv0.iter().any(|b| *b == b'/') {
             ::fs::canonicalize(OsStr::from_bytes(argv0))
         } else {
-            Ok(PathBuf::from(OsStr::from_bytes(argv0)))
+            Err(io::Error::new(io::ErrorKind::Other,
+                               "no current exe available (short)"))
         }
     }
 }
