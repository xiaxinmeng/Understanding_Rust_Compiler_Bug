plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir  main.rs --emit=mir -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
diff -u --strip-trailing-cr dump.mir "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
--- dump.mir 2022-06-28 21:05:17.362539707 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir/dump.mir 2022-06-28 21:38:33.488050754 +0000
@@ -2,8 +2,14 @@
 // and is subject to change without notice. Knock yourself out.
 fn foo() -> i32 {
     let mut _0: i32;                     // return place in scope 0 at main.rs:4:19: 4:22
+    let mut _1: (i32, bool);             // in scope 0 at main.rs:5:5: 5:10
     bb0: {
     bb0: {
+        _1 = const (11_i32, false);      // scope 0 at main.rs:5:5: 5:10
+        assert(!move (_1.1: bool), "attempt to compute `{} + {}`, which would overflow", const 5_i32, const 6_i32) -> bb1; // scope 0 at main.rs:5:5: 5:10
+
+    bb1: {
+    bb1: {
         _0 = const 11_i32;               // scope 0 at main.rs:5:5: 5:10
         return;                          // scope 0 at main.rs:6:2: 6:2
------------------------------------------
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag


warning: 1 warning emitted

make: *** [Makefile:5: all] Error 1



failures:
