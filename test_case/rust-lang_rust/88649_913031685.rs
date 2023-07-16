diff
diff --git a/mir_dump-old/foo.main.-------.mir_map.0.mir b/mir_dump-new/foo.main.-------.mir_map.0.mir
index 468567de..4f94cb3e 100644
--- a/mir_dump-old/foo.main.-------.mir_map.0.mir
+++ b/mir_dump-new/foo.main.-------.mir_map.0.mir
@@ -53,6 +53,7 @@ fn main() -> () {
     bb7: {
         _2 = const ();                   // scope 1 at foo.rs:9:57: 9:59
         StorageDead(_6);                 // scope 0 at foo.rs:9:58: 9:59
+        StorageDead(_9);                 // scope 0 at foo.rs:9:58: 9:59
         StorageDead(_7);                 // scope 0 at foo.rs:9:58: 9:59
         goto -> bb16;                    // scope 0 at foo.rs:9:58: 9:59
     }
@@ -67,7 +68,6 @@ fn main() -> () {
     }
 
     bb9: {
-        StorageDead(_8);                 // scope 0 at foo.rs:9:58: 9:59
         FakeRead(ForMatchGuard, _5);     // scope 0 at foo.rs:9:52: 9:53
         FakeRead(ForGuardBinding, _7);   // scope 0 at foo.rs:9:52: 9:53
         StorageLive(_6);                 // scope 0 at foo.rs:9:27: 9:28
@@ -76,12 +76,12 @@ fn main() -> () {
     }
 
     bb10: {
-        StorageDead(_8);                 // scope 0 at foo.rs:9:58: 9:59
-        StorageDead(_7);                 // scope 0 at foo.rs:9:58: 9:59
         goto -> bb11;                    // scope 0 at foo.rs:9:52: 9:53
     }
 
     bb11: {
+        StorageDead(_8);                 // scope 0 at foo.rs:9:58: 9:59
+        StorageDead(_7);                 // scope 0 at foo.rs:9:58: 9:59
         falseEdge -> [real: bb4, imaginary: bb5]; // scope 0 at foo.rs:9:52: 9:53
     }
 
@@ -95,7 +95,6 @@ fn main() -> () {
     }
 
     bb13: {
-        StorageDead(_9);                 // scope 0 at foo.rs:9:58: 9:59
         FakeRead(ForMatchGuard, _5);     // scope 0 at foo.rs:9:52: 9:53
         FakeRead(ForGuardBinding, _7);   // scope 0 at foo.rs:9:52: 9:53
         StorageLive(_6);                 // scope 0 at foo.rs:9:46: 9:47
@@ -104,12 +103,12 @@ fn main() -> () {
     }
 
     bb14: {
-        StorageDead(_9);                 // scope 0 at foo.rs:9:58: 9:59
-        StorageDead(_7);                 // scope 0 at foo.rs:9:58: 9:59
         goto -> bb15;                    // scope 0 at foo.rs:9:52: 9:53
     }
 
     bb15: {
+        StorageDead(_9);                 // scope 0 at foo.rs:9:58: 9:59
+        StorageDead(_7);                 // scope 0 at foo.rs:9:58: 9:59
         falseEdge -> [real: bb6, imaginary: bb4]; // scope 0 at foo.rs:9:52: 9:53
     }
 