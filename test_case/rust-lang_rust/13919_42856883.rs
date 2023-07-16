 diff
diff --git a/src/liblibc/lib.rs b/src/liblibc/lib.rs
index e2b647f..446f753 100644
--- a/src/liblibc/lib.rs
+++ b/src/liblibc/lib.rs
@@ -431,8 +431,17 @@ pub mod types {
                     pub ai_socktype: c_int,
                     pub ai_protocol: c_int,
                     pub ai_addrlen: socklen_t,
+
+                    #[cfg(target_os = "linux")]
                     pub ai_addr: *sockaddr,
+                    #[cfg(target_os = "linux")]
+                    pub ai_canonname: *c_char,
+
+                    #[cfg(target_os = "android")]
                     pub ai_canonname: *c_char,
+                    #[cfg(target_os = "android")]
+                    pub ai_addr: *sockaddr,
+
                     pub ai_next: *addrinfo,
                 }
                 pub struct sockaddr_un {
