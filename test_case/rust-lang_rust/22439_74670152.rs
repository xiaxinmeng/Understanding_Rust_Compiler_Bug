 patch
From b1f8e58045ec798abcd0d36d93d59f60b0595e52 Mon Sep 17 00:00:00 2001
From: Simonas Kazlauskas <git@kazlauskas.me>
Date: Tue, 17 Feb 2015 15:50:18 +0200
Subject: [PATCH] use ops::Range instead of iter::Range

---
 src/libstd/sys/windows/os.rs | 4 ++--
 1 file changed, 2 insertions(+), 2 deletions(-)

diff --git a/src/libstd/sys/windows/os.rs b/src/libstd/sys/windows/os.rs
index 6aa1ac0..502d70d 100644
--- a/src/libstd/sys/windows/os.rs
+++ b/src/libstd/sys/windows/os.rs
@@ -18,7 +18,7 @@ use os::windows::*;
 use error::Error as StdError;
 use ffi::{OsString, OsStr, AsOsStr};
 use fmt;
-use iter::Range;
+use ops::Range;
 use libc::types::os::arch::extra::LPWCH;
 use libc::{self, c_int, c_void};
 use mem;
@@ -319,7 +319,7 @@ pub fn args() -> Args {
         let lpCmdLine = c::GetCommandLineW();
         let szArgList = c::CommandLineToArgvW(lpCmdLine, &mut nArgs);

-        Args { cur: szArgList, range: range(0, nArgs as isize) }
+        Args { cur: szArgList, range: 0..(nArgs as isize) }
     }
 }

-- 
2.3.0
