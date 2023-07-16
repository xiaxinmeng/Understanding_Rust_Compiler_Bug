diff
From 68f2e1ebf2e0f959685fbce3f69349b3e21f34cc Mon Sep 17 00:00:00 2001
From: Alex Crichton <alex@alexcrichton.com>
Date: Thu, 25 Jan 2018 21:04:01 -0800
Subject: [PATCH] Ignore an i128 test on emscripten

---
 src/test/run-pass/issue-38763.rs | 2 ++
 1 file changed, 2 insertions(+)

diff --git a/src/test/run-pass/issue-38763.rs b/src/test/run-pass/issue-38763.rs
index 4bf9513d64f9..01cc8265a399 100644
--- a/src/test/run-pass/issue-38763.rs
+++ b/src/test/run-pass/issue-38763.rs
@@ -8,6 +8,8 @@
 // option. This file may not be copied, modified, or distributed
 // except according to those terms.
 
+// ignore-emscripten
+
 #![feature(i128_type)]
 
 #[repr(C)]
