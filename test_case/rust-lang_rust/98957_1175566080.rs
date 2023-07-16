plain

Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 57 tests
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
i.i.i.F..iiii............ii...iiiiiii.iiii...............

---- [run-make] src/test/run-make/const_fn_mir stdout ----

error: make failed
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir  main.rs --emit=mir -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
diff -u --strip-trailing-cr dump.mir "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
--- dump.mir 2022-07-05 21:58:32.324879035 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir/dump.mir 2022-07-05 22:31:51.287855788 +0000
@@ -30,10 +30,10 @@
     let _1: i32;                         // in scope 0 at main.rs:9:5: 9:10
     bb0: {
     bb0: {
-        _1 = foo() -> bb1;               // scope 0 at main.rs:9:5: 9:10
+        _1 = Zst: fn() -> i32 {foo}() -> bb1; // scope 0 at main.rs:9:5: 9:10
                                          // mir::Constant
                                          // + span: main.rs:9:5: 9:8
-                                         // + literal: Const { ty: fn() -> i32 {foo}, val: Value(Scalar(<ZST>)) }
+                                         // + literal: Const { ty: fn() -> i32 {foo}, val: Value(<ZST>) }
 
     bb1: {
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted

make: *** [Makefile:5: all] Error 1



failures:
