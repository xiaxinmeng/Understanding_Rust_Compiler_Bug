plain
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir  main.rs --emit=mir -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
diff -u --strip-trailing-cr dump.mir "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
--- dump.mir 2022-02-19 16:03:21.169230364 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir/dump.mir 2022-02-19 16:39:55.075664974 +0000
@@ -3,7 +3,7 @@
 fn foo() -> i32 {
     let mut _0: i32;                     // return place in scope 0 at main.rs:4:19: 4:22
-    bb0: {
-    bb0: {
+    bb0: {                               // preds: []
         _0 = const 11_i32;               // scope 0 at main.rs:5:5: 5:10
         return;                          // scope 0 at main.rs:6:2: 6:2
     }
@@ -14,12 +14,12 @@
     let mut _0: i32;                     // return place in scope 0 at main.rs:4:19: 4:22
     let mut _1: (i32, bool);             // in scope 0 at main.rs:5:5: 5:10
-    bb0: {
-    bb0: {
+    bb0: {                               // preds: []
         _1 = CheckedAdd(const 5_i32, const 6_i32); // scope 0 at main.rs:5:5: 5:10
         assert(!move (_1.1: bool), "attempt to compute `{} + {}`, which would overflow", const 5_i32, const 6_i32) -> bb1; // scope 0 at main.rs:5:5: 5:10
 
-    bb1: {
-    bb1: {
+    bb1: {                               // preds: [bb0]
         _0 = move (_1.0: i32);           // scope 0 at main.rs:5:5: 5:10
         return;                          // scope 0 at main.rs:6:2: 6:2
     }
@@ -29,14 +29,14 @@
     let mut _0: ();                      // return place in scope 0 at main.rs:8:11: 8:11
     let _1: i32;                         // in scope 0 at main.rs:9:5: 9:10
-    bb0: {
-    bb0: {
+    bb0: {                               // preds: []
         _1 = foo() -> bb1;               // scope 0 at main.rs:9:5: 9:10
                                          // mir::Constant
                                          // + span: main.rs:9:5: 9:8
                                          // + literal: Const { ty: fn() -> i32 {foo}, val: Value(Scalar(<ZST>)) }
 
-    bb1: {
-    bb1: {
+    bb1: {                               // preds: [bb0]
         return;                          // scope 0 at main.rs:10:2: 10:2
 }

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted

make: *** [Makefile:5: all] Error 1
------------------------------------------



