patch
From 34a7c65154e2b49d6a58ec5883a3ee9a40289ab7 Mon Sep 17 00:00:00 2001
From: Aleksa Sarai <cyphar@cyphar.com>
Date: Sat, 6 Jul 2019 00:51:19 +1000
Subject: [PATCH] filedesc: don't use FIOCLEX on Linux

All ioctl(2)s will fail on O_PATH file descriptors on Linux (because
they use &empty_fops as a security measure against O_PATH descriptors
affecting the backing file).

As a result, File::try_clone() and various other methods would always
fail with -EBADF on O_PATH file descriptors. The solution is to simply
use F_SETFD (as is used on other unices) which works on O_PATH
descriptors because it operates through the fnctl(2) layer and not
through ioctl(2)s.

Fixes: rust-lang/rust#62314
Signed-off-by: Aleksa Sarai <cyphar@cyphar.com>
---
 src/libstd/sys/unix/fd.rs | 2 ++
 1 file changed, 2 insertions(+)

diff --git a/src/libstd/sys/unix/fd.rs b/src/libstd/sys/unix/fd.rs
index 6d23963e141a..0cecdd7ffa0b 100644
--- a/src/libstd/sys/unix/fd.rs
+++ b/src/libstd/sys/unix/fd.rs
@@ -175,6 +175,7 @@ impl FileDesc {
                   target_os = "emscripten",
                   target_os = "fuchsia",
                   target_os = "l4re",
+                  target_os = "linux",
                   target_os = "haiku")))]
     pub fn set_cloexec(&self) -> io::Result<()> {
         unsafe {
@@ -187,6 +188,7 @@ impl FileDesc {
               target_os = "emscripten",
               target_os = "fuchsia",
               target_os = "l4re",
+              target_os = "linux",
               target_os = "haiku"))]
     pub fn set_cloexec(&self) -> io::Result<()> {
         unsafe {
-- 
2.22.0

