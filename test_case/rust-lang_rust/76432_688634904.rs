diff
--- dead.main-test.005-004.SimplifyComparisonIntegral.before.mir 
+++ dead.main-test.005-004.SimplifyComparisonIntegral.after.mir
@@ -1,4 +1,4 @@
-// MIR for `test` before SimplifyComparisonIntegral
+// MIR for `test` after SimplifyComparisonIntegral
 
 fn test(_1: T) -> () {
     debug x => _1;                       // in scope 0 at dead.rs:4:43: 4:44
@@ -56,11 +56,12 @@
         StorageLive(_9);                 // scope 1 at dead.rs:6:9: 9:10
         _10 = Len((*_2));                // scope 1 at dead.rs:7:13: 7:37
         _11 = const 3_usize;             // scope 1 at dead.rs:7:13: 7:37
-        _12 = Eq(move _10, const 3_usize); // scope 1 at dead.rs:7:13: 7:37
-        switchInt(move _12) -> [false: bb1, otherwise: bb2]; // scope 1 at dead.rs:7:13: 7:37
+        nop;                             // scope 1 at dead.rs:7:13: 7:37
+        switchInt(move _10) -> [3_usize: bb2, otherwise: bb1]; // scope 1 at dead.rs:7:13: 7:37
     }
 
     bb1: {
+        StorageDead(_10);                // scope 1 at dead.rs:7:13: 7:37
         StorageLive(_22);                // scope 1 at /rust/library/std/src/macros.rs:13:23: 13:52
         begin_panic::<&str>(const "internal error: entered unreachable code"); // scope 1 at /rust/library/std/src/macros.rs:13:23: 13:52
                                          // mir::Constant
@@ -75,6 +76,7 @@
     }
 
     bb2: {
+        StorageDead(_10);                // scope 1 at dead.rs:7:13: 7:37
         StorageLive(_13);                // scope 1 at dead.rs:7:14: 7:20
         _13 = &(*_2)[0 of 3];            // scope 1 at dead.rs:7:14: 7:20
         StorageLive(_14);                // scope 1 at dead.rs:7:22: 7:28
