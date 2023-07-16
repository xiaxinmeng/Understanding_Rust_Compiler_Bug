
From 221f4b10d421dca92ffb42e079c3ad8f2ff55232 Mon Sep 17 00:00:00 2001
From: Jonah Petri <jonah@petri.us>
Date: Tue, 3 Aug 2021 14:14:53 -0400
Subject: [PATCH] uclibc does not pass argv/argc in a glibc-compatible way

---
 library/std/src/sys/unix/args.rs | 4 ++--
 1 file changed, 2 insertions(+), 2 deletions(-)

diff --git a/library/std/src/sys/unix/args.rs b/library/std/src/sys/unix/args.rs
index 2805b2964d1..94364f9c874 100644
--- a/library/std/src/sys/unix/args.rs
+++ b/library/std/src/sys/unix/args.rs
@@ -103,14 +103,14 @@ mod imp {
         // still initialize here.
         #[cfg(any(
             miri,
-            not(all(target_os = "linux", any(target_env = "gnu", target_env = "uclibc")))
+            not(all(target_os = "linux", any(target_env = "gnu")))
         ))]
         really_init(_argc, _argv);
     }
 
     /// glibc passes argc, argv, and envp to functions in .init_array, as a non-standard extension.
     /// This allows `std::env::args` to work even in a `cdylib`, as it does on macOS and Windows.
-    #[cfg(all(target_os = "linux", any(target_env = "gnu", target_env = "uclibc")))]
+    #[cfg(all(target_os = "linux", any(target_env = "gnu")))]
     #[used]
     #[link_section = ".init_array.00099"]
     static ARGV_INIT_ARRAY: extern "C" fn(
-- 
2.32.0

