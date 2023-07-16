
commit b5f81ba50c5bdcb0a3c0cf8f87f04c8ddd3f5246
Author: Martin Husemann <martin@NetBSD.org>
Date:   Thu Nov 23 10:20:38 2017 +0100

    RUNPATH is a Linuxism, so only use it there.
    RPATH is working fine on most other OSes.

diff --git a/src/librustc_trans/back/rpath.rs b/src/librustc_trans/back/rpath.rs
index 8e5e7d3764..719fa8a58e 100644
--- a/src/librustc_trans/back/rpath.rs
+++ b/src/librustc_trans/back/rpath.rs
@@ -41,7 +41,7 @@ pub fn get_rpath_flags(config: &mut RPathConfig) -> Vec<String> {
     flags.extend_from_slice(&rpaths_to_flags(&rpaths));
 
     // Use DT_RUNPATH instead of DT_RPATH if available
-    if config.linker_is_gnu {
+    if config.linker_is_gnu && target.target_os == "linux" {
         flags.push("-Wl,--enable-new-dtags".to_string());
     }

 