diff
diff --git a/library/std/src/sys/unix/args.rs b/library/std/src/sys/unix/args.rs
index a342f0f5e85..4231f0b4070 100644
--- a/library/std/src/sys/unix/args.rs
+++ b/library/std/src/sys/unix/args.rs
@@ -142,9 +142,16 @@ fn clone() -> Vec<OsString> {
             let argv = ARGV.load(Ordering::Relaxed);
             let argc = if argv.is_null() { 0 } else { ARGC.load(Ordering::Relaxed) };
             (0..argc)
-                .map(|i| {
-                    let cstr = CStr::from_ptr(*argv.offset(i) as *const libc::c_char);
-                    OsStringExt::from_vec(cstr.to_bytes().to_vec())
+                .filter_map(|i| {
+                    let ptr = *argv.offset(i) as *const libc::c_char;
+                    // Some C commandline parsers are replacing already handled arguments in `argv`
+                    // with `NULL` so it's necessary to skip over them here.
+                    if ptr.is_null() {
+                        None
+                    } else {
+                        let cstr = CStr::from_ptr(ptr);
+                        Some(OsStringExt::from_vec(cstr.to_bytes().to_vec()))
+                    }
                 })
                 .collect()
         }
