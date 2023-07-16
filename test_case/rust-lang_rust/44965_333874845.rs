diff
diff --git a/src/libstd/sys/unix/net.rs b/src/libstd/sys/unix/net.rs
index 94f24889e6..fccf314849 100644
--- a/src/libstd/sys/unix/net.rs
+++ b/src/libstd/sys/unix/net.rs
@@ -375,8 +375,11 @@ impl IntoInner<c_int> for Socket {
 pub fn res_init_if_glibc_before_2_26() -> io::Result<()> {
     // If the version fails to parse, we treat it the same as "not glibc".
     if let Some(Ok(version_str)) = glibc_version_cstr().map(CStr::to_str) {
+        println!("{:?}", version_str);
         if let Some(version) = parse_glibc_version(version_str) {
+            println!("{:?}", version);
             if version < (2, 26) {
+                println!("calling res_init");
                 let ret = unsafe { libc::res_init() };
                 if ret != 0 {
                     return Err(io::Error::last_os_error());
