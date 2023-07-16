diff
--- a.main.005-007.CopyPropagation.before.mir	2021-06-15 10:42:21.821538556 +0200
+++ a.main.005-007.CopyPropagation.after.mir	2021-06-15 10:42:21.821538556 +0200
@@ -1,37 +1,37 @@
-// MIR for `main` before CopyPropagation
+// MIR for `main` after CopyPropagation
 
 fn main() -> () {
     let mut _0: ();                      // return place in scope 0 at a.rs:1:11: 1:11
     let _1: [u8; 128];                   // in scope 0 at a.rs:2:9: 2:10
     let mut _2: !;                       // in scope 0 at a.rs:3:5: 5:6
     let mut _3: ();                      // in scope 0 at a.rs:1:1: 6:2
     let _4: ();                          // in scope 0 at a.rs:4:9: 4:13
     let mut _5: [u8; 128];               // in scope 0 at a.rs:4:11: 4:12
     scope 1 {
         debug a => _1;                   // in scope 1 at a.rs:2:9: 2:10
     }
 
     bb0: {
-        StorageLive(_1);                 // scope 0 at a.rs:2:9: 2:10
+        nop;                             // scope 0 at a.rs:2:9: 2:10
         _1 = [const 0_u8; 128];          // scope 0 at a.rs:2:13: 2:21
         StorageLive(_2);                 // scope 1 at a.rs:3:5: 5:6
         goto -> bb1;                     // scope 1 at a.rs:3:5: 5:6
     }
 
     bb1: {
         StorageLive(_4);                 // scope 1 at a.rs:4:9: 4:13
-        StorageLive(_5);                 // scope 1 at a.rs:4:11: 4:12
-        _5 = _1;                         // scope 1 at a.rs:4:11: 4:12
-        _4 = f(move _5) -> bb2;          // scope 1 at a.rs:4:9: 4:13
+        nop;                             // scope 1 at a.rs:4:11: 4:12
+        nop;                             // scope 1 at a.rs:4:11: 4:12
+        _4 = f(move _1) -> bb2;          // scope 1 at a.rs:4:9: 4:13
                                          // mir::Constant
                                          // + span: a.rs:4:9: 4:10
                                          // + literal: Const { ty: fn([u8; 128]) {f}, val: Value(Scalar(<ZST>)) }
     }
 
     bb2: {
-        StorageDead(_5);                 // scope 1 at a.rs:4:12: 4:13
+        nop;                             // scope 1 at a.rs:4:12: 4:13
         StorageDead(_4);                 // scope 1 at a.rs:4:13: 4:14
         _3 = const ();                   // scope 1 at a.rs:3:10: 5:6
         goto -> bb1;                     // scope 1 at a.rs:3:5: 5:6
     }
 }
