
From 7d649f37476f533de1bbd5c964350e7a9cc93ac3 Mon Sep 17 00:00:00 2001
From: Evan Martin <martine@danga.com>
Date: Wed, 20 Feb 2013 10:46:51 -0800
Subject: [PATCH] ReaderUtil::each_byte shouldn't include EOF byte -- Issue
 #5056

---
 src/libcore/io.rs | 18 ++++++++++++++++--
 1 file changed, 16 insertions(+), 2 deletions(-)

diff --git a/src/libcore/io.rs b/src/libcore/io.rs
index 2173efe..84877e7 100644
--- a/src/libcore/io.rs
+++ b/src/libcore/io.rs
@@ -277,12 +277,16 @@ impl<T: Reader> ReaderUtil for T {
     }

     fn each_byte(&self, it: fn(int) -> bool) {
-        while !self.eof() {
-            if !it(self.read_byte()) { break; }
+        loop {
+            match self.read_byte() {
+                -1 => break,
+                ch => if !it(ch) { break; }
+            }
         }
     }

     fn each_char(&self, it: fn(char) -> bool) {
+        // FIXME this doesn't handle EOF properly -- see issue #5056
         while !self.eof() {
             if !it(self.read_char()) { break; }
         }
@@ -1211,6 +1215,16 @@ mod tests {
     }

     #[test]
+    fn test_each_byte_empty() {
+        // Issue #5056 -- shouldn't include trailing EOF.
+        do io::with_str_reader(~"") |inp| {
+            for inp.each_byte() |b| {
+                assert(b != -1);
+            }
+        }
+    }
+
+    #[test]
     fn test_readchars_empty() {
         do io::with_str_reader(~"") |inp| {
             let res : ~[char] = inp.read_chars(128);
-- 
1.8.1.3
